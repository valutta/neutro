use crate::{Context, Error, v2_components::мяу_v2_посылка_90__};
use poise::serenity_prelude as serenity;
use serde_json::json;

macro_rules! мяф {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
}

async fn мяу_стафф_проверка_95__(ctx: &Context<'_>) -> bool {
    let Some(guild_id) = ctx.guild_id() else {
        return false;
    };
    let member = match ctx.author_member().await {
        Some(m) => m,
        None => return false,
    };
    if let Ok(perms) = member.permissions(ctx.cache()) {
        if perms.administrator() {
            return true;
        }
    }
    if let Some(role_id) = ctx.data().мяу_роль_стаффа_39__(Some(guild_id)).await {
        return member.roles.contains(&role_id);
    }
    false
}

async fn мяу_сервер_ответ_98__(ctx: Context<'_>, text: String) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([{
        "type": 17, 
        "components": [{ "type": 10, "content": text }]
    }])).await?;
    Ok(())
}

async fn мяу_сервер_ошибка_99__(ctx: Context<'_>, text: String) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([{
        "type": 17, 
        "components": [{ "type": 10, "content": text }]
    }])).await?;
    Ok(())
}

/// Enable sticky setup in the current channel
#[poise::command(prefix_command, slash_command, rename = "sticky")]
pub async fn мяу_липкость_100__(ctx: Context<'_>) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }

    мяф!(мяу_uid <- ctx.author().id.to_string());
    мяф!(мяу_ch <- ctx.channel_id().to_string());
    ctx.data().awaiting_sticky.write().await.insert(мяу_uid, мяу_ch);
    мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_STICKY_SETUP").await).await
}

/// Remove sticky message from current channel
#[poise::command(prefix_command, slash_command, rename = "dsticky")]
pub async fn мяу_антилипкость_101__(ctx: Context<'_>) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }

    мяф!(мяу_ключ <- ctx.channel_id().to_string());
    мяф!(мяу_снесли <- ctx.data().sticky_messages.write().await.remove(&мяу_ключ));
    ctx.data().мяу_сохрани_липкие_32__().await;

    if мяу_снесли.is_some() {
        мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_STICKY_REMOVED").await).await
    } else {
        мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_STICKY_NOT_FOUND").await).await
    }
}

/// Set auto-role for new guild members
#[poise::command(prefix_command, slash_command, rename = "ar")]
pub async fn мяу_autorole_102__(
    ctx: Context<'_>,
    #[description = "Role for new members"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }

    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await,
    };

    if let Some(role) = role {
        ctx.data().auto_roles.write().await.insert(guild_id, role.id.to_string());
        ctx.data().мяу_сохрани_autoroles_31__().await;
        return мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_AUTOROLE_SET").await.replace("{role}", &role.name)).await;
    }

    let current = ctx.data().auto_roles.read().await.get(&guild_id).cloned();
    match current {
        Some(role_id) => мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_AUTOROLE_CURRENT").await.replace("{role_id}", &role_id)).await,
        None => мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_AUTOROLE_EMPTY").await).await,
    }
}

/// Assign private-voice role (compat with !pvoice)
#[poise::command(prefix_command, slash_command, rename = "pvoice")]
pub async fn мяу_pvoice_103__(
    ctx: Context<'_>,
    #[description = "Target member"] target: serenity::Member,
) -> Result<(), Error> {
    let cfg = ctx.data().мяу_конфиг_сервера_38__(ctx.guild_id()).await;
    let Some(pvoice_role_id) = cfg.pvoice_role_id.and_then(|id| id.parse::<u64>().ok()) else {
        return мяу_сервер_ошибка_99__(ctx, if crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await == "en" {
            "Private voice role is not configured. Use `!settings pvoice @Role`.".to_string()
        } else {
            "Роль private voice не настроена. Используй `!settings pvoice @Роль`.".to_string()
        }).await;
    };
    let owner_root_role_id = cfg.owner_root_role_id.and_then(|id| id.parse::<u64>().ok());
    let member = match ctx.author_member().await {
        Some(m) => m,
        None => return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_ONLY_GUILD").await).await,
    };
    let allowed = member.roles.contains(&serenity::RoleId::new(pvoice_role_id))
        || owner_root_role_id.map(|id| member.roles.contains(&serenity::RoleId::new(id))).unwrap_or(false);
    if !allowed {
        return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }

    if let Err(e) = target.add_role(ctx.http(), serenity::RoleId::new(pvoice_role_id)).await {
        return мяу_сервер_ошибка_99__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_ADD_ROLE_FAIL").await.replace("{err}", &e.to_string())).await;
    }
    
    мяу_сервер_ответ_98__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SERVER_PVOICE_ASSIGNED").await.replace("{user_id}", &target.user.id.to_string())).await
}
