use crate::{Context, Error, v2_components::мяу_v2_посылка_90__};
use poise::serenity_prelude::{self as serenity, Color};
use serde_json::json;

macro_rules! мяф {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
}

/// Simple ping
#[poise::command(prefix_command, slash_command, rename = "ping")]
pub async fn мяу_ping_117__(ctx: Context<'_>) -> Result<(), Error> {
    мяф!(мяу_shards <- ctx.framework().shard_manager());
    мяф!(мяу_бегуны <- мяу_shards.runners.lock().await);
    мяф!(мяу_ping <- мяу_бегуны
        .values()
        .next()
        .and_then(|r| r.latency)
        .map(|d| d.as_millis())
        .unwrap_or(0));
    
    мяу_v2_посылка_90__(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "PING").await.replace("{ms}", &мяу_ping.to_string())
                }
            ]
        }
    ])).await?;
    
    Ok(())
}

/// Get user ID
#[poise::command(prefix_command, slash_command, rename = "id")]
pub async fn мяу_id_118__(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    мяф!(мяу_target <- user.as_ref().unwrap_or_else(|| ctx.author()));
    мяу_v2_посылка_90__(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_USER_ID").await.replace("{id}", &мяу_target.id.to_string())
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Extract role ID
#[poise::command(prefix_command, slash_command, rename = "rid")]
pub async fn мяу_rid_119__(
    ctx: Context<'_>,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_ROLE_ID").await.replace("{id}", &role.id.to_string())
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Show avatar URL
#[poise::command(prefix_command, slash_command, rename = "avatar")]
pub async fn мяу_avatar_120__(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    мяф!(мяу_target <- user.as_ref().unwrap_or_else(|| ctx.author()));
    мяф!(мяу_url <- мяу_target.face());
    мяу_v2_посылка_90__(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_AVATAR").await.replace("{user}", &мяу_target.name)
                },
                {
                    "type": 12, // MEDIA_GALLERY
                    "items": [
                        {
                            "media": {
                                "url": мяу_url
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
#[poise::command(prefix_command, slash_command, rename = "profile")]
pub async fn мяу_profile_121__(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    
    let mut joined_info = String::new();
    let mut roles_info = String::new();
    
    if let Some(guild_id) = ctx.guild_id() {
        if let Ok(member) = guild_id.member(ctx.http(), target.id).await {
            if let Some(joined) = member.joined_at {
                joined_info = crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_PROFILE_JOINED").await.replace("{joined}", &joined.to_string());
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
                    roles_info = crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_PROFILE_ROLES").await.replace("{roles}", &r_list.join(", "));
                } else {
                    roles_info = crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_PROFILE_ROLES_HIDDEN").await.replace("{count}", &member.roles.len().to_string());
                }
            }
        }
    }

    let registered = target.created_at().to_string();
    let profile_text = crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_PROFILE")
        .await
        .replace("{user}", target.global_name.as_deref().unwrap_or(&target.name))
        .replace("{id}", &target.id.to_string())
        .replace("{registered}", &registered)
        .replace("{joined}", &joined_info)
        .replace("{roles}", &roles_info);

    мяу_v2_посылка_90__(ctx, json!([
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
    ])).await?;
    
    Ok(())
}

/// General help
#[poise::command(prefix_command, slash_command, rename = "help")]
pub async fn мяу_help_122__(ctx: Context<'_>) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([
        {
            "type": 17, // CONTAINER
            "components": [
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_HELP_TITLE").await
                },
                {
                    "type": 14, // SEPARATOR
                    "divider": true,
                    "spacing": 1
                },
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_HELP_BASIC").await
                },
                {
                    "type": 10, // TEXT_DISPLAY
                    "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_HELP_MOD").await
                }
            ]
        }
    ])).await?;
    Ok(())
}

/// Send quote embed (optionally with hex color, e.g. #2B2D31)
#[poise::command(prefix_command, slash_command, rename = "aquote")]
pub async fn мяу_aquote_123__(
    ctx: Context<'_>,
    #[description = "Text to quote (optionally starts with #RRGGBB)"] text: String,
) -> Result<(), Error> {
    let raw = text.trim();
    if raw.is_empty() {
        мяу_v2_посылка_90__(ctx, json!([{"type": 17, "components": [{"type": 10, "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_AQUOTE_USAGE").await}]}])).await?;
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
        мяу_v2_посылка_90__(ctx, json!([{"type": 17, "components": [{"type": 10, "content": crate::i18n::мяу_скажи_91__(&ctx, "UTILITY_AQUOTE_AFTER_COLOR").await}]}])).await?;
        return Ok(());
    }
    
    мяу_v2_посылка_90__(ctx, json!([
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
    ])).await?;
    
    Ok(())
}
