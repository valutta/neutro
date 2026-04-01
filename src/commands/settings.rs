macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

use schweiz_miau_proc::{fondue, grueti_mitenand, мяу, schoggi};
мяу_предмет! {
use crate::{
    Context, Error,
    state::{МяуКонфигСервера, МяуДанные},
    v2_components::мяу_v2_посылка_90__,
};
}
use poise::serenity_prelude as serenity;
use serde_json::json;

async fn мяу_админ_настроек_126__(ctx: &Context<'_>) -> bool {
    let Some(guild_id) = ctx.guild_id() else {
        return false;
    };
    let Ok(member) = guild_id.member(ctx.http(), ctx.author().id).await else {
        return false;
    };

    if let Ok(perms) = member.permissions(ctx.cache()) {
        if perms.administrator() {
            return true;
        }
    }

    if let Some(guild) = ctx.cache().guild(guild_id) {
        if guild.owner_id == ctx.author().id {
            return true;
        }
    }

    if let Some(role_id) = ctx.data().мяу_роль_стаффа_39__(Some(guild_id)).await {
        return member.roles.contains(&role_id);
    }

    false
}

fn мяу_текст_конфига_127__(cfg: &МяуКонфигСервера, lang: &str) -> String {
    let no = if lang == "en" { "Not set" } else { "Не задано" };
    let fmt_id = |value: &Option<String>, kind: &str| -> String {
        match (value, kind) {
            (Some(id), "channel") => format!("<#{}>", id),
            (Some(id), "role") => format!("<@&{}>", id),
            (Some(id), _) => id.clone(),
            (None, _) => no.to_string(),
        }
    };
    let star_threshold = cfg
        .starboard_threshold
        .map(|v| v.to_string())
        .unwrap_or_else(|| no.to_string());

    if lang == "en" {
        format!(
            "## Server Setup\nStaff role: {}\nTerminal channel: {}\nLog channel: {}\nStarboard channel: {}\nStarboard threshold: {}\nPrivate voice role: {}\nOwner root role: {}\nStream ping role: {}\n\nExamples:\n`!settings staff @Role`\n`!settings terminal #channel`\n`!settings logs #channel`\n`!settings starboard #channel 5`\n`!settings pvoice @Role`\n`!settings ownerroot @Role`\n`!settings streamrole @Role`\n`!settings show`",
            fmt_id(&cfg.staff_role_id, "role"),
            fmt_id(&cfg.terminal_channel_id, "channel"),
            fmt_id(&cfg.log_channel_id, "channel"),
            fmt_id(&cfg.starboard_channel_id, "channel"),
            star_threshold,
            fmt_id(&cfg.pvoice_role_id, "role"),
            fmt_id(&cfg.owner_root_role_id, "role"),
            fmt_id(&cfg.stream_ping_role_id, "role"),
        )
    } else {
        format!(
            "## Настройка сервера\nСтафф-роль: {}\nКанал терминала: {}\nКанал логов: {}\nКанал starboard: {}\nПорог starboard: {}\nРоль private voice: {}\nRoot-owner роль: {}\nРоль для stream ping: {}\n\nПримеры:\n`!settings staff @Роль`\n`!settings terminal #канал`\n`!settings logs #канал`\n`!settings starboard #канал 5`\n`!settings pvoice @Роль`\n`!settings ownerroot @Роль`\n`!settings streamrole @Роль`\n`!settings show`",
            fmt_id(&cfg.staff_role_id, "role"),
            fmt_id(&cfg.terminal_channel_id, "channel"),
            fmt_id(&cfg.log_channel_id, "channel"),
            fmt_id(&cfg.starboard_channel_id, "channel"),
            star_threshold,
            fmt_id(&cfg.pvoice_role_id, "role"),
            fmt_id(&cfg.owner_root_role_id, "role"),
            fmt_id(&cfg.stream_ping_role_id, "role"),
        )
    }
}

#[grueti_mitenand]
async fn мяу_ответ_128__(ctx: Context<'_>, text: String) -> Result<(), Error> {
    мяу_v2_посылка_90__(ctx, fondue!(json!([{
        "type": 17,
        "components": [{ "type": 10, "content": text }]
    }]))).await?;
    Ok(())
}

async fn мяу_обнови_конфиг_129__(
    data: &МяуДанные,
    guild_id: serenity::GuildId,
    update: impl FnOnce(&mut МяуКонфигСервера),
) {
    {
        let mut all = data.guild_configs.write().await;
        let cfg = all.entry(guild_id.to_string()).or_default();
        update(cfg);
    }
    data.мяу_сохрани_конфиги_37__().await;
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "settings",
    subcommands(
        "мяу_язык_97__",
        "мяу_show_130__",
        "мяу_staff_131__",
        "мяу_terminal_132__",
        "мяу_logs_133__",
        "мяу_starboard_134__",
        "мяу_pvoice_135__",
        "мяу_ownerroot_136__",
        "мяу_streamrole_137__"
    )
)]
#[мяу]
pub async fn мяу_настройка_96__(ctx: Context<'_>) -> Result<(), Error> {
    let lang = schoggi!(crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await);
    let cfg = ctx.data().мяу_конфиг_сервера_38__(ctx.guild_id()).await;
    мяу_ответ_128__(ctx, мяу_текст_конфига_127__(&cfg, &lang)).await
}

