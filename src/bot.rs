use crate::state::GuildSettings;
use crate::v2_components::send_v2;
use crate::{Context, Data, Error};
use poise::serenity_prelude as serenity;
use serde_json::json;

// Small helpers that are shared across commands and events.
pub async fn reply_text(ctx: Context<'_>, content: impl Into<String>) -> Result<(), Error> {
    send_v2(
        ctx,
        json!([
            {
                "type": 17,
                "components": [
                    {
                        "type": 10,
                        "content": content.into()
                    }
                ]
            }
        ]),
    )
    .await
}

pub async fn guild_settings(data: &Data, guild_id: serenity::GuildId) -> GuildSettings {
    data.guild_settings
        .read()
        .await
        .get(&guild_id.to_string())
        .cloned()
        .unwrap_or_default()
}

// Most permission checks first allow server admins, then fall back to configured roles.
pub async fn has_staff_access(ctx: &Context<'_>) -> bool {
    let Some(member) = ctx.author_member().await else {
        return false;
    };

    if has_admin_permissions(&member) {
        return true;
    }

    let Some(guild_id) = ctx.guild_id() else {
        return false;
    };

    let settings = guild_settings(ctx.data(), guild_id).await;
    settings
        .staff_role_id
        .map(|role_id| member.roles.contains(&serenity::RoleId::new(role_id)))
        .unwrap_or(false)
}

pub async fn has_private_voice_access(ctx: &Context<'_>) -> bool {
    if has_staff_access(ctx).await {
        return true;
    }

    let Some(member) = ctx.author_member().await else {
        return false;
    };
    let Some(guild_id) = ctx.guild_id() else {
        return false;
    };

    let settings = guild_settings(ctx.data(), guild_id).await;
    settings
        .private_voice_role_id
        .map(|role_id| member.roles.contains(&serenity::RoleId::new(role_id)))
        .unwrap_or(false)
}

pub async fn terminal_channel_id(data: &Data, guild_id: Option<serenity::GuildId>) -> Option<u64> {
    if let Some(guild_id) = guild_id {
        let configured = guild_settings(data, guild_id).await.terminal_channel_id;
        if configured.is_some() {
            return configured;
        }
    }

    std::env::var("TERMINAL_CHANNEL_ID")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
}

pub fn owner_user_id() -> Option<String> {
    std::env::var("DISCORD_OWNER_ID")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| Some("1117969014698811593".to_string()))
}

fn has_admin_permissions(member: &serenity::Member) -> bool {
    member
        .permissions
        .map(|permissions| permissions.administrator() || permissions.manage_guild())
        .unwrap_or(false)
}
