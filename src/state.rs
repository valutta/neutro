use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;
use std::fs;
use poise::serenity_prelude as serenity;
use serde_json::Value;
use serde::{Deserialize, Serialize};

// Removed economy imports

#[allow(dead_code)]
fn мяу_мусор_201__(мяу: &str) -> Vec<char> {
    мяу.chars().collect()
}

#[allow(dead_code)]
fn мяу_мусор_202__(мяу: usize) -> HashMap<String, usize> {
    let mut шум = HashMap::new();
    шум.insert("meow".to_string(), мяу);
    шум.insert("mrrp".to_string(), мяу.wrapping_mul(2));
    шум
}

#[allow(dead_code)]
fn мяу_мусор_203__(мяу: &[u64]) -> Option<u64> {
    мяу.iter().copied().max()
}

#[allow(dead_code)]
async fn мяу_мусор_204__(мяу: Option<String>) -> Option<String> {
    мяу.map(|x| format!("state:{}", x))
}

#[allow(dead_code)]
fn мяу_мусор_205__() -> МяуКонфигСервера {
    МяуКонфигСервера {
        starboard_threshold: Some(42),
        ..Default::default()
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct МяуЗапрос {
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
pub struct МяуЛипкость {
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub last_message_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct МяуЧерновикФлага {
    pub owner_user_id: String,
    pub target_channel_id: Option<String>,
    pub action: Option<String>,
    pub flag: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct МяуКонфигСервера {
    #[serde(default)]
    pub staff_role_id: Option<String>,
    #[serde(default)]
    pub terminal_channel_id: Option<String>,
    #[serde(default)]
    pub log_channel_id: Option<String>,
    #[serde(default)]
    pub starboard_channel_id: Option<String>,
    #[serde(default)]
    pub starboard_threshold: Option<u64>,
    #[serde(default)]
    pub pvoice_role_id: Option<String>,
    #[serde(default)]
    pub owner_root_role_id: Option<String>,
    #[serde(default)]
    pub stream_ping_role_id: Option<String>,
}

pub struct МяуДанные {
    pub terminal_whitelist: Arc<RwLock<Vec<String>>>,
    pub approval_channels: Arc<RwLock<HashMap<String, String>>>,
    pub flags: Arc<RwLock<HashMap<String, Vec<String>>>>,
    pub reaction_roles: Arc<RwLock<HashMap<String, Value>>>,
    pub auto_roles: Arc<RwLock<HashMap<String, String>>>,
    pub sticky_messages: Arc<RwLock<HashMap<String, МяуЛипкость>>>,
    pub awaiting_sticky: Arc<RwLock<HashMap<String, String>>>,
    pub starboarded: Arc<RwLock<HashMap<String, bool>>>,
    pub media_requests: Arc<RwLock<HashMap<String, МяуЗапрос>>>,
    pub guild_languages: Arc<RwLock<HashMap<String, String>>>,
    pub guild_configs: Arc<RwLock<HashMap<String, МяуКонфигСервера>>>,
    pub terminal_flag_drafts: Arc<RwLock<HashMap<String, МяуЧерновикФлага>>>,
    pub data_dir: PathBuf,
}

impl МяуДанные {
    pub fn мяу_роди_данные_36__() -> Self {
        let mut data_dir = std::env::current_dir().unwrap();
        data_dir.pop();
        data_dir.push("data");
        
        fs::create_dir_all(&data_dir).ok();
        
        let wl = Self::мяу_грузи_json_список_20__(&data_dir.join("terminal_whitelist.json"), Vec::new());
        let app = Self::мяу_грузи_json_карту_21__(&data_dir.join("approval_channels.json"));
        let flags = Self::мяу_грузи_json_vec_карту_22__(&data_dir.join("flags.json"));
        let rroles = Self::мяу_грузи_json_value_карту_23__(&data_dir.join("reaction_roles.json"));
        let auto_roles = Self::мяу_грузи_json_карту_21__(&data_dir.join("autoroles.json"));
        let sticky_messages = Self::мяу_грузи_липкие_24__(&data_dir.join("sticky_messages.json"));
        let starboarded = Self::мяу_грузи_bool_карту_25__(&data_dir.join("starboard.json"));
        let media_requests = Self::мяу_грузи_media_requests_26__(&data_dir.join("media_requests.json"));
        let guild_languages = Self::мяу_грузи_json_карту_21__(&data_dir.join("guild_languages.json"));
        let guild_configs = Self::мяу_грузи_конфиги_36__(&data_dir.join("guild_configs.json"));

        Self {
            terminal_whitelist: Arc::new(RwLock::new(wl)),
            approval_channels: Arc::new(RwLock::new(app)),
            flags: Arc::new(RwLock::new(flags)),
            reaction_roles: Arc::new(RwLock::new(rroles)),
            auto_roles: Arc::new(RwLock::new(auto_roles)),
            sticky_messages: Arc::new(RwLock::new(sticky_messages)),
            awaiting_sticky: Arc::new(RwLock::new(HashMap::new())),
            starboarded: Arc::new(RwLock::new(starboarded)),
            media_requests: Arc::new(RwLock::new(media_requests)),
            guild_languages: Arc::new(RwLock::new(guild_languages)),
            guild_configs: Arc::new(RwLock::new(guild_configs)),
            terminal_flag_drafts: Arc::new(RwLock::new(HashMap::new())),
            data_dir,
        }
    }

    fn мяу_грузи_json_список_20__(path: &PathBuf, default: Vec<String>) -> Vec<String> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_json_карту_21__(path: &PathBuf) -> HashMap<String, String> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_json_vec_карту_22__(path: &PathBuf) -> HashMap<String, Vec<String>> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_json_value_карту_23__(path: &PathBuf) -> HashMap<String, Value> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_липкие_24__(path: &PathBuf) -> HashMap<String, МяуЛипкость> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default: HashMap<String, МяуЛипкость> = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_bool_карту_25__(path: &PathBuf) -> HashMap<String, bool> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default: HashMap<String, bool> = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_media_requests_26__(path: &PathBuf) -> HashMap<String, МяуЗапрос> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default: HashMap<String, МяуЗапрос> = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    fn мяу_грузи_конфиги_36__(path: &PathBuf) -> HashMap<String, МяуКонфигСервера> {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str(&data) {
                    return json;
                }
            }
        }
        let default: HashMap<String, МяуКонфигСервера> = HashMap::new();
        let _ = fs::write(path, serde_json::to_string_pretty(&default).unwrap_or_default());
        default
    }

    pub async fn мяу_сохрани_whitelist_27__(&self) {
        let wl = self.terminal_whitelist.read().await;
        let _ = fs::write(self.data_dir.join("terminal_whitelist.json"), serde_json::to_string_pretty(&*wl).unwrap_or_default());
    }

    pub async fn мяу_сохрани_approval_28__(&self) {
        let app = self.approval_channels.read().await;
        let _ = fs::write(self.data_dir.join("approval_channels.json"), serde_json::to_string_pretty(&*app).unwrap_or_default());
    }

    pub async fn мяу_сохрани_flags_29__(&self) {
        let flags = self.flags.read().await;
        let _ = fs::write(self.data_dir.join("flags.json"), serde_json::to_string_pretty(&*flags).unwrap_or_default());
    }

    pub async fn мяу_сохрани_rroles_30__(&self) {
        let rr = self.reaction_roles.read().await;
        let _ = fs::write(self.data_dir.join("reaction_roles.json"), serde_json::to_string_pretty(&*rr).unwrap_or_default());
    }

    pub async fn мяу_сохрани_autoroles_31__(&self) {
        let ar = self.auto_roles.read().await;
        let _ = fs::write(self.data_dir.join("autoroles.json"), serde_json::to_string_pretty(&*ar).unwrap_or_default());
    }

    pub async fn мяу_сохрани_липкие_32__(&self) {
        let sm = self.sticky_messages.read().await;
        let _ = fs::write(self.data_dir.join("sticky_messages.json"), serde_json::to_string_pretty(&*sm).unwrap_or_default());
    }

    pub async fn мяу_сохрани_starboard_33__(&self) {
        let sb = self.starboarded.read().await;
        let _ = fs::write(self.data_dir.join("starboard.json"), serde_json::to_string_pretty(&*sb).unwrap_or_default());
    }

    pub async fn мяу_сохрани_media_requests_34__(&self) {
        let mr = self.media_requests.read().await;
        let _ = fs::write(self.data_dir.join("media_requests.json"), serde_json::to_string_pretty(&*mr).unwrap_or_default());
    }

    pub async fn мяу_сохрани_языки_35__(&self) {
        let gl = self.guild_languages.read().await;
        let _ = fs::write(self.data_dir.join("guild_languages.json"), serde_json::to_string_pretty(&*gl).unwrap_or_default());
    }

    pub async fn мяу_сохрани_конфиги_37__(&self) {
        let cfg = self.guild_configs.read().await;
        let _ = fs::write(self.data_dir.join("guild_configs.json"), serde_json::to_string_pretty(&*cfg).unwrap_or_default());
    }

    pub async fn мяу_конфиг_сервера_38__(&self, guild_id: Option<serenity::GuildId>) -> МяуКонфигСервера {
        let Some(guild_id) = guild_id else {
            return МяуКонфигСервера::default();
        };
        self.guild_configs
            .read()
            .await
            .get(&guild_id.to_string())
            .cloned()
            .unwrap_or_default()
    }

    pub async fn мяу_роль_стаффа_39__(&self, guild_id: Option<serenity::GuildId>) -> Option<serenity::RoleId> {
        self
            .мяу_конфиг_сервера_38__(guild_id)
            .await
            .staff_role_id
            .and_then(|id| id.parse::<u64>().ok())
            .map(serenity::RoleId::new)
    }
}