#[poise::command(slash_command, prefix_command, rename = "setup")]
pub async fn мяу_setup_138__(ctx: Context<'_>) -> Result<(), Error> {
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await;
    let cfg = ctx.data().мяу_конфиг_сервера_38__(ctx.guild_id()).await;
    мяу_ответ_128__(ctx, мяу_текст_конфига_127__(&cfg, &lang)).await
}

#[poise::command(slash_command, prefix_command, rename = "show")]
pub async fn мяу_show_130__(ctx: Context<'_>) -> Result<(), Error> {
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await;
    let cfg = ctx.data().мяу_конфиг_сервера_38__(ctx.guild_id()).await;
    мяу_ответ_128__(ctx, мяу_текст_конфига_127__(&cfg, &lang)).await
}

#[poise::command(slash_command, prefix_command, rename = "language")]
pub async fn мяу_язык_97__(
    ctx: Context<'_>,
    #[description = "Language code (ru or en)"] lang: String,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await?;
        return Ok(());
    }

    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => {
            мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await?;
            return Ok(());
        }
    };

    let target_lang = lang.to_lowercase();
    if target_lang != "ru" && target_lang != "en" {
        мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "SETTINGS_INVALID_LANG").await).await?;
        return Ok(());
    }

    ctx.data().guild_languages.write().await.insert(guild_id, target_lang.clone());
    ctx.data().мяу_сохрани_языки_35__().await;

    let response = if target_lang == "ru" {
        crate::i18n::мяу_скажи_91__(&ctx, "SETTINGS_LANG_RU").await
    } else {
        crate::i18n::мяу_скажи_91__(&ctx, "SETTINGS_LANG_EN").await
    };

    мяу_ответ_128__(ctx, response).await
}

#[poise::command(slash_command, prefix_command, rename = "staff")]
pub async fn мяу_staff_131__(
    ctx: Context<'_>,
    #[description = "Role mention or ID"] role: serenity::Role,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.staff_role_id = Some(role.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Staff role saved: <@&{}>", role.id)
    } else {
        format!("Стафф-роль сохранена: <@&{}>", role.id)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "terminal")]
pub async fn мяу_terminal_132__(
    ctx: Context<'_>,
    #[description = "Channel"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.terminal_channel_id = Some(channel.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Terminal channel saved: <#{}>", channel.id)
    } else {
        format!("Канал терминала сохранён: <#{}>", channel.id)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "logs")]
pub async fn мяу_logs_133__(
    ctx: Context<'_>,
    #[description = "Channel"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.log_channel_id = Some(channel.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Log channel saved: <#{}>", channel.id)
    } else {
        format!("Канал логов сохранён: <#{}>", channel.id)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "starboard")]
pub async fn мяу_starboard_134__(
    ctx: Context<'_>,
    #[description = "Channel"] channel: serenity::GuildChannel,
    #[description = "Minimum star count"] threshold: u64,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| {
        cfg.starboard_channel_id = Some(channel.id.to_string());
        cfg.starboard_threshold = Some(threshold);
    }).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Starboard saved: <#{}>, threshold {}", channel.id, threshold)
    } else {
        format!("Starboard сохранён: <#{}>, порог {}", channel.id, threshold)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "pvoice")]
pub async fn мяу_pvoice_135__(
    ctx: Context<'_>,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.pvoice_role_id = Some(role.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Private voice role saved: <@&{}>", role.id)
    } else {
        format!("Роль private voice сохранена: <@&{}>", role.id)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "ownerroot")]
pub async fn мяу_ownerroot_136__(
    ctx: Context<'_>,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.owner_root_role_id = Some(role.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Owner root role saved: <@&{}>", role.id)
    } else {
        format!("Root-owner роль сохранена: <@&{}>", role.id)
    };
    мяу_ответ_128__(ctx, text).await
}

#[poise::command(slash_command, prefix_command, rename = "streamrole")]
pub async fn мяу_streamrole_137__(
    ctx: Context<'_>,
    #[description = "Role"] role: serenity::Role,
) -> Result<(), Error> {
    if !мяу_админ_настроек_126__(&ctx).await {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_STAFF").await).await;
    }
    let Some(guild_id) = ctx.guild_id() else {
        return мяу_ответ_128__(ctx, crate::i18n::мяу_скажи_91__(&ctx, "ERR_NO_GUILD").await).await;
    };
    мяу_обнови_конфиг_129__(ctx.data(), guild_id, |cfg| cfg.stream_ping_role_id = Some(role.id.to_string())).await;
    let lang = crate::i18n::мяу_язык_сервера_92__(ctx.data(), Some(guild_id)).await;
    let text = if lang == "en" {
        format!("Stream ping role saved: <@&{}>", role.id)
    } else {
        format!("Роль stream ping сохранена: <@&{}>", role.id)
    };
    мяу_ответ_128__(ctx, text).await
}
