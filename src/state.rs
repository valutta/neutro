use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaRequest {
    pub original_channel_id: String,
    pub original_user_id: String,
    #[serde(default)]
    pub stored_files: Vec<String>,
    #[serde(default)]
    pub content_urls: Vec<String>,
    #[serde(default)]
    pub attachment_urls: Vec<String>,
    #[serde(default)]
    pub original_text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickyMessage {
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub last_message_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TerminalFlagDraft {
    pub owner_user_id: String,
    pub target_channel_id: Option<String>,
    pub action: Option<String>,
    pub flag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettings {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub staff_role_id: Option<u64>,
    #[serde(default)]
    pub stream_ping_role_id: Option<u64>,
    #[serde(default)]
    pub private_voice_role_id: Option<u64>,
    #[serde(default)]
    pub auto_role_id: Option<u64>,
    #[serde(default)]
    pub log_channel_id: Option<u64>,
    #[serde(default)]
    pub starboard_channel_id: Option<u64>,
    #[serde(default = "default_starboard_threshold")]
    pub starboard_threshold: u64,
    #[serde(default)]
    pub terminal_channel_id: Option<u64>,
    #[serde(default)]
    pub ai_enabled: bool,
    #[serde(default)]
    pub ai_channel_id: Option<u64>,
    #[serde(default = "default_ai_model")]
    pub ai_model: String,
}

fn default_language() -> String {
    "ru".to_string()
}

fn default_starboard_threshold() -> u64 {
    10
}

fn default_ai_model() -> String {
    "adaptive".to_string()
}

impl Default for GuildSettings {
    fn default() -> Self {
        Self {
            language: default_language(),
            staff_role_id: None,
            stream_ping_role_id: None,
            private_voice_role_id: None,
            auto_role_id: None,
            log_channel_id: None,
            starboard_channel_id: None,
            starboard_threshold: default_starboard_threshold(),
            terminal_channel_id: None,
            ai_enabled: false,
            ai_channel_id: None,
            ai_model: default_ai_model(),
        }
    }
}

pub struct Data {
    pub terminal_whitelist: Arc<RwLock<Vec<String>>>,
    pub approval_channels: Arc<RwLock<HashMap<String, String>>>,
    pub flags: Arc<RwLock<HashMap<String, Vec<String>>>>,
    pub reaction_roles: Arc<RwLock<HashMap<String, Value>>>,
    pub sticky_messages: Arc<RwLock<HashMap<String, StickyMessage>>>,
    pub awaiting_sticky: Arc<RwLock<HashMap<String, String>>>,
    pub starboarded: Arc<RwLock<HashMap<String, bool>>>,
    pub media_requests: Arc<RwLock<HashMap<String, MediaRequest>>>,
    pub terminal_flag_drafts: Arc<RwLock<HashMap<String, TerminalFlagDraft>>>,
    pub guild_settings: Arc<RwLock<HashMap<String, GuildSettings>>>,
    pub data_dir: PathBuf,
}

impl Data {
    pub fn new() -> Self {
        let data_dir = prepare_data_dir();

        let terminal_whitelist = load_json_or_default(
            &data_dir.join("terminal_whitelist.json"),
            vec!["1117969014698811593".to_string()],
        );
        let approval_channels =
            load_json_or_default(&data_dir.join("approval_channels.json"), HashMap::new());
        let flags = load_json_or_default(&data_dir.join("flags.json"), HashMap::new());
        let reaction_roles =
            load_json_or_default(&data_dir.join("reaction_roles.json"), HashMap::new());
        let sticky_messages =
            load_json_or_default(&data_dir.join("sticky_messages.json"), HashMap::new());
        let starboarded = load_json_or_default(&data_dir.join("starboard.json"), HashMap::new());
        let media_requests =
            load_json_or_default(&data_dir.join("media_requests.json"), HashMap::new());
        let guild_settings = load_guild_settings(&data_dir);

        Self {
            terminal_whitelist: Arc::new(RwLock::new(terminal_whitelist)),
            approval_channels: Arc::new(RwLock::new(approval_channels)),
            flags: Arc::new(RwLock::new(flags)),
            reaction_roles: Arc::new(RwLock::new(reaction_roles)),
            sticky_messages: Arc::new(RwLock::new(sticky_messages)),
            awaiting_sticky: Arc::new(RwLock::new(HashMap::new())),
            starboarded: Arc::new(RwLock::new(starboarded)),
            media_requests: Arc::new(RwLock::new(media_requests)),
            terminal_flag_drafts: Arc::new(RwLock::new(HashMap::new())),
            guild_settings: Arc::new(RwLock::new(guild_settings)),
            data_dir,
        }
    }

    pub async fn save_whitelist(&self) {
        save_json(
            self.data_dir.join("terminal_whitelist.json"),
            &*self.terminal_whitelist.read().await,
        );
    }

    pub async fn save_approval_channels(&self) {
        save_json(
            self.data_dir.join("approval_channels.json"),
            &*self.approval_channels.read().await,
        );
    }

    pub async fn save_flags(&self) {
        save_json(self.data_dir.join("flags.json"), &*self.flags.read().await);
    }

    pub async fn save_reaction_roles(&self) {
        save_json(
            self.data_dir.join("reaction_roles.json"),
            &*self.reaction_roles.read().await,
        );
    }

    pub async fn save_sticky_messages(&self) {
        save_json(
            self.data_dir.join("sticky_messages.json"),
            &*self.sticky_messages.read().await,
        );
    }

    pub async fn save_starboard(&self) {
        save_json(
            self.data_dir.join("starboard.json"),
            &*self.starboarded.read().await,
        );
    }

    pub async fn save_media_requests(&self) {
        save_json(
            self.data_dir.join("media_requests.json"),
            &*self.media_requests.read().await,
        );
    }

    pub async fn save_guild_settings(&self) {
        save_json(
            self.data_dir.join("guild_settings.json"),
            &*self.guild_settings.read().await,
        );
    }
}

fn prepare_data_dir() -> PathBuf {
    let mut data_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    data_dir.pop();
    data_dir.push("data");
    fs::create_dir_all(&data_dir).ok();
    data_dir
}

fn load_json_or_default<T>(path: &PathBuf, default: T) -> T
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    if path.exists() {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(parsed) = serde_json::from_str(&contents) {
                return parsed;
            }
        }
    }

    save_json(path.clone(), &default);
    default
}

fn save_json<T>(path: PathBuf, value: &T)
where
    T: Serialize,
{
    if let Ok(serialized) = serde_json::to_string_pretty(value) {
        let _ = fs::write(path, serialized);
    }
}

fn load_guild_settings(data_dir: &PathBuf) -> HashMap<String, GuildSettings> {
    let settings_path = data_dir.join("guild_settings.json");
    let mut settings: HashMap<String, GuildSettings> =
        load_json_or_default(&settings_path, HashMap::new());

    let legacy_languages: HashMap<String, String> =
        load_json_or_default(&data_dir.join("guild_languages.json"), HashMap::new());
    for (guild_id, language) in legacy_languages {
        settings.entry(guild_id).or_default().language = language;
    }

    save_json(settings_path, &settings);
    settings
}
