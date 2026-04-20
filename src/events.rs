use crate::{
    Data, Error,
    ai::{VertexClient, load_prompt_images, strip_bot_mention},
    bot::guild_settings,
    state::{MediaRequest, StickyMessage},
};
use poise::serenity_prelude as serenity;
use serde_json::json;
use serenity::{Color, CreateEmbed};
use std::path::{Path, PathBuf};
use tokio::fs;

fn extract_urls(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .filter(|w| w.starts_with("http://") || w.starts_with("https://"))
        .map(|s| s.to_string())
        .collect()
}

fn build_media_gallery_items(urls: impl IntoIterator<Item = String>) -> Vec<serde_json::Value> {
    urls.into_iter()
        .take(10)
        .map(|url| json!({ "media": { "url": url } }))
        .collect()
}

fn is_direct_media_url(url: &str) -> bool {
    let lower = url.split('?').next().unwrap_or(url).to_ascii_lowercase();
    [
        ".png", ".jpg", ".jpeg", ".gif", ".webp", ".mp4", ".webm", ".mov", ".avif",
    ]
    .iter()
    .any(|ext| lower.ends_with(ext))
}

fn extract_meta_content(body: &str, marker: &str) -> Option<String> {
    let start = body.find(marker)? + marker.len();
    let tail = &body[start..];
    let end = tail.find('"').or_else(|| tail.find('\''))?;
    Some(tail[..end].to_string())
}

fn extract_tenor_media_url(body: &str) -> Option<String> {
    let normalized = body.replace("\\/", "/");
    for marker in [
        "property=\"og:video:secure_url\" content=\"",
        "property=\"og:video\" content=\"",
        "property=\"og:image\" content=\"",
        "property='og:video:secure_url' content='",
        "property='og:video' content='",
        "property='og:image' content='",
    ] {
        if let Some(url) = extract_meta_content(&normalized, marker) {
            if is_direct_media_url(&url) {
                return Some(url);
            }
        }
    }

    let needle = "https://media.tenor.com/";
    if let Some(start) = normalized.find(needle) {
        let tail = &normalized[start..];
        let end = tail
            .find('"')
            .or_else(|| tail.find('\''))
            .or_else(|| tail.find('<'))
            .unwrap_or(tail.len());
        let candidate = tail[..end].to_string();
        if is_direct_media_url(&candidate) {
            return Some(candidate);
        }
    }

    None
}

async fn resolve_external_media_url(url: &str) -> Option<String> {
    if is_direct_media_url(url) {
        return Some(url.to_string());
    }

    if !(url.contains("tenor.com/") || url.contains("tenor.com/view/")) {
        return None;
    }

    let response = reqwest::get(url).await.ok()?;
    let body = response.text().await.ok()?;
    extract_tenor_media_url(&body)
}

async fn resolve_media_urls(urls: &[String]) -> Vec<String> {
    let mut resolved = Vec::new();
    for url in urls {
        if let Some(media_url) = resolve_external_media_url(url).await {
            resolved.push(media_url);
        }
    }
    resolved
}

fn build_attachment_media_items(paths: &[String]) -> Vec<serde_json::Value> {
    paths.iter()
        .filter_map(|path| {
            Path::new(path)
                .file_name()
                .map(|name| json!({ "media": { "url": format!("attachment://{}", name.to_string_lossy()) } }))
        })
        .collect()
}

async fn load_create_attachments(paths: &[String]) -> Vec<serenity::CreateAttachment> {
    let mut files = Vec::new();
    for path in paths {
        if let Ok(file) = serenity::CreateAttachment::path(path).await {
            files.push(file);
        }
    }
    files
}

async fn download_request_attachments(
    ctx: &serenity::Context,
    data: &Data,
    message_id: serenity::MessageId,
    attachments: &[serenity::Attachment],
) -> Vec<String> {
    if attachments.is_empty() {
        return Vec::new();
    }

    let request_dir = data
        .data_dir
        .join("request_media")
        .join(message_id.to_string());
    if fs::create_dir_all(&request_dir).await.is_err() {
        return Vec::new();
    }

    let mut stored = Vec::new();
    for (idx, attachment) in attachments.iter().enumerate() {
        let Ok(file) = serenity::CreateAttachment::url(&ctx.http, &attachment.url).await else {
            continue;
        };

        let filename = format!("{}_{}", idx, file.filename);
        let path = request_dir.join(filename);
        if fs::write(&path, &file.data).await.is_ok() {
            stored.push(path.to_string_lossy().to_string());
        }
    }

    stored
}

