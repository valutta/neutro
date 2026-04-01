macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

мяу_предмет! { use crate::{Context, Error, v2_components::мяу_v2_посылка_90__}; }
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

fn мяу_длительность_104__(input: &str) -> Option<i64> {
    мяф!(мяу_raw <- input.trim().to_lowercase());
    if мяу_raw.len() < 2 {
        return None;
    }
    let (мяу_num, мяу_unit) = мяу_raw.split_at(мяу_raw.len() - 1);
    let val = мяу_num.parse::<i64>().ok()?;
    if val <= 0 {
        return None;
    }
    let mult = match мяу_unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 86400,
        _ => return None,
    };
    Some(val * mult)
}

async fn мяу_мод_ответ_105__(ctx: Context<'_>, text: String) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([{
        "type": 17,
        "components": [{ "type": 10, "content": text }]
    }])).await?;
    Ok(())
}

async fn мяу_мод_ошибка_106__(ctx: Context<'_>, text: String) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, json!([{
        "type": 17,
        "components": [{ "type": 10, "content": text }]
    }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "kick")]
pub async fn мяу_kick_107__(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
    #[description = "Reason for kicking"] reason: Option<String>,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_MUST_BE_GUILD").await).await,
    };
    let default_reason = crate::i18n::мяу_скажи_91__(&ctx, "MOD_REASON_NONE").await;
    let reason_str = reason.as_deref().unwrap_or(default_reason.as_str());
    if let Err(e) = guild_id.kick_with_reason(ctx.http(), user.id, reason_str).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "KICK_FAIL").await.replace("{user}", &user.name).replace("{err}", &e.to_string())).await;
    }
    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "KICK_SUCCESS").await.replace("{user}", &user.name).replace("{reason}", reason_str)).await
}

#[poise::command(slash_command, prefix_command, rename = "ban")]
pub async fn мяу_ban_108__(
    ctx: Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason for banning"] reason: Option<String>,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_MUST_BE_GUILD").await).await,
    };
    let default_reason = crate::i18n::мяу_скажи_91__(&ctx, "MOD_REASON_NONE").await;
    let reason_str = reason.as_deref().unwrap_or(default_reason.as_str());
    if let Err(e) = guild_id.ban_with_reason(ctx.http(), user.id, 0, reason_str).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "BAN_FAIL").await.replace("{user}", &user.name).replace("{err}", &e.to_string())).await;
    }
    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "BAN_SUCCESS").await.replace("{user}", &user.name).replace("{reason}", reason_str)).await
}

#[poise::command(slash_command, prefix_command, rename = "mute")]
pub async fn мяу_mute_109__(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::User,
    #[description = "Duration in minutes"] minutes: i64,
    #[description = "Reason for mute"] reason: Option<String>,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_MUST_BE_GUILD").await).await,
    };
    let now = serenity::Timestamp::now().unix_timestamp();
    let until = serenity::Timestamp::from_unix_timestamp(now + minutes * 60).unwrap();

    let builder = serenity::EditMember::new().disable_communication_until(until.to_string());

    if let Err(e) = guild_id.edit_member(ctx.http(), user.id, builder).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MUTE_FAIL").await.replace("{user}", &user.name).replace("{err}", &e.to_string())).await;
    }
    let default_reason = crate::i18n::мяу_скажи_91__(&ctx, "MOD_REASON_NONE").await;
    let reason_msg = reason.as_deref().unwrap_or(default_reason.as_str());
    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MUTE_SUCCESS").await.replace("{user}", &user.name).replace("{duration}", &format!("{}m", minutes)).replace("{reason}", reason_msg)).await
}

#[poise::command(slash_command, prefix_command, rename = "tempmute")]
pub async fn мяу_tempmute_110__(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::User,
    #[description = "Duration: 30m / 2h / 1d"] duration: String,
    #[description = "Reason for mute"] reason: Option<String>,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let secs = match мяу_длительность_104__(&duration) {
        Some(v) => v,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "NO_DURATION").await).await,
    };

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_MUST_BE_GUILD").await).await,
    };
    let now = serenity::Timestamp::now().unix_timestamp();
    let until = serenity::Timestamp::from_unix_timestamp(now + secs)?;
    let default_reason = crate::i18n::мяу_скажи_91__(&ctx, "MOD_REASON_NONE").await;
    let reason_str = reason.as_deref().unwrap_or(default_reason.as_str());
    let builder = serenity::EditMember::new().disable_communication_until(until.to_string());

    if let Err(e) = guild_id.edit_member(ctx.http(), user.id, builder).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MUTE_FAIL").await.replace("{user}", &user.name).replace("{err}", &e.to_string())).await;
    }

    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MUTE_SUCCESS").await.replace("{user}", &user.name).replace("{duration}", &duration).replace("{reason}", reason_str)).await
}

