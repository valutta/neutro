use crate::{Context, Error, v2_components::send_v2};
use poise::serenity_prelude::{self as serenity, Color};
use serde_json::json;

/// Simple ping
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let shard_manager = ctx.framework().shard_manager();
    let runners = shard_manager.runners.lock().await;

    let latency = runners
        .values()
        .next()
        .and_then(|r| r.latency)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    send_v2(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::t(&ctx, "PING").await.replace("{ms}", &latency.to_string())
                }
            ]
        }
    ])).await?;

    Ok(())
}

/// Get user ID
#[poise::command(prefix_command, slash_command)]
pub async fn id(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    send_v2(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::t(&ctx, "UTILITY_USER_ID").await.replace("{id}", &target.id.to_string())
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Extract role ID
#[poise::command(prefix_command, slash_command, rename = "rid")]
pub async fn rid(
    ctx: Context<'_>,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    send_v2(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::t(&ctx, "UTILITY_ROLE_ID").await.replace("{id}", &role.id.to_string())
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Show avatar URL
#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let url = target.face();
    send_v2(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::t(&ctx, "UTILITY_AVATAR").await.replace("{user}", &target.name)
                },
                {
                    "type": 12, // MEDIA_GALLERY
                    "items": [
                        {
                            "media": {
                                "url": url
                            }
                        }
                    ]
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Show user info
#[poise::command(prefix_command, slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());

    let mut joined_info = String::new();
    let mut roles_info = String::new();

    if let Some(guild_id) = ctx.guild_id() {
        if let Ok(member) = guild_id.member(ctx.http(), target.id).await {
            if let Some(joined) = member.joined_at {
                joined_info = crate::i18n::t(&ctx, "UTILITY_PROFILE_JOINED")
                    .await
                    .replace("{joined}", &joined.to_string());
            }
            if !member.roles.is_empty() {
                let mut r_list = Vec::new();

                // Try from cache first, then HTTP fallback
                let mut resolved = false;
                if let Some(g) = ctx.cache().guild(guild_id) {
                    for r in &member.roles {
                        if let Some(role) = g.roles.get(r) {
                            r_list.push(format!("@{}", role.name));
                        }
                    }
                    resolved = true;
                }

                if !resolved {
                    if let Ok(guild) = guild_id.to_partial_guild(ctx.http()).await {
                        for r in &member.roles {
                            if let Some(role) = guild.roles.get(r) {
                                r_list.push(format!("@{}", role.name));
                            }
                        }
                    }
                }

                if !r_list.is_empty() {
                    roles_info = crate::i18n::t(&ctx, "UTILITY_PROFILE_ROLES")
                        .await
                        .replace("{roles}", &r_list.join(", "));
                } else {
                    roles_info = crate::i18n::t(&ctx, "UTILITY_PROFILE_ROLES_HIDDEN")
                        .await
                        .replace("{count}", &member.roles.len().to_string());
                }
            }
        }
    }

    let registered = target.created_at().to_string();
    let profile_text = crate::i18n::t(&ctx, "UTILITY_PROFILE")
        .await
        .replace(
            "{user}",
            target.global_name.as_deref().unwrap_or(&target.name),
        )
        .replace("{id}", &target.id.to_string())
        .replace("{registered}", &registered)
        .replace("{joined}", &joined_info)
        .replace("{roles}", &roles_info);

    send_v2(
        ctx,
        json!([
            {
                "type": 17, // CONTAINER
                "components": [
                    {
                        "type": 10, // TEXT_DISPLAY
                        "content": profile_text
                    }
                ],
                "accessory": {
                    "type": 11, // THUMBNAIL
                    "media": {
                        "url": target.face()
                    }
                }
            }
        ]),
    )
    .await?;

    Ok(())
}

/// General help
#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    send_v2(
        ctx,
        json!([
            {
                "type": 17, // CONTAINER
                "components": [
                    {
                        "type": 10, // TEXT_DISPLAY
                        "content": crate::i18n::t(&ctx, "UTILITY_HELP_TITLE").await
                    },
                    {
                        "type": 14, // SEPARATOR
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10, // TEXT_DISPLAY
                        "content": crate::i18n::t(&ctx, "UTILITY_HELP_BASIC").await
                    },
                    {
                        "type": 10, // TEXT_DISPLAY
                        "content": crate::i18n::t(&ctx, "UTILITY_HELP_MOD").await
                    }
                ]
            }
        ]),
    )
    .await?;
    Ok(())
}

/// Send quote embed (optionally with hex color, e.g. #2B2D31)
#[poise::command(prefix_command, slash_command)]
pub async fn aquote(
    ctx: Context<'_>,
    #[description = "Text to quote (optionally starts with #RRGGBB)"] text: String,
) -> Result<(), Error> {
    let raw = text.trim();
    if raw.is_empty() {
        send_v2(ctx, json!([{"type": 17, "components": [{"type": 10, "content": crate::i18n::t(&ctx, "UTILITY_AQUOTE_USAGE").await}]}])).await?;
        return Ok(());
    }
    let mut color = Color::from_rgb(43, 45, 49);
    let mut content = raw.to_string();
    if let Some(first) = raw.split_whitespace().next() {
        let hex = first.trim_start_matches('#');
        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(parsed) = u32::from_str_radix(hex, 16) {
                color = Color::new(parsed);
                content = raw[first.len()..].trim().to_string();
            }
        }
    }
    if content.is_empty() {
        send_v2(ctx, json!([{"type": 17, "components": [{"type": 10, "content": crate::i18n::t(&ctx, "UTILITY_AQUOTE_AFTER_COLOR").await}]}])).await?;
        return Ok(());
    }

    send_v2(
        ctx,
        json!([
            {
                "type": 17, // CONTAINER
                "accent_color": color.0,
                "components": [
                    {
                        "type": 10, // TEXT_DISPLAY
                        "content": content
                    }
                ]
            }
        ]),
    )
    .await?;

    Ok(())
}