async fn cleanup_stored_request_files(paths: &[String]) {
    for path in paths {
        let _ = fs::remove_file(path).await;
    }

    if let Some(parent) = paths
        .first()
        .and_then(|path| PathBuf::from(path).parent().map(|p| p.to_path_buf()))
    {
        let _ = fs::remove_dir(parent).await;
    }
}

async fn send_log(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: Option<serenity::GuildId>,
    title: &str,
    description: String,
    color: Color,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
    let Some(channel_id) = guild_settings(data, guild_id).await.log_channel_id else {
        return;
    };

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .timestamp(serenity::Timestamp::now());
    let _ = serenity::ChannelId::new(channel_id)
        .send_message(ctx, serenity::CreateMessage::new().embed(embed))
        .await;
}

async fn fetch_latest_audit_executor(
    ctx: &serenity::Context,
    guild_id: serenity::GuildId,
    action: serenity::audit_log::Action,
) -> Option<(serenity::UserId, Option<u64>)> {
    let logs = guild_id
        .audit_logs(&ctx.http, Some(action), None, None, Some(1))
        .await
        .ok()?;
    let entry = logs.entries.first()?;
    Some((entry.user_id, entry.target_id.map(|t| t.get())))
}

pub async fn handle_event(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    // This is the main event router for behavior that doesn't fit neatly into a command.
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let settings = guild_settings(data, new_member.guild_id).await;
            let fallback = std::env::var("AUTO_JOIN_ROLE_ID")
                .ok()
                .and_then(|value| value.parse::<u64>().ok());
            if let Some(role_id) = settings.auto_role_id.or(fallback) {
                let _ = new_member
                    .add_role(ctx, serenity::RoleId::new(role_id))
                    .await;
            }
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            handle_reaction(ctx, data, add_reaction, true).await?;
        }
        serenity::FullEvent::ReactionRemove { removed_reaction } => {
            handle_reaction(ctx, data, removed_reaction, false).await?;
        }
        serenity::FullEvent::Message { new_message } => {
            if !new_message.author.bot {
                let terminal_channel_id =
                    crate::bot::terminal_channel_id(data, new_message.guild_id).await;
                if new_message.content.starts_with('!')
                    || new_message.content.starts_with('$')
                    || terminal_channel_id
                        .map(|id| new_message.channel_id == serenity::ChannelId::new(id))
                        .unwrap_or(false)
                {
                    send_log(
                        ctx,
                        data,
                        new_message.guild_id,
                        "⚙️ Command Executed",
                        format!(
                            "**User:** {} (<@{}>)\n**Channel:** <#{}>\n**Command:** `{}`",
                            new_message.author.tag(),
                            new_message.author.id,
                            new_message.channel_id,
                            new_message.content
                        ),
                        Color::new(0xaa44ff),
                    )
                    .await;
                }

                if handle_media_request_queue(ctx, data, new_message).await? {
                    return Ok(());
                }
                if crate::commands::terminal::handle_terminal_channel_message(
                    ctx,
                    data,
                    new_message,
                )
                .await?
                {
                    return Ok(());
                }
                if handle_sticky_setup(ctx, data, new_message).await? {
                    return Ok(());
                }
                handle_ai_mention(ctx, data, new_message).await?;
                handle_sticky_repost(ctx, data, new_message).await?;
            }
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let serenity::Interaction::Component(component) = interaction {
                if crate::commands::terminal::handle_terminal_component_interaction(
                    ctx, data, component,
                )
                .await?
                {
                    return Ok(());
                }
                handle_component_interaction(ctx, data, component, _framework).await?;
            }
        }
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            let deleted_by = if let Some(gid) = guild_id {
                fetch_latest_audit_executor(
                    ctx,
                    *gid,
                    serenity::audit_log::Action::Message(
                        serenity::audit_log::MessageAction::Delete,
                    ),
                )
                .await
                .map(|(u, _)| format!("<@{}>", u))
                .unwrap_or_else(|| "Unknown/Self".to_string())
            } else {
                "Unknown/Self".to_string()
            };
            send_log(
                ctx,
                data,
                *guild_id,
                "🗑️ Message Deleted",
                format!(
                    "**Channel:** <#{}>\n**Message ID:** `{}`\n**Deleted By:** {}",
                    channel_id, deleted_message_id, deleted_by
                ),
                Color::new(0xff4444),
            )
            .await;
        }
        serenity::FullEvent::GuildMemberUpdate {
            old_if_available,
            new,
            event: _,
        } => {
            if let Some(new_member) = new {
                let old_roles = old_if_available
                    .as_ref()
                    .map(|m| m.roles.clone())
                    .unwrap_or_default();
                if old_roles != new_member.roles {
                    let added: Vec<_> = new_member
                        .roles
                        .iter()
                        .filter(|r| !old_roles.contains(r))
                        .map(|r| format!("<@&{}>", r))
                        .collect();
                    let removed: Vec<_> = old_roles
                        .iter()
                        .filter(|r| !new_member.roles.contains(r))
                        .map(|r| format!("<@&{}>", r))
                        .collect();
                    let mut changes = Vec::new();
                    if !added.is_empty() {
                        changes.push(format!("**Added:** {}", added.join(", ")));
                    }
                    if !removed.is_empty() {
                        changes.push(format!("**Removed:** {}", removed.join(", ")));
                    }
                    if !changes.is_empty() {
                        let updated_by = fetch_latest_audit_executor(
                            ctx,
                            new_member.guild_id,
                            serenity::audit_log::Action::Member(
                                serenity::audit_log::MemberAction::RoleUpdate,
                            ),
                        )
                        .await
                        .and_then(|(u, target)| {
                            let is_target = target
                                .map(|t| t.to_string() == new_member.user.id.to_string())
                                .unwrap_or(false);
                            if is_target {
                                Some(format!("<@{}>", u))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| "Unknown/Self".to_string());
                        send_log(
                            ctx,
                            data,
                            Some(new_member.guild_id),
                            "🛡️ Roles Updated",
                            format!(
                                "**User:** <@{}>\n**Updated By:** {}\n{}",
                                new_member.user.id,
                                updated_by,
                                changes.join("\n")
                            ),
                            Color::new(0x44aaff),
                        )
                        .await;
                    }
                }
            }
        }
        serenity::FullEvent::UserUpdate { old_data, new } => {
            let _ = (old_data, new);
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {
            handle_voice_state_event(ctx, data, old, new).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_ai_mention(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    // Mention-based AI replies are handled here so users don't need a prefix command.
    let Some(guild_id) = message.guild_id else {
        return Ok(());
    };

    let bot_user_id = ctx.cache.current_user().id;
    let is_mention = message.mentions.iter().any(|user| user.id == bot_user_id);
    if !is_mention {
        return Ok(());
    }

    let settings = guild_settings(data, guild_id).await;
    if !settings.ai_enabled {
        return Ok(());
    }

    if let Some(channel_id) = settings.ai_channel_id {
        if message.channel_id != serenity::ChannelId::new(channel_id) {
            let restricted_message = serenity::CreateMessage::new()
                .content(format!(
                    "AI replies are restricted to <#{channel_id}> on this server."
                ))
                .allowed_mentions(serenity::CreateAllowedMentions::new());
            let _ = message
                .channel_id
                .send_message(ctx, restricted_message)
                .await;
            return Ok(());
        }
    }

    let current_text = strip_bot_mention(&message.content, bot_user_id.get());
    if current_text.is_empty() {
        let reply = serenity::CreateMessage::new()
            .content("Ask a question in the same message when you tag me.")
            .allowed_mentions(serenity::CreateAllowedMentions::new())
            .reference_message(message);
        let _ = message.channel_id.send_message(ctx, reply).await;
        return Ok(());
    }

    let prompt = build_ai_prompt(ctx, message, &current_text).await;
    let images = load_prompt_images(&message.attachments).await;
    let client = VertexClient::from_env()?;
    let response = match client
        .generate_text(&settings.ai_model, &prompt, &images)
        .await
    {
        Ok(response) => response,
        Err(error) => {
            let reply = serenity::CreateMessage::new()
                .content(format!("Vertex AI request failed: {error}"))
                .allowed_mentions(serenity::CreateAllowedMentions::new())
                .reference_message(message);
            let _ = message.channel_id.send_message(ctx, reply).await;
            return Ok(());
        }
    };

    let reply = serenity::CreateMessage::new()
        .content(response.text)
        .allowed_mentions(serenity::CreateAllowedMentions::new())
        .reference_message(message);
    let _ = message.channel_id.send_message(ctx, reply).await;
    Ok(())
}

async fn build_ai_prompt(
    ctx: &serenity::Context,
    message: &serenity::Message,
    current_text: &str,
) -> String {
    let mut lines = Vec::new();

    if let Ok(mut history) = message
        .channel_id
        .messages(
            ctx,
            serenity::GetMessages::new().before(message.id).limit(20),
        )
        .await
    {
        history.reverse();

        for entry in history {
            if entry.content.trim().is_empty() && entry.attachments.is_empty() {
                continue;
            }

            let speaker = display_name(&entry.author);
            let content = entry.content.trim();

            if content.is_empty() {
                lines.push(format!("{speaker}: [attachment only]"));
            } else {
                lines.push(format!("{speaker}: {content}"));
            }
        }
    }

    let current_speaker = display_name(&message.author);
    if message.attachments.is_empty() {
        lines.push(format!("{current_speaker}: {current_text}"));
    } else {
        lines.push(format!(
            "{current_speaker}: {current_text}\n[Attached images: {}]",
            message.attachments.len()
        ));
    }

    format!(
        "Recent channel context, oldest first:\n{}\n\nReply to the latest user message.",
        lines.join("\n")
    )
}

fn display_name(user: &serenity::User) -> String {
    user.global_name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or(&user.name)
        .to_string()
}

async fn handle_component_interaction(
    ctx: &serenity::Context,
    data: &Data,
    component: &serenity::ComponentInteraction,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    let custom_id = &component.data.custom_id;
    if !custom_id.starts_with("media_") {
        return Ok(());
    }

    let msg_id = component.message.id.to_string();
    let req = data.media_requests.read().await.get(&msg_id).cloned();
    let Some(req) = req else {
        let _ = component
            .create_response(
                ctx,
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Request not found in state.")
                        .ephemeral(true),
                ),
            )
            .await;
        return Ok(());
    };

    if custom_id.starts_with("media_approve_") {
        let original_ch = req
            .original_channel_id
            .parse::<u64>()
            .ok()
            .map(serenity::ChannelId::new);
        if let Some(ch) = original_ch {
            let mut media_items = build_attachment_media_items(&req.stored_files);
            media_items.extend(build_media_gallery_items(req.content_urls.iter().cloned()));
            media_items.extend(build_media_gallery_items(
                req.attachment_urls.iter().cloned(),
            ));
            media_items.truncate(10);
            let files = load_create_attachments(&req.stored_files).await;

            let mut display_text = format!(
                "Approved by <@{}>\nOriginal sender <@{}>",
                component.user.id, req.original_user_id
            );
            if let Some(txt) = &req.original_text {
                if !txt.is_empty() {
                    display_text.push_str(&format!("\n\n{}", txt));
                }
            }

            let payload = if media_items.is_empty() {
                json!({
                    "flags": 1 << 15,
                    "components": [
                        {
                            "type": 17, // CONTAINER
                            "components": [
                                {
                                    "type": 10, // TEXT_DISPLAY
                                    "content": display_text
                                }
                            ]
                        }
                    ]
                })
            } else {
                json!({
                    "flags": 1 << 15,
                    "components": [
                        {
                            "type": 17, // CONTAINER
                            "components": [
                                {
                                    "type": 10, // TEXT_DISPLAY
                                    "content": display_text
                                },
                                {
                                    "type": 12, // MEDIA_GALLERY
                                    "items": media_items
                                }
                            ]
                        }
                    ]
                })
            };
            let _ = ctx.http.send_message(ch, files, &payload).await;
        }
    } else if custom_id.starts_with("media_block_") {
        if let Ok(ch_num) = req.original_channel_id.parse::<u64>() {
            let ch = serenity::ChannelId::new(ch_num);
            if let Ok(member_num) = req.original_user_id.parse::<u64>() {
                let target_user = serenity::UserId::new(member_num);
                let overwrite = serenity::PermissionOverwrite {
                    allow: serenity::Permissions::empty(),
                    deny: serenity::Permissions::SEND_MESSAGES,
                    kind: serenity::PermissionOverwriteType::Member(target_user),
                };
                let _ = ch.create_permission(ctx, overwrite).await;
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(24 * 60 * 60)).await;
                    let _ = ch
                        .delete_permission(
                            &ctx_clone,
                            serenity::PermissionOverwriteType::Member(target_user),
                        )
                        .await;
                });
            }
        }
    }

    // Cleanup request and message
    cleanup_stored_request_files(&req.stored_files).await;
    data.media_requests.write().await.remove(&msg_id);
    data.save_media_requests().await;
    let _ = component.message.delete(ctx).await;
    let _ = component.defer(ctx).await;

    Ok(())
}

