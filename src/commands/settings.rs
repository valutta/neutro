use crate::bot::{guild_settings, has_staff_access, reply_text};
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

// This file is the setup surface for server owners.
// Each subcommand writes one small part of the guild config.
#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "show",
        "language",
        "staff_role",
        "stream_role",
        "private_voice_role",
        "auto_role",
        "log_channel",
        "starboard",
        "terminal_channel",
        "ai"
    )
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    show_settings(ctx).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn show(ctx: Context<'_>) -> Result<(), Error> {
    show_settings(ctx).await
}

async fn show_settings(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, "This command can only be used in a server.").await;
    };

    let current = guild_settings(ctx.data(), guild_id).await;
    let body = format!(
        "## Server Settings\n\
        Language: `{}`\n\
        Staff role: {}\n\
        Stream ping role: {}\n\
        Private voice role: {}\n\
        Auto role: {}\n\
        Log channel: {}\n\
        Starboard channel: {}\n\
        Starboard threshold: `{}`\n\
        Terminal channel: {}\n\
        AI enabled: `{}`\n\
        AI channel: {}\n\
        AI mode: {}",
        current.language,
        role_label(current.staff_role_id),
        role_label(current.stream_ping_role_id),
        role_label(current.private_voice_role_id),
        role_label(current.auto_role_id),
        channel_label(current.log_channel_id),
        channel_label(current.starboard_channel_id),
        current.starboard_threshold,
        channel_label(current.terminal_channel_id),
        current.ai_enabled,
        channel_label(current.ai_channel_id),
        crate::commands::ai::ai_mode_summary(&current.ai_model),
    );

    reply_text(ctx, body).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn language(
    ctx: Context<'_>,
    #[description = "Language code: ru or en"] lang: String,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;

    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let lang = lang.trim().to_lowercase();
    if lang != "ru" && lang != "en" {
        return reply_text(ctx, "Invalid language. Use `ru` or `en`.").await;
    }

    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.language = lang.clone();
    }
    ctx.data().save_guild_settings().await;

    reply_text(ctx, format!("Language updated to `{lang}`.")).await
}

// Role-based settings.
#[poise::command(slash_command, prefix_command)]
pub async fn staff_role(
    ctx: Context<'_>,
    #[description = "Role allowed to use staff commands"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let role_id = role.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.staff_role_id = role_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Staff role updated to {}.", role_label(role_id)),
    )
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn stream_role(
    ctx: Context<'_>,
    #[description = "Role pinged by !stream"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let role_id = role.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.stream_ping_role_id = role_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Stream role updated to {}.", role_label(role_id)),
    )
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn private_voice_role(
    ctx: Context<'_>,
    #[description = "Role granted by !pvoice"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let role_id = role.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.private_voice_role_id = role_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Private voice role updated to {}.", role_label(role_id)),
    )
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn auto_role(
    ctx: Context<'_>,
    #[description = "Role given to new members"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let role_id = role.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.auto_role_id = role_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Auto role updated to {}.", role_label(role_id)),
    )
    .await
}

// Channel-based settings.
#[poise::command(slash_command, prefix_command)]
pub async fn log_channel(
    ctx: Context<'_>,
    #[description = "Channel used for moderation and audit logs"] channel: Option<
        serenity::GuildChannel,
    >,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let channel_id = channel.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.log_channel_id = channel_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Log channel updated to {}.", channel_label(channel_id)),
    )
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn starboard(
    ctx: Context<'_>,
    #[description = "Channel used for starboard posts"] channel: Option<serenity::GuildChannel>,
    #[description = "Minimum star count before a message is reposted"] threshold: Option<u64>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    if threshold == Some(0) {
        return reply_text(ctx, "Starboard threshold must be at least `1`.").await;
    }

    let channel_id = channel.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.starboard_channel_id = channel_id;
        if let Some(threshold) = threshold {
            server.starboard_threshold = threshold;
        }
    }
    ctx.data().save_guild_settings().await;

    let current = guild_settings(ctx.data(), guild_id).await;
    let body = format!(
        "Starboard updated.\nChannel: {}\nThreshold: `{}`",
        channel_label(current.starboard_channel_id),
        current.starboard_threshold
    );
    reply_text(ctx, body).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn terminal_channel(
    ctx: Context<'_>,
    #[description = "Channel used by terminal-style message commands"] channel: Option<
        serenity::GuildChannel,
    >,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let channel_id = channel.as_ref().map(|item| item.id.get());
    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.terminal_channel_id = channel_id;
    }
    ctx.data().save_guild_settings().await;

    reply_text(
        ctx,
        format!("Terminal channel updated to {}.", channel_label(channel_id)),
    )
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn ai(
    ctx: Context<'_>,
    #[description = "Enable or disable AI replies"] enabled: bool,
    #[description = "Optional channel restriction"] channel: Option<serenity::GuildChannel>,
    #[description = "Mode: adaptive, flash, or pro"] model: Option<String>,
) -> Result<(), Error> {
    ensure_staff(&ctx).await?;
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    let model = model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase);
    let channel_id = channel.as_ref().map(|item| item.id.get());

    {
        let mut all_settings = ctx.data().guild_settings.write().await;
        let server = all_settings.entry(guild_id.to_string()).or_default();
        server.ai_enabled = enabled;
        server.ai_channel_id = channel_id;
        if let Some(model) = &model {
            server.ai_model = model.clone();
        }
    }
    ctx.data().save_guild_settings().await;

    let current = guild_settings(ctx.data(), guild_id).await;
    let body = format!(
        "AI settings updated.\nEnabled: `{}`\nChannel: {}\nMode: {}",
        current.ai_enabled,
        channel_label(current.ai_channel_id),
        crate::commands::ai::ai_mode_summary(&current.ai_model),
    );
    reply_text(ctx, body).await
}

async fn ensure_staff(ctx: &Context<'_>) -> Result<(), Error> {
    if has_staff_access(ctx).await {
        Ok(())
    } else {
        reply_text(ctx.clone(), crate::i18n::t(ctx, "ERR_NO_STAFF").await).await
    }
}

fn role_label(role_id: Option<u64>) -> String {
    match role_id {
        Some(role_id) => format!("<@&{role_id}>"),
        None => "`not set`".to_string(),
    }
}

fn channel_label(channel_id: Option<u64>) -> String {
    match channel_id {
        Some(channel_id) => format!("<#{channel_id}>"),
        None => "`not set`".to_string(),
    }
}
