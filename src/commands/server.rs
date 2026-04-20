use crate::bot::{guild_settings, has_private_voice_access, has_staff_access, reply_text};
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

// Server tools are intentionally small and mostly touch one piece of state each.
#[poise::command(prefix_command, slash_command)]
pub async fn sticky(ctx: Context<'_>) -> Result<(), Error> {
    if !has_staff_access(&ctx).await {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_STAFF").await).await;
    }

    let user_id = ctx.author().id.to_string();
    let channel_id = ctx.channel_id().to_string();
    ctx.data()
        .awaiting_sticky
        .write()
        .await
        .insert(user_id, channel_id);

    reply_text(ctx, crate::i18n::t(&ctx, "SERVER_STICKY_SETUP").await).await
}

#[poise::command(prefix_command, slash_command, rename = "dsticky")]
pub async fn dsticky(ctx: Context<'_>) -> Result<(), Error> {
    if !has_staff_access(&ctx).await {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_STAFF").await).await;
    }

    let removed = ctx
        .data()
        .sticky_messages
        .write()
        .await
        .remove(&ctx.channel_id().to_string());
    ctx.data().save_sticky_messages().await;

    if removed.is_some() {
        reply_text(ctx, crate::i18n::t(&ctx, "SERVER_STICKY_REMOVED").await).await
    } else {
        reply_text(ctx, crate::i18n::t(&ctx, "SERVER_STICKY_NOT_FOUND").await).await
    }
}

#[poise::command(prefix_command, slash_command, rename = "ar")]
pub async fn ar(
    ctx: Context<'_>,
    #[description = "Role for new members"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    if !has_staff_access(&ctx).await {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_STAFF").await).await;
    }

    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_GUILD").await).await;
    };

    if let Some(role) = role {
        let role_name = role.name.clone();
        {
            let mut all_settings = ctx.data().guild_settings.write().await;
            let server = all_settings.entry(guild_id.to_string()).or_default();
            server.auto_role_id = Some(role.id.get());
        }
        ctx.data().save_guild_settings().await;
        return reply_text(
            ctx,
            crate::i18n::t(&ctx, "SERVER_AUTOROLE_SET")
                .await
                .replace("{role}", &role_name),
        )
        .await;
    }

    let settings = guild_settings(ctx.data(), guild_id).await;
    match settings.auto_role_id {
        Some(role_id) => {
            reply_text(
                ctx,
                crate::i18n::t(&ctx, "SERVER_AUTOROLE_CURRENT")
                    .await
                    .replace("{role_id}", &role_id.to_string()),
            )
            .await
        }
        None => reply_text(ctx, crate::i18n::t(&ctx, "SERVER_AUTOROLE_EMPTY").await).await,
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn pvoice(
    ctx: Context<'_>,
    #[description = "Target member"] target: serenity::Member,
) -> Result<(), Error> {
    if !has_private_voice_access(&ctx).await {
        return reply_text(ctx, crate::i18n::t(&ctx, "ERR_NO_STAFF").await).await;
    }

    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, crate::i18n::t(&ctx, "SERVER_ONLY_GUILD").await).await;
    };

    let settings = guild_settings(ctx.data(), guild_id).await;
    let Some(role_id) = settings.private_voice_role_id else {
        return reply_text(
            ctx,
            "No private voice role is configured. Set one with `!settings private_voice_role @Role`.",
        )
        .await;
    };

    match target
        .add_role(ctx.http(), serenity::RoleId::new(role_id))
        .await
    {
        Ok(_) => {
            reply_text(
                ctx,
                crate::i18n::t(&ctx, "SERVER_PVOICE_ASSIGNED")
                    .await
                    .replace("{user_id}", &target.user.id.to_string()),
            )
            .await
        }
        Err(error) => {
            reply_text(
                ctx,
                crate::i18n::t(&ctx, "SERVER_ADD_ROLE_FAIL")
                    .await
                    .replace("{err}", &error.to_string()),
            )
            .await
        }
    }
}
