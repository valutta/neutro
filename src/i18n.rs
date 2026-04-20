use crate::Context;
use crate::Data;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;

pub async fn t(ctx: &Context<'_>, key: &str) -> String {
    let lang = lang_for_guild(ctx.data(), ctx.guild_id()).await;
    let dict = get_dict(&lang);
    dict.get(key).unwrap_or(&key).to_string()
}

pub async fn lang_for_guild(data: &Data, guild_id: Option<serenity::GuildId>) -> String {
    if let Some(guild_id) = guild_id {
        let settings = data.guild_settings.read().await;
        settings
            .get(&guild_id.to_string())
            .map(|entry| entry.language.clone())
            .unwrap_or_else(|| "ru".to_string())
    } else {
        "ru".to_string()
    }
}

fn get_dict(lang: &str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    match lang {
        "en" => {
            map.insert(
                "ERR_NO_STAFF",
                "You do not have staff permissions for this command.",
            );
            map.insert("ERR_NO_GUILD", "This command can only be used in a server.");
            map.insert("KICK_SUCCESS", "Kicked {user}. Reason: {reason}");
            map.insert("KICK_FAIL", "Failed to kick {user}: {err}");
            map.insert("BAN_SUCCESS", "Banned {user}. Reason: {reason}");
            map.insert("BAN_FAIL", "Failed to ban {user}: {err}");
            map.insert(
                "MUTE_SUCCESS",
                "Muted {user} for {duration}. Reason: {reason}",
            );
            map.insert("MUTE_FAIL", "Failed to mute {user}: {err}");
            map.insert("UNMUTE_SUCCESS", "Unmuted {user}");
            map.insert("UNMUTE_FAIL", "Failed to unmute {user}: {err}");
            map.insert("CLEAR_FAIL", "Failed to clear messages: {err}");
            map.insert("CLEAR_SUCCESS", "Cleared {amount} messages.");
            map.insert("ROLE_REMOVE_FAIL", "Failed to remove role: {err}");
            map.insert("ROLE_REMOVE_SUCCESS", "Removed role {role} from {user}");
            map.insert("ROLE_ADD_FAIL", "Failed to add role: {err}");
            map.insert("ROLE_ADD_SUCCESS", "Added role {role} to {user}");
            map.insert("ANNOUNCE_FAIL", "Failed to announce: {err}");
            map.insert("ANNOUNCE_SUCCESS", "Announcement sent.");
            map.insert("EANNOUNCE_SUCCESS", "Everyone announcement sent.");
            map.insert("STREAM_FAIL", "Failed to send stream announcement: {err}");
            map.insert(
                "NO_DURATION",
                "Invalid duration format. Example: 30m / 2h / 1d",
            );
            map.insert("PING", "Pong! Ping: **{ms} ms**");
            map.insert("NO_TEXT", "Please provide text.");
            map.insert(
                "SETTINGS_HELP",
                "Use `/settings language <ru|en>` to configure the bot.",
            );
            map.insert(
                "SETTINGS_INVALID_LANG",
                "Invalid language. Please use `ru` or `en`.",
            );
            map.insert(
                "SETTINGS_LANG_RU",
                "Server language successfully changed to Russian.",
            );
            map.insert(
                "SETTINGS_LANG_EN",
                "Server language successfully changed to English.",
            );
            map.insert("UTILITY_USER_ID", "User ID: `{id}`");
            map.insert("UTILITY_ROLE_ID", "Role ID: `{id}`");
            map.insert("UTILITY_AVATAR", "## Avatar for {user}");
            map.insert("UTILITY_PROFILE_JOINED", "\n**Joined Server:** {joined}");
            map.insert("UTILITY_PROFILE_ROLES", "\n**Roles:** {roles}");
            map.insert(
                "UTILITY_PROFILE_ROLES_HIDDEN",
                "\n**Roles:** {count} roles hidden",
            );
            map.insert(
                "UTILITY_PROFILE",
                "## Profile - {user}\n**ID:** {id}\n**Registered:** {registered}{joined}{roles}",
            );
            map.insert(
                "UTILITY_HELP_TITLE",
                "## NeutroBot Commands\nAvailable commands:",
            );
            map.insert(
                "UTILITY_HELP_BASIC",
                "**Basic**\n`!ping` `!rid` `!id` `!avatar` `!profile` `!aquote` `!help`",
            );
            map.insert("UTILITY_HELP_MOD", "**Moderation**\n`!kick` `!ban` `!mute` `!tempmute` `!unmute` `!clear` `!role` `!announce` `!eannounce` `!sticky` `!dsticky`\n`!stream` `!ar` `!pvoice`");
            map.insert(
                "UTILITY_AQUOTE_USAGE",
                "Provide text. Example: `!aquote #2B2D31 hello`",
            );
            map.insert(
                "UTILITY_AQUOTE_AFTER_COLOR",
                "Text is required after the color.",
            );
            map.insert("SERVER_STICKY_SETUP", "**Sticky Message Setup**\nSend the next message with text and/or an image. It will be saved as the sticky message for this channel.");
            map.insert("SERVER_STICKY_REMOVED", "Sticky message removed.");
            map.insert(
                "SERVER_STICKY_NOT_FOUND",
                "No sticky message is configured for this channel.",
            );
            map.insert("SERVER_AUTOROLE_SET", "Auto-role set to @{role}");
            map.insert("SERVER_AUTOROLE_CURRENT", "Current auto-role ID: {role_id}");
            map.insert(
                "SERVER_AUTOROLE_EMPTY",
                "Auto-role is not configured. Example: `!ar @Role`",
            );
            map.insert(
                "SERVER_ONLY_GUILD",
                "This command can only be used in a server.",
            );
            map.insert(
                "SERVER_PVOICE_ASSIGNED",
                "Assigned Private Voice role to <@{user_id}>",
            );
            map.insert("SERVER_ADD_ROLE_FAIL", "Failed to add role: {err}");
            map.insert("MOD_REASON_NONE", "No reason provided");
            map.insert(
                "MOD_MUST_BE_GUILD",
                "This command can only be used in a server.",
            );
            map.insert("MOD_CLEAR_FAIL", "Failed to clear messages: {err}");
            map.insert("MOD_ROLE_REMOVE_FAIL", "Failed to remove role: {err}");
            map.insert("MOD_ROLE_REMOVE_SUCCESS", "Removed role {role} from {user}");
            map.insert("MOD_ROLE_ADD_FAIL", "Failed to add role: {err}");
            map.insert("MOD_ROLE_ADD_SUCCESS", "Added role {role} to {user}");
            map.insert("MOD_ANNOUNCE_SENT", "Announcement sent.");
            map.insert(
                "MOD_STREAM_TEXT",
                "<@&{role_id}> {user} started a stream. Watch here: {link}",
            );
        }
        _ => {
            // "ru" as default
            map.insert("ERR_NO_STAFF", "У тебя нет прав стаффа для этой команды.");
            map.insert("ERR_NO_GUILD", "Команда доступна только на сервере.");
            map.insert("KICK_SUCCESS", "Кикнул {user}. Причина: {reason}");
            map.insert("KICK_FAIL", "Не удалось кикнуть {user}: {err}");
            map.insert("BAN_SUCCESS", "Забанил {user}. Причина: {reason}");
            map.insert("BAN_FAIL", "Не удалось забанить {user}: {err}");
            map.insert(
                "MUTE_SUCCESS",
                "Замутил {user} на {duration}. Причина: {reason}",
            );
            map.insert("MUTE_FAIL", "Не удалось замутить {user}: {err}");
            map.insert("UNMUTE_SUCCESS", "Размутил {user}");
            map.insert("UNMUTE_FAIL", "Не удалось размутить {user}: {err}");
            map.insert("CLEAR_FAIL", "Не удалось удалить сообщения: {err}");
            map.insert("CLEAR_SUCCESS", "Удалено {amount} сообщений.");
            map.insert("ROLE_REMOVE_FAIL", "ОШИБКА: Не удалось снять роль: {err}");
            map.insert("ROLE_REMOVE_SUCCESS", "Снял роль {role} с {user}");
            map.insert("ROLE_ADD_FAIL", "ОШИБКА: Не удалось выдать роль: {err}");
            map.insert("ROLE_ADD_SUCCESS", "Выдал роль {role} пользователю {user}");
            map.insert("ANNOUNCE_FAIL", "ОШИБКА отправки анонса: {err}");
            map.insert("ANNOUNCE_SUCCESS", "Анонс отправлен.");
            map.insert("EANNOUNCE_SUCCESS", "Анонс с пингом отправлен.");
            map.insert("STREAM_FAIL", "Ошибка отправки стрим-уведомления: {err}");
            map.insert(
                "NO_DURATION",
                "Неверный формат времени. Пример: `30m` / `2h` / `1d`",
            );
            map.insert("PING", "Понг! Пинг: **{ms} ms**");
            map.insert("NO_TEXT", "Укажи текст.");
            map.insert(
                "SETTINGS_HELP",
                "Используй `/settings language <ru|en>`, чтобы настроить язык бота.",
            );
            map.insert(
                "SETTINGS_INVALID_LANG",
                "Неверный язык. Используй `ru` или `en`.",
            );
            map.insert(
                "SETTINGS_LANG_RU",
                "Язык сервера успешно изменен на русский.",
            );
            map.insert(
                "SETTINGS_LANG_EN",
                "Язык сервера успешно изменен на английский.",
            );
            map.insert("UTILITY_USER_ID", "User ID: `{id}`");
            map.insert("UTILITY_ROLE_ID", "Role ID: `{id}`");
            map.insert("UTILITY_AVATAR", "## Аватар пользователя {user}");
            map.insert("UTILITY_PROFILE_JOINED", "\n**На сервере с:** {joined}");
            map.insert("UTILITY_PROFILE_ROLES", "\n**Роли:** {roles}");
            map.insert("UTILITY_PROFILE_ROLES_HIDDEN", "\n**Роли:** скрыто {count}");
            map.insert("UTILITY_PROFILE", "## Профиль - {user}\n**ID:** {id}\n**Зарегистрирован:** {registered}{joined}{roles}");
            map.insert(
                "UTILITY_HELP_TITLE",
                "## Команды NeutroBot\nДоступные команды:",
            );
            map.insert(
                "UTILITY_HELP_BASIC",
                "**Основные**\n`!ping` `!rid` `!id` `!avatar` `!profile` `!aquote` `!help`",
            );
            map.insert("UTILITY_HELP_MOD", "**Модерация**\n`!kick` `!ban` `!mute` `!tempmute` `!unmute` `!clear` `!role` `!announce` `!eannounce` `!sticky` `!dsticky`\n`!stream` `!ar` `!pvoice`");
            map.insert(
                "UTILITY_AQUOTE_USAGE",
                "Укажи текст. Пример: `!aquote #2B2D31 hello`",
            );
            map.insert("UTILITY_AQUOTE_AFTER_COLOR", "После цвета нужен текст.");
            map.insert("SERVER_STICKY_SETUP", "**Настройка Sticky Message**\nОтправь следующим сообщением текст и/или картинку. Это сообщение будет сохранено как sticky для канала.");
            map.insert("SERVER_STICKY_REMOVED", "Sticky-сообщение удалено.");
            map.insert(
                "SERVER_STICKY_NOT_FOUND",
                "В этом канале sticky-сообщение не найдено.",
            );
            map.insert("SERVER_AUTOROLE_SET", "Auto-role установлен: @{role}");
            map.insert("SERVER_AUTOROLE_CURRENT", "Текущий auto-role ID: {role_id}");
            map.insert(
                "SERVER_AUTOROLE_EMPTY",
                "Auto-role не установлен. Пример: `!ar @Role`",
            );
            map.insert("SERVER_ONLY_GUILD", "Команда доступна только на сервере.");
            map.insert(
                "SERVER_PVOICE_ASSIGNED",
                "Выдана роль Private Voice пользователю <@{user_id}>",
            );
            map.insert("SERVER_ADD_ROLE_FAIL", "Не удалось выдать роль: {err}");
            map.insert("MOD_REASON_NONE", "Причина не указана");
            map.insert("MOD_MUST_BE_GUILD", "Команда доступна только на сервере.");
            map.insert("MOD_CLEAR_FAIL", "Не удалось удалить сообщения: {err}");
            map.insert("MOD_ROLE_REMOVE_FAIL", "Не удалось снять роль: {err}");
            map.insert("MOD_ROLE_REMOVE_SUCCESS", "Снял роль {role} с {user}");
            map.insert("MOD_ROLE_ADD_FAIL", "Не удалось выдать роль: {err}");
            map.insert(
                "MOD_ROLE_ADD_SUCCESS",
                "Выдал роль {role} пользователю {user}",
            );
            map.insert("MOD_ANNOUNCE_SENT", "Анонс отправлен.");
            map.insert(
                "MOD_STREAM_TEXT",
                "<@&{role_id}> {user} запустил(а) стрим. Ссылка: {link}",
            );
        }
    }
    map
}
