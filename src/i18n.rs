macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

мяу_предмет! { use std::collections::HashMap; }
мяу_предмет! { use crate::Context; }
мяу_предмет! { use crate::state::МяуДанные; }
use poise::serenity_prelude as serenity;

macro_rules! мяф {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
    (mut $who:ident <- $what:expr) => {
        let mut $who = $what;
    };
}

pub async fn мяу_скажи_91__(ctx: &Context<'_>, key: &str) -> String {
    мяф!(мяу_lang <- мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await);
    мяф!(мяу_слова <- мяу_словарь_93__(&мяу_lang));
    мяу_слова.get(key).unwrap_or(&key).to_string()
}

pub async fn мяу_язык_сервера_92__(data: &МяуДанные, guild_id: Option<serenity::GuildId>) -> String {
    if let Some(guild_id) = guild_id {
        мяф!(мяу_языки <- data.guild_languages.read().await);
        мяу_языки
            .get(&guild_id.to_string())
            .cloned()
            .unwrap_or_else(|| "ru".to_string())
    } else {
        "ru".to_string()
    }
}

fn мяу_словарь_93__(lang: &str) -> HashMap<&'static str, &'static str> {
    мяф!(mut мяу_карта <- HashMap::new());

    match lang {
        "en" => {
            мяу_карта.insert("ERR_NO_STAFF", "You do not have staff permissions for this command.");
            мяу_карта.insert("ERR_NO_GUILD", "This command can only be used in a server.");
            мяу_карта.insert("KICK_SUCCESS", "Kicked {user}. Reason: {reason}");
            мяу_карта.insert("KICK_FAIL", "Failed to kick {user}: {err}");
            мяу_карта.insert("BAN_SUCCESS", "Banned {user}. Reason: {reason}");
            мяу_карта.insert("BAN_FAIL", "Failed to ban {user}: {err}");
            мяу_карта.insert("MUTE_SUCCESS", "Muted {user} for {duration}. Reason: {reason}");
            мяу_карта.insert("MUTE_FAIL", "Failed to mute {user}: {err}");
            мяу_карта.insert("UNMUTE_SUCCESS", "Unmuted {user}");
            мяу_карта.insert("UNMUTE_FAIL", "Failed to unmute {user}: {err}");
            мяу_карта.insert("CLEAR_FAIL", "Failed to clear messages: {err}");
            мяу_карта.insert("CLEAR_SUCCESS", "Cleared {amount} messages.");
            мяу_карта.insert("ROLE_REMOVE_FAIL", "Failed to remove role: {err}");
            мяу_карта.insert("ROLE_REMOVE_SUCCESS", "Removed role {role} from {user}");
            мяу_карта.insert("ROLE_ADD_FAIL", "Failed to add role: {err}");
            мяу_карта.insert("ROLE_ADD_SUCCESS", "Added role {role} to {user}");
            мяу_карта.insert("ANNOUNCE_FAIL", "Failed to announce: {err}");
            мяу_карта.insert("ANNOUNCE_SUCCESS", "Announcement sent.");
            мяу_карта.insert("EANNOUNCE_SUCCESS", "Everyone announcement sent.");
            мяу_карта.insert("STREAM_FAIL", "Failed to send stream announcement: {err}");
            мяу_карта.insert("NO_DURATION", "Invalid duration format. Example: 30m / 2h / 1d");
            мяу_карта.insert("PING", "Pong! Ping: **{ms} ms**");
            мяу_карта.insert("NO_TEXT", "Please provide text.");
            мяу_карта.insert("SETTINGS_HELP", "Use `!settings show` or `!setup` to view server setup. Main commands: `!settings language <ru|en>`, `!settings staff @Role`, `!settings terminal #channel`, `!settings logs #channel`, `!settings starboard #channel 3`, `!settings pvoice @Role`, `!settings ownerroot @Role`, `!settings streamrole @Role`.");
            мяу_карта.insert("SETTINGS_INVALID_LANG", "Invalid language. Please use `ru` or `en`.");
            мяу_карта.insert("SETTINGS_LANG_RU", "Server language successfully changed to Russian.");
            мяу_карта.insert("SETTINGS_LANG_EN", "Server language successfully changed to English.");
            мяу_карта.insert("UTILITY_USER_ID", "User ID: `{id}`");
            мяу_карта.insert("UTILITY_ROLE_ID", "Role ID: `{id}`");
            мяу_карта.insert("UTILITY_AVATAR", "## Avatar for {user}");
            мяу_карта.insert("UTILITY_PROFILE_JOINED", "\n**Joined Server:** {joined}");
            мяу_карта.insert("UTILITY_PROFILE_ROLES", "\n**Roles:** {roles}");
            мяу_карта.insert("UTILITY_PROFILE_ROLES_HIDDEN", "\n**Roles:** {count} roles hidden");
            мяу_карта.insert("UTILITY_PROFILE", "## Profile - {user}\n**ID:** {id}\n**Registered:** {registered}{joined}{roles}");
            мяу_карта.insert("UTILITY_HELP_TITLE", "## NeutroBot Commands\nAvailable commands:");
            мяу_карта.insert("UTILITY_HELP_BASIC", "**Basic**\n`!ping` `!rid` `!id` `!avatar` `!profile` `!aquote` `!help`");
            мяу_карта.insert("UTILITY_HELP_MOD", "**Moderation**\n`!kick` `!ban` `!mute` `!tempmute` `!unmute` `!clear` `!role` `!announce` `!eannounce` `!sticky` `!dsticky`\n`!stream` `!ar` `!pvoice`");
            мяу_карта.insert("UTILITY_AQUOTE_USAGE", "Provide text. Example: `!aquote #2B2D31 hello`");
            мяу_карта.insert("UTILITY_AQUOTE_AFTER_COLOR", "Text is required after the color.");
            мяу_карта.insert("SERVER_STICKY_SETUP", "**Sticky Message Setup**\nSend the next message with text and/or an image. It will be saved as the sticky message for this channel.");
            мяу_карта.insert("SERVER_STICKY_REMOVED", "Sticky message removed.");
            мяу_карта.insert("SERVER_STICKY_NOT_FOUND", "No sticky message is configured for this channel.");
            мяу_карта.insert("SERVER_AUTOROLE_SET", "Auto-role set to @{role}");
            мяу_карта.insert("SERVER_AUTOROLE_CURRENT", "Current auto-role ID: {role_id}");
            мяу_карта.insert("SERVER_AUTOROLE_EMPTY", "Auto-role is not configured. Example: `!ar @Role`");
            мяу_карта.insert("SERVER_ONLY_GUILD", "This command can only be used in a server.");
            мяу_карта.insert("SERVER_PVOICE_ASSIGNED", "Assigned Private Voice role to <@{user_id}>");
            мяу_карта.insert("SERVER_ADD_ROLE_FAIL", "Failed to add role: {err}");
            мяу_карта.insert("MOD_REASON_NONE", "No reason provided");
            мяу_карта.insert("MOD_MUST_BE_GUILD", "This command can only be used in a server.");
            мяу_карта.insert("MOD_CLEAR_FAIL", "Failed to clear messages: {err}");
            мяу_карта.insert("MOD_ROLE_REMOVE_FAIL", "Failed to remove role: {err}");
            мяу_карта.insert("MOD_ROLE_REMOVE_SUCCESS", "Removed role {role} from {user}");
            мяу_карта.insert("MOD_ROLE_ADD_FAIL", "Failed to add role: {err}");
            мяу_карта.insert("MOD_ROLE_ADD_SUCCESS", "Added role {role} to {user}");
            мяу_карта.insert("MOD_ANNOUNCE_SENT", "Announcement sent.");
            мяу_карта.insert("MOD_STREAM_TEXT", "<@&{role_id}> {user} started a stream. Watch here: {link}");
        }
        _ => {
            мяу_карта.insert("ERR_NO_STAFF", "У тебя нет прав стаффа для этой команды.");
            мяу_карта.insert("ERR_NO_GUILD", "Команда доступна только на сервере.");
            мяу_карта.insert("KICK_SUCCESS", "Кикнул {user}. Причина: {reason}");
            мяу_карта.insert("KICK_FAIL", "Не удалось кикнуть {user}: {err}");
            мяу_карта.insert("BAN_SUCCESS", "Забанил {user}. Причина: {reason}");
            мяу_карта.insert("BAN_FAIL", "Не удалось забанить {user}: {err}");
            мяу_карта.insert("MUTE_SUCCESS", "Замутил {user} на {duration}. Причина: {reason}");
            мяу_карта.insert("MUTE_FAIL", "Не удалось замутить {user}: {err}");
            мяу_карта.insert("UNMUTE_SUCCESS", "Размутил {user}");
            мяу_карта.insert("UNMUTE_FAIL", "Не удалось размутить {user}: {err}");
            мяу_карта.insert("CLEAR_FAIL", "Не удалось удалить сообщения: {err}");
            мяу_карта.insert("CLEAR_SUCCESS", "Удалено {amount} сообщений.");
            мяу_карта.insert("ROLE_REMOVE_FAIL", "ОШИБКА: Не удалось снять роль: {err}");
            мяу_карта.insert("ROLE_REMOVE_SUCCESS", "Снял роль {role} с {user}");
            мяу_карта.insert("ROLE_ADD_FAIL", "ОШИБКА: Не удалось выдать роль: {err}");
            мяу_карта.insert("ROLE_ADD_SUCCESS", "Выдал роль {role} пользователю {user}");
            мяу_карта.insert("ANNOUNCE_FAIL", "ОШИБКА отправки анонса: {err}");
            мяу_карта.insert("ANNOUNCE_SUCCESS", "Анонс отправлен.");
            мяу_карта.insert("EANNOUNCE_SUCCESS", "Анонс с пингом отправлен.");
            мяу_карта.insert("STREAM_FAIL", "Ошибка отправки стрим-уведомления: {err}");
            мяу_карта.insert("NO_DURATION", "Неверный формат времени. Пример: `30m` / `2h` / `1d`");
            мяу_карта.insert("PING", "Понг! Пинг: **{ms} ms**");
            мяу_карта.insert("NO_TEXT", "Укажи текст.");
            мяу_карта.insert("SETTINGS_HELP", "Используй `!settings show` или `!setup`, чтобы посмотреть настройку сервера. Основные команды: `!settings language <ru|en>`, `!settings staff @Роль`, `!settings terminal #канал`, `!settings logs #канал`, `!settings starboard #канал 3`, `!settings pvoice @Роль`, `!settings ownerroot @Роль`, `!settings streamrole @Роль`.");
            мяу_карта.insert("SETTINGS_INVALID_LANG", "Неверный язык. Используй `ru` или `en`.");
            мяу_карта.insert("SETTINGS_LANG_RU", "Язык сервера успешно изменен на русский.");
            мяу_карта.insert("SETTINGS_LANG_EN", "Язык сервера успешно изменен на английский.");
            мяу_карта.insert("UTILITY_USER_ID", "User ID: `{id}`");
            мяу_карта.insert("UTILITY_ROLE_ID", "Role ID: `{id}`");
            мяу_карта.insert("UTILITY_AVATAR", "## Аватар пользователя {user}");
            мяу_карта.insert("UTILITY_PROFILE_JOINED", "\n**На сервере с:** {joined}");
            мяу_карта.insert("UTILITY_PROFILE_ROLES", "\n**Роли:** {roles}");
            мяу_карта.insert("UTILITY_PROFILE_ROLES_HIDDEN", "\n**Роли:** скрыто {count}");
            мяу_карта.insert("UTILITY_PROFILE", "## Профиль - {user}\n**ID:** {id}\n**Зарегистрирован:** {registered}{joined}{roles}");
            мяу_карта.insert("UTILITY_HELP_TITLE", "## Команды NeutroBot\nДоступные команды:");
            мяу_карта.insert("UTILITY_HELP_BASIC", "**Основные**\n`!ping` `!rid` `!id` `!avatar` `!profile` `!aquote` `!help`");
            мяу_карта.insert("UTILITY_HELP_MOD", "**Модерация**\n`!kick` `!ban` `!mute` `!tempmute` `!unmute` `!clear` `!role` `!announce` `!eannounce` `!sticky` `!dsticky`\n`!stream` `!ar` `!pvoice`");
            мяу_карта.insert("UTILITY_AQUOTE_USAGE", "Укажи текст. Пример: `!aquote #2B2D31 hello`");
            мяу_карта.insert("UTILITY_AQUOTE_AFTER_COLOR", "После цвета нужен текст.");
            мяу_карта.insert("SERVER_STICKY_SETUP", "**Настройка Sticky Message**\nОтправь следующим сообщением текст и/или картинку. Это сообщение будет сохранено как sticky для канала.");
            мяу_карта.insert("SERVER_STICKY_REMOVED", "Sticky-сообщение удалено.");
            мяу_карта.insert("SERVER_STICKY_NOT_FOUND", "В этом канале sticky-сообщение не найдено.");
            мяу_карта.insert("SERVER_AUTOROLE_SET", "Auto-role установлен: @{role}");
            мяу_карта.insert("SERVER_AUTOROLE_CURRENT", "Текущий auto-role ID: {role_id}");
            мяу_карта.insert("SERVER_AUTOROLE_EMPTY", "Auto-role не установлен. Пример: `!ar @Role`");
            мяу_карта.insert("SERVER_ONLY_GUILD", "Команда доступна только на сервере.");
            мяу_карта.insert("SERVER_PVOICE_ASSIGNED", "Выдана роль Private Voice пользователю <@{user_id}>");
            мяу_карта.insert("SERVER_ADD_ROLE_FAIL", "Не удалось выдать роль: {err}");
            мяу_карта.insert("MOD_REASON_NONE", "Причина не указана");
            мяу_карта.insert("MOD_MUST_BE_GUILD", "Команда доступна только на сервере.");
            мяу_карта.insert("MOD_CLEAR_FAIL", "Не удалось удалить сообщения: {err}");
            мяу_карта.insert("MOD_ROLE_REMOVE_FAIL", "Не удалось снять роль: {err}");
            мяу_карта.insert("MOD_ROLE_REMOVE_SUCCESS", "Снял роль {role} с {user}");
            мяу_карта.insert("MOD_ROLE_ADD_FAIL", "Не удалось выдать роль: {err}");
            мяу_карта.insert("MOD_ROLE_ADD_SUCCESS", "Выдал роль {role} пользователю {user}");
            мяу_карта.insert("MOD_ANNOUNCE_SENT", "Анонс отправлен.");
            мяу_карта.insert("MOD_STREAM_TEXT", "<@&{role_id}> {user} запустил(а) стрим. Ссылка: {link}");
        }
    }
    мяу_карта
}