async fn handle_media_request_queue(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<bool, Error> {
    if message.author.bot {
        return Ok(false);
    }

    // Request queue is enabled only for channels marked with the "request" flag.
    let ch_id = message.channel_id.to_string();
    let has_request_flag = data
        .flags
        .read()
        .await
        .get(&ch_id)
        .map(|list| list.iter().any(|f| f.eq_ignore_ascii_case("request")))
        .unwrap_or(false);
    if !has_request_flag {
        return Ok(false);
    }

    let approval_channel = data.approval_channels.read().await.get(&ch_id).cloned();
    let Some(approval_channel) = approval_channel else {
        return Ok(false);
    };

    let urls_in_content = extract_urls(&message.content);
    let resolved_content_urls = resolve_media_urls(&urls_in_content).await;
    let attach_urls = message
        .attachments
        .iter()
        .map(|a| a.url.clone())
        .collect::<Vec<_>>();
    if urls_in_content.is_empty() && attach_urls.is_empty() {
        return Ok(false);
    }

    let approval_ch = approval_channel.parse::<u64>().unwrap_or(0);
    if approval_ch == 0 {
        return Ok(false);
    }

    let stored_files =
        download_request_attachments(ctx, data, message.id, &message.attachments).await;
    let mut media_items = build_attachment_media_items(&stored_files);
    media_items.extend(build_media_gallery_items(
        resolved_content_urls.iter().cloned(),
    ));
    if stored_files.is_empty() {
        media_items.extend(build_media_gallery_items(attach_urls.iter().cloned()));
    }
    media_items.truncate(10);

    let mut info_text = format!(
        "## Media Approval Request
**From:** <@{}>
**Channel:** <#{}>",
        message.author.id, message.channel_id
    );
    if !message.content.is_empty() {
        info_text.push_str(&format!(
            "

{}",
            message.content
        ));
    }

    let payload = if media_items.is_empty() {
        json!({
            "flags": 1 << 15,
            "components": [
                {
                    "type": 17, // CONTAINER
                    "components": [
                        {
                            "type": 10, // TEXT_DISPLAY
                            "content": info_text
                        }
                    ]
                },
                {
                    "type": 1, // ACTION_ROW
                    "components": [
                        { "type": 2, "style": 3, "label": "Approve", "custom_id": format!("media_approve_{}", message.id) },
                        { "type": 2, "style": 4, "label": "Deny", "custom_id": format!("media_deny_{}", message.id) },
                        { "type": 2, "style": 2, "label": "Block (24h)", "custom_id": format!("media_block_{}", message.id) }
                    ]
                }
            ]
        })
    } else {
        json!({
            "flags": 1 << 15,
            "components": [
                {
                    "type": 17, // CONTAINER
                    "components": [
                        {
                            "type": 10, // TEXT_DISPLAY
                            "content": info_text
                        },
                        {
                            "type": 12, // MEDIA_GALLERY
                            "items": media_items
                        }
                    ]
                },
                {
                    "type": 1, // ACTION_ROW
                    "components": [
                        { "type": 2, "style": 3, "label": "Approve", "custom_id": format!("media_approve_{}", message.id) },
                        { "type": 2, "style": 4, "label": "Deny", "custom_id": format!("media_deny_{}", message.id) },
                        { "type": 2, "style": 2, "label": "Block (24h)", "custom_id": format!("media_block_{}", message.id) }
                    ]
                }
            ]
        })
    };

    let approval_files = load_create_attachments(&stored_files).await;
    let no_stored_files = stored_files.is_empty();
    let forwarded = ctx
        .http
        .send_message(
            serenity::ChannelId::new(approval_ch),
            approval_files,
            &payload,
        )
        .await?;

    data.media_requests.write().await.insert(
        forwarded.id.to_string(),
        MediaRequest {
            original_channel_id: ch_id,
            original_user_id: message.author.id.to_string(),
            stored_files,
            content_urls: resolved_content_urls,
            attachment_urls: if no_stored_files {
                attach_urls
            } else {
                Vec::new()
            },
            original_text: if message.content.is_empty() {
                None
            } else {
                Some(message.content.clone())
            },
        },
    );
    data.save_media_requests().await;
    let _ = message.delete(ctx).await;
    Ok(true)
}

async fn handle_voice_state_event(
    ctx: &serenity::Context,
    data: &Data,
    old: &Option<serenity::VoiceState>,
    new: &serenity::VoiceState,
) -> Result<(), Error> {
    let uid = new.user_id.to_string();
    let old_channel = old.as_ref().and_then(|v| v.channel_id);
    let new_channel = new.channel_id;
    if old_channel.is_none() && new_channel.is_some() {
        send_log(
            ctx,
            data,
            new.guild_id,
            "🎙️ Voice Joined",
            format!(
                "**User:** <@{}>\n**Channel:** <#{}>",
                uid,
                new_channel.unwrap()
            ),
            Color::new(0x44ff44),
        )
        .await;
    } else if old_channel.is_some() && new_channel.is_none() {
        let action_by = if let Some(gid) = new.guild_id {
            fetch_latest_audit_executor(
                ctx,
                gid,
                serenity::audit_log::Action::Member(
                    serenity::audit_log::MemberAction::MemberDisconnect,
                ),
            )
            .await
            .and_then(|(u, target)| {
                let is_target = target.map(|t| t.to_string() == uid).unwrap_or(false);
                if is_target {
                    Some(format!("<@{}>", u))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Self/Unknown".to_string())
        } else {
            "Self/Unknown".to_string()
        };
        send_log(
            ctx,
            data,
            new.guild_id,
            "🎙️ Voice Left / Disconnected",
            format!(
                "**User:** <@{}>\n**Channel:** <#{}>\n**Action By:** {}",
                uid,
                old_channel.unwrap(),
                action_by
            ),
            Color::new(0xffaa44),
        )
        .await;
    } else if old_channel.is_some() && new_channel.is_some() && old_channel != new_channel {
        let moved_by = if let Some(gid) = new.guild_id {
            fetch_latest_audit_executor(
                ctx,
                gid,
                serenity::audit_log::Action::Member(serenity::audit_log::MemberAction::MemberMove),
            )
            .await
            .and_then(|(u, target)| {
                let is_target = target.map(|t| t.to_string() == uid).unwrap_or(false);
                if is_target {
                    Some(format!("<@{}>", u))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Self/Unknown".to_string())
        } else {
            "Self/Unknown".to_string()
        };
        send_log(
            ctx,
            data,
            new.guild_id,
            "🎙️ Voice Moved",
            format!(
                "**User:** <@{}>\n**From:** <#{}>\n**To:** <#{}>\n**Moved By:** {}",
                uid,
                old_channel.unwrap(),
                new_channel.unwrap(),
                moved_by
            ),
            Color::new(0xaaaaff),
        )
        .await;
    }
    Ok(())
}

async fn handle_sticky_setup(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<bool, Error> {
    let uid = message.author.id.to_string();
    let channel_id = message.channel_id.to_string();

    let expected_channel = data.awaiting_sticky.read().await.get(&uid).cloned();
    if expected_channel.as_deref() != Some(channel_id.as_str()) {
        return Ok(false);
    }

    let content = message.content.trim().to_string();
    let image_url = message.attachments.first().map(|a| a.url.clone());

    if content.is_empty() && image_url.is_none() {
        message
            .channel_id
            .say(
                ctx,
                "Пустое сообщение нельзя сделать sticky. Отправь текст и/или картинку.",
            )
            .await?;
        return Ok(true);
    }

    {
        let mut awaiting = data.awaiting_sticky.write().await;
        awaiting.remove(&uid);
    }

    let sticky = StickyMessage {
        content: if content.is_empty() {
            None
        } else {
            Some(content.clone())
        },
        image_url: image_url.clone(),
        last_message_id: None,
    };
    {
        let mut sm = data.sticky_messages.write().await;
        sm.insert(channel_id.clone(), sticky);
    }
    data.save_sticky_messages().await;

    message.channel_id.say(ctx, "✅ **Sticky Message Set!**\nТеперь после каждого нового сообщения sticky будет появляться внизу.").await?;

    let mut create = serenity::CreateMessage::new();
    if !content.is_empty() {
        create = create.content(content);
    }
    if let Some(url) = image_url {
        create = create.embed(CreateEmbed::new().image(url));
    }
    let sent = message.channel_id.send_message(ctx, create).await?;
    {
        let mut sm = data.sticky_messages.write().await;
        if let Some(entry) = sm.get_mut(&channel_id) {
            entry.last_message_id = Some(sent.id.to_string());
        }
    }
    data.save_sticky_messages().await;

    Ok(true)
}

async fn handle_sticky_repost(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    let channel_id = message.channel_id.to_string();
    let sticky = data.sticky_messages.read().await.get(&channel_id).cloned();
    let Some(sticky) = sticky else {
        return Ok(());
    };

    if let Some(last_id) = sticky.last_message_id.as_deref() {
        if let Ok(last_id_num) = last_id.parse::<u64>() {
            let _ = message
                .channel_id
                .delete_message(ctx, serenity::MessageId::new(last_id_num))
                .await;
        }
    }

    let mut create = serenity::CreateMessage::new();
    if let Some(content) = sticky.content.as_deref() {
        if !content.is_empty() {
            create = create.content(content);
        }
    }
    if let Some(url) = sticky.image_url.as_deref() {
        create = create.embed(CreateEmbed::new().image(url));
    }
    let sent = message.channel_id.send_message(ctx, create).await?;

    {
        let mut sm = data.sticky_messages.write().await;
        if let Some(entry) = sm.get_mut(&channel_id) {
            entry.last_message_id = Some(sent.id.to_string());
        }
    }
    data.save_sticky_messages().await;
    Ok(())
}

async fn handle_reaction(
    ctx: &serenity::Context,
    data: &Data,
    reaction: &serenity::Reaction,
    added: bool,
) -> Result<(), Error> {
    if let Some(user_id) = reaction.user_id {
        if let Some(guild_id) = reaction.guild_id {
            let msg_id = reaction.message_id.to_string();
            let emoji_id = match &reaction.emoji {
                serenity::ReactionType::Custom { id, .. } => id.to_string(),
                serenity::ReactionType::Unicode(s) => s.clone(),
                _ => return Ok(()),
            };

            // Media request moderation queue
            if added {
                let req = data.media_requests.read().await.get(&msg_id).cloned();
                if let Some(req) = req {
                    if emoji_id == "✅" {
                        let original_ch = req
                            .original_channel_id
                            .parse::<u64>()
                            .ok()
                            .map(serenity::ChannelId::new);
                        if let Some(ch) = original_ch {
                            let mut content = format!(
                                "Вложение одобрил <@{}>\nОригинальный отправитель <@{}>",
                                user_id, req.original_user_id
                            );
                            if let Some(txt) = req.original_text {
                                if !txt.is_empty() {
                                    content.push_str(&format!("\n{}", txt));
                                }
                            }
                            if !req.content_urls.is_empty() {
                                content.push_str(&format!("\n{}", req.content_urls.join("\n")));
                            }
                            if !req.attachment_urls.is_empty() {
                                content.push_str(&format!("\n{}", req.attachment_urls.join("\n")));
                            }
                            let _ = ch.say(ctx, content).await;
                        }
                        data.media_requests.write().await.remove(&msg_id);
                        data.save_media_requests().await;
                        let _ = reaction
                            .channel_id
                            .delete_message(ctx, reaction.message_id)
                            .await;
                        return Ok(());
                    } else if emoji_id == "1475200940964319414" {
                        if let Ok(ch_num) = req.original_channel_id.parse::<u64>() {
                            let ch = serenity::ChannelId::new(ch_num);
                            if let Ok(member_num) = req.original_user_id.parse::<u64>() {
                                let target_user = serenity::UserId::new(member_num);
                                let overwrite = serenity::PermissionOverwrite {
                                    allow: serenity::Permissions::empty(),
                                    deny: serenity::Permissions::SEND_MESSAGES,
                                    kind: serenity::PermissionOverwriteType::Member(target_user),
                                };
                                let _ = ch.create_permission(ctx, overwrite).await;
                                let ctx_clone = ctx.clone();
                                tokio::spawn(async move {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(
                                        24 * 60 * 60,
                                    ))
                                    .await;
                                    let _ = ch
                                        .delete_permission(
                                            &ctx_clone,
                                            serenity::PermissionOverwriteType::Member(target_user),
                                        )
                                        .await;
                                });
                            }
                        }
                        data.media_requests.write().await.remove(&msg_id);
                        data.save_media_requests().await;
                        let _ = reaction
                            .channel_id
                            .delete_message(ctx, reaction.message_id)
                            .await;
                        return Ok(());
                    } else if emoji_id == "❌"
                        || emoji_id == "🚫"
                        || emoji_id == "1474811226143068303"
                    {
                        data.media_requests.write().await.remove(&msg_id);
                        data.save_media_requests().await;
                        let _ = reaction
                            .channel_id
                            .delete_message(ctx, reaction.message_id)
                            .await;
                        return Ok(());
                    }
                }
            }

            // Starboard
            if added
                && (emoji_id == "⭐"
                    || emoji_id == "star"
                    || emoji_id == "1475196363343003973"
                    || emoji_id == "1475196011549818970")
            {
                let already = data
                    .starboarded
                    .read()
                    .await
                    .get(&msg_id)
                    .copied()
                    .unwrap_or(false);
                if !already {
                    let msg = reaction.channel_id.message(ctx, reaction.message_id).await;
                    if let Ok(msg) = msg {
                        let mut star_count: u64 = 0;
                        for r in msg.reactions.iter() {
                            let eid = match &r.reaction_type {
                                serenity::ReactionType::Custom { id, .. } => id.to_string(),
                                serenity::ReactionType::Unicode(s) => s.clone(),
                                _ => String::new(),
                            };
                            if eid == "⭐"
                                || eid == "star"
                                || eid == "1475196363343003973"
                                || eid == "1475196011549818970"
                            {
                                star_count += r.count;
                            }
                        }
                        let settings = guild_settings(data, guild_id).await;
                        let Some(starboard_channel_id) = settings.starboard_channel_id else {
                            return Ok(());
                        };

                        if star_count >= settings.starboard_threshold {
                            data.starboarded.write().await.insert(msg_id.clone(), true);
                            data.save_starboard().await;
                            let mut content = format!(
                                "⭐ **{}** | <#{}>\n\n**{}**: {}",
                                star_count, msg.channel_id, msg.author.name, msg.content
                            );
                            if !msg.attachments.is_empty() {
                                content.push_str("\n\nAttachments:\n");
                                for a in msg.attachments.iter() {
                                    content.push_str(&format!("{}\n", a.url));
                                }
                            }
                            content.push_str(&format!(
                                "\n[Jump to Message](https://discord.com/channels/{}/{}/{})",
                                guild_id, msg.channel_id, msg.id
                            ));
                            let _ = serenity::ChannelId::new(starboard_channel_id)
                                .send_message(
                                    ctx,
                                    serenity::CreateMessage::new()
                                        .content(content)
                                        .allowed_mentions(serenity::CreateAllowedMentions::new()),
                                )
                                .await;

                            // Economy reward removed
                        }
                    }
                }
            }

            let rr = data.reaction_roles.read().await;
            if let Some(config) = rr.get(&msg_id) {
                // Config format: { "roleId": "...", "emoji": "..." }
                if let (Some(role_id_val), Some(emoji_val)) =
                    (config.get("roleId"), config.get("emoji"))
                {
                    if emoji_val.as_str() == Some(&emoji_id) {
                        if let Some(role_id_str) = role_id_val.as_str() {
                            if let Ok(role_id) = role_id_str.parse::<u64>() {
                                if let Ok(member) = guild_id.member(ctx, user_id).await {
                                    if added {
                                        let _ = member
                                            .add_role(ctx, serenity::RoleId::new(role_id))
                                            .await;
                                    } else {
                                        let _ = member
                                            .remove_role(ctx, serenity::RoleId::new(role_id))
                                            .await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