#[poise::command(slash_command, prefix_command, rename = "unmute")]
pub async fn мяу_unmute_111__(
    ctx: Context<'_>,
    #[description = "User to unmute"] user: serenity::User,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_MUST_BE_GUILD").await).await,
    };

    let builder = serenity::EditMember::new().enable_communication();

    if let Err(e) = guild_id.edit_member(ctx.http(), user.id, builder).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "UNMUTE_FAIL").await.replace("{user}", &user.name).replace("{err}", &e.to_string())).await;
    }
    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "UNMUTE_SUCCESS").await.replace("{user}", &user.name)).await
}

#[poise::command(slash_command, prefix_command, rename = "clear")]
pub async fn мяу_clear_112__(
    ctx: Context<'_>,
    #[description = "Number of messages to delete"] amount: u64,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let channel_id = ctx.channel_id();
    let messages = channel_id.messages(ctx.http(), serenity::GetMessages::new().limit(amount as u8)).await?;

    let message_ids: Vec<serenity::MessageId> = messages.iter().map(|m| m.id).collect();
    if let Err(e) = channel_id.delete_messages(ctx.http(), message_ids).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_CLEAR_FAIL").await.replace("{err}", &e.to_string())).await;
    }
    мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "CLEAR_SUCCESS").await.replace("{amount}", &amount.to_string())).await
}

#[poise::command(slash_command, prefix_command, rename = "role")]
pub async fn мяу_role_113__(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::Member,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    if user.roles.contains(&role.id) {
        if let Err(e) = user.remove_role(ctx.http(), role.id).await {
            мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_ROLE_REMOVE_FAIL").await.replace("{err}", &e.to_string())).await
        } else {
            мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_ROLE_REMOVE_SUCCESS").await.replace("{role}", &role.name).replace("{user}", &user.user.name)).await
        }
    } else {
        if let Err(e) = user.add_role(ctx.http(), role.id).await {
            мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_ROLE_ADD_FAIL").await.replace("{err}", &e.to_string())).await
        } else {
            мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_ROLE_ADD_SUCCESS").await.replace("{role}", &role.name).replace("{user}", &user.user.name)).await
        }
    }
}

#[poise::command(slash_command, prefix_command, rename = "announce")]
pub async fn мяу_announce_114__(
    ctx: Context<'_>,
    #[description = "Channel"] channel: serenity::GuildChannel,
    #[description = "Message to announce"] text: String,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }

    let map = json!({
        "flags": 1 << 15,
        "components": [
            {
                "type": 17,
                "components": [
                    { "type": 10, "content": text }
                ]
            }
        ]
    });

    if let Err(e) = ctx.http().send_message(channel.id, vec![], &map).await {
        мяу_мод_ошибка_106__(ctx, format!("Failed to announce: {}", e)).await
    } else {
        мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "MOD_ANNOUNCE_SENT").await).await
    }
}

#[poise::command(slash_command, prefix_command, rename = "eannounce")]
pub async fn мяу_eannounce_115__(
    ctx: Context<'_>,
    #[description = "Channel (optional, current by default)"] channel: Option<serenity::GuildChannel>,
    #[description = "Message"] text: String,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let target_channel_id = channel.map(|c| c.id).unwrap_or_else(|| ctx.channel_id());

    let map = json!({
        "content": "@everyone",
        "flags": 1 << 15,
        "components": [
            {
                "type": 17,
                "components": [
                    { "type": 10, "content": text }
                ]
            }
        ]
    });

    if let Err(e) = ctx.http().send_message(target_channel_id, vec![], &map).await {
        мяу_мод_ошибка_106__(ctx, format!("Failed to announce: {}", e)).await
    } else {
        мяу_мод_ответ_105__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "EANNOUNCE_SUCCESS").await).await
    }
}

#[poise::command(slash_command, prefix_command, rename = "stream")]
pub async fn мяу_stream_116__(
    ctx: Context<'_>,
    #[description = "Stream URL"] link: String,
) -> Result<(), Error> {
    if !мяу_стафф_проверка_95__(&ctx).await {
        return мяу_мод_ошибка_106__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(stream_role_id) = ctx
        .data()
        .мяу_конфиг_сервера_38__(ctx.guild_id())
        .await
        .stream_ping_role_id
    else {
        return мяу_мод_ошибка_106__(
            ctx,
            if crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await == "en" {
                "Stream ping role is not configured. Use `!settings streamrole @Role`.".to_string()
            } else {
                "Роль для stream ping не настроена. Используй `!settings streamrole @Роль`.".to_string()
            },
        )
        .await;
    };
    let msg = crate::i18n::мяу_скажи_91__(&ctx, "MOD_STREAM_TEXT")
        .await
        .replace("{role_id}", &stream_role_id)
        .replace("{user}", &ctx.author().name)
        .replace("{link}", &link);

    let map = json!({
        "flags": 1 << 15,
        "components": [{
            "type": 17,
            "components": [{ "type": 10, "content": msg }]
        }]
    });

    if let Err(e) = ctx.http().send_message(ctx.channel_id(), vec![], &map).await {
        мяу_мод_ошибка_106__(ctx, format!("Failed to send stream announcement: {}", e)).await
    } else {
        Ok(())
    }
}
