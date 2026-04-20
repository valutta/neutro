use crate::bot::{guild_settings, has_staff_access, reply_text};
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde_json::json;

// Moderation commands stay fairly direct on purpose.
// Reading a command top-to-bottom should tell you what it does.

fn parse_duration_to_secs(input: &str) -> Option<i64> {
    let value = input.trim().to_lowercase();
    if value.len() < 2 {
        return None;
    }

    let (amount, unit) = value.split_at(value.len() - 1);
    let amount = amount.parse::<i64>().ok()?;
    if amount <= 0 {
        return None;
    }

    let seconds = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 86400,
        _ => return None,
    };

    Some(amount * seconds)
}

async fn check_staff(ctx: &Context<'_>) -> Result<(), Error> {
    if has_staff_access(ctx).await {
        return Ok(());
    }

    reply_text(ctx.clone(), crate::i18n::t(ctx, "ERR_NO_STAFF").await).await
}

async fn current_guild(ctx: &Context<'_>) -> Result<serenity::GuildId, Error> {
    if let Some(guild_id) = ctx.guild_id() {
        return Ok(guild_id);
    }

    Err(crate::i18n::t(ctx, "MOD_MUST_BE_GUILD").await.into())
}

fn fill_text(template: String, replacements: &[(&str, &str)]) -> String {
    let mut text = template;
    for (needle, value) in replacements {
        text = text.replace(needle, value);
    }
    text
}

#[poise::command(slash_command, prefix_command)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
    #[description = "Reason for kicking"] reason: Option<String>,
) -> Result<(), Error> {
    check_staff(&ctx).await?;
    let guild_id = current_guild(&ctx).await?;
    let reason = reason.unwrap_or_else(|| "No reason provided".to_string());

    if let Err(error) = guild_id
        .kick_with_reason(ctx.http(), user.id, &reason)
        .await
    {
        let text = fill_text(
            crate::i18n::t(&ctx, "KICK_FAIL").await,
            &[("{user}", &user.name), ("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let text = fill_text(
        crate::i18n::t(&ctx, "KICK_SUCCESS").await,
        &[("{user}", &user.name), ("{reason}", &reason)],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason for banning"] reason: Option<String>,
) -> Result<(), Error> {
    check_staff(&ctx).await?;
    let guild_id = current_guild(&ctx).await?;
    let reason = reason.unwrap_or_else(|| "No reason provided".to_string());

    if let Err(error) = guild_id
        .ban_with_reason(ctx.http(), user.id, 0, &reason)
        .await
    {
        let text = fill_text(
            crate::i18n::t(&ctx, "BAN_FAIL").await,
            &[("{user}", &user.name), ("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let text = fill_text(
        crate::i18n::t(&ctx, "BAN_SUCCESS").await,
        &[("{user}", &user.name), ("{reason}", &reason)],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::User,
    #[description = "Duration in minutes"] minutes: i64,
    #[description = "Reason for mute"] reason: Option<String>,
) -> Result<(), Error> {
    check_staff(&ctx).await?;
    let guild_id = current_guild(&ctx).await?;

    let timeout_until = serenity::Timestamp::from_unix_timestamp(
        serenity::Timestamp::now().unix_timestamp() + minutes * 60,
    )?;
    let edit = serenity::EditMember::new().disable_communication_until(timeout_until.to_string());

    if let Err(error) = guild_id.edit_member(ctx.http(), user.id, edit).await {
        let text = fill_text(
            crate::i18n::t(&ctx, "MUTE_FAIL").await,
            &[("{user}", &user.name), ("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let reason = reason.unwrap_or_else(|| "No reason provided".to_string());
    let duration = format!("{minutes}m");
    let text = fill_text(
        crate::i18n::t(&ctx, "MUTE_SUCCESS").await,
        &[
            ("{user}", &user.name),
            ("{duration}", &duration),
            ("{reason}", &reason),
        ],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn tempmute(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::User,
    #[description = "Duration: 30m / 2h / 1d"] duration: String,
    #[description = "Reason for mute"] reason: Option<String>,
) -> Result<(), Error> {
    check_staff(&ctx).await?;
    let guild_id = current_guild(&ctx).await?;

    let Some(seconds) = parse_duration_to_secs(&duration) else {
        return reply_text(ctx, crate::i18n::t(&ctx, "NO_DURATION").await).await;
    };

    let timeout_until = serenity::Timestamp::from_unix_timestamp(
        serenity::Timestamp::now().unix_timestamp() + seconds,
    )?;
    let edit = serenity::EditMember::new().disable_communication_until(timeout_until.to_string());

    if let Err(error) = guild_id.edit_member(ctx.http(), user.id, edit).await {
        let text = fill_text(
            crate::i18n::t(&ctx, "MUTE_FAIL").await,
            &[("{user}", &user.name), ("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let reason = reason.unwrap_or_else(|| "No reason provided".to_string());
    let text = fill_text(
        crate::i18n::t(&ctx, "MUTE_SUCCESS").await,
        &[
            ("{user}", &user.name),
            ("{duration}", &duration),
            ("{reason}", &reason),
        ],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "User to unmute"] user: serenity::User,
) -> Result<(), Error> {
    check_staff(&ctx).await?;
    let guild_id = current_guild(&ctx).await?;
    let edit = serenity::EditMember::new().enable_communication();

    if let Err(error) = guild_id.edit_member(ctx.http(), user.id, edit).await {
        let text = fill_text(
            crate::i18n::t(&ctx, "UNMUTE_FAIL").await,
            &[("{user}", &user.name), ("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let text = fill_text(
        crate::i18n::t(&ctx, "UNMUTE_SUCCESS").await,
        &[("{user}", &user.name)],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn clear(
    ctx: Context<'_>,
    #[description = "Number of messages to delete"] amount: u64,
) -> Result<(), Error> {
    check_staff(&ctx).await?;

    let channel_id = ctx.channel_id();
    let messages = channel_id
        .messages(ctx.http(), serenity::GetMessages::new().limit(amount as u8))
        .await?;
    let ids = messages
        .into_iter()
        .map(|message| message.id)
        .collect::<Vec<_>>();

    if let Err(error) = channel_id.delete_messages(ctx.http(), ids).await {
        let text = fill_text(
            crate::i18n::t(&ctx, "MOD_CLEAR_FAIL").await,
            &[("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let amount_text = amount.to_string();
    let text = fill_text(
        crate::i18n::t(&ctx, "CLEAR_SUCCESS").await,
        &[("{amount}", &amount_text)],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn role(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::Member,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    check_staff(&ctx).await?;

    if user.roles.contains(&role.id) {
        if let Err(error) = user.remove_role(ctx.http(), role.id).await {
            let text = fill_text(
                crate::i18n::t(&ctx, "MOD_ROLE_REMOVE_FAIL").await,
                &[("{err}", &error.to_string())],
            );
            return reply_text(ctx, text).await;
        }

        let text = fill_text(
            crate::i18n::t(&ctx, "MOD_ROLE_REMOVE_SUCCESS").await,
            &[("{role}", &role.name), ("{user}", &user.user.name)],
        );
        return reply_text(ctx, text).await;
    }

    if let Err(error) = user.add_role(ctx.http(), role.id).await {
        let text = fill_text(
            crate::i18n::t(&ctx, "MOD_ROLE_ADD_FAIL").await,
            &[("{err}", &error.to_string())],
        );
        return reply_text(ctx, text).await;
    }

    let text = fill_text(
        crate::i18n::t(&ctx, "MOD_ROLE_ADD_SUCCESS").await,
        &[("{role}", &role.name), ("{user}", &user.user.name)],
    );
    reply_text(ctx, text).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn announce(
    ctx: Context<'_>,
    #[description = "Channel"] channel: serenity::GuildChannel,
    #[description = "Message to announce"] text: String,
) -> Result<(), Error> {
    check_staff(&ctx).await?;

    let payload = json!({
        "flags": 1 << 15,
        "components": [
            {
                "type": 17,
                "components": [{ "type": 10, "content": text }]
            }
        ]
    });

    if let Err(error) = ctx.http().send_message(channel.id, vec![], &payload).await {
        return reply_text(ctx, format!("Failed to announce: {error}")).await;
    }

    reply_text(ctx, crate::i18n::t(&ctx, "MOD_ANNOUNCE_SENT").await).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn eannounce(
    ctx: Context<'_>,
    #[description = "Channel (optional, current by default)"] channel: Option<
        serenity::GuildChannel,
    >,
    #[description = "Message"] text: String,
) -> Result<(), Error> {
    check_staff(&ctx).await?;

    let target_channel = channel
        .map(|channel| channel.id)
        .unwrap_or_else(|| ctx.channel_id());
    let payload = json!({
        "content": "@everyone",
        "flags": 1 << 15,
        "components": [
            {
                "type": 17,
                "components": [{ "type": 10, "content": text }]
            }
        ]
    });

    if let Err(error) = ctx
        .http()
        .send_message(target_channel, vec![], &payload)
        .await
    {
        return reply_text(ctx, format!("Failed to announce: {error}")).await;
    }

    reply_text(ctx, crate::i18n::t(&ctx, "EANNOUNCE_SUCCESS").await).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn stream(
    ctx: Context<'_>,
    #[description = "Stream URL"] link: String,
) -> Result<(), Error> {
    check_staff(&ctx).await?;

    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let settings = guild_settings(ctx.data(), guild_id).await;
    let Some(role_id) = settings.stream_ping_role_id else {
        return reply_text(
            ctx,
            "No stream ping role is configured. Set one with `!settings stream_role @Role`.",
        )
        .await;
    };

    let message = crate::i18n::t(&ctx, "MOD_STREAM_TEXT")
        .await
        .replace("{role_id}", &role_id.to_string())
        .replace("{user}", &ctx.author().name)
        .replace("{link}", &link);

    let payload = json!({
        "flags": 1 << 15,
        "components": [
            {
                "type": 17,
                "components": [{ "type": 10, "content": message }]
            }
        ]
    });

    if let Err(error) = ctx
        .http()
        .send_message(ctx.channel_id(), vec![], &payload)
        .await
    {
        return reply_text(ctx, format!("Failed to send stream announcement: {error}")).await;
    }

    Ok(())
}
