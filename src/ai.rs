use crate::Error;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const GOOGLE_CLOUD_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
const FLASH_MODEL: &str = "gemini-2.5-flash";
const PRO_MODEL: &str = "gemini-2.5-pro";
const IMAGE_MODEL: &str = "gemini-2.5-flash-image";
const DEFAULT_PROMPT: &str = "You are Neutro, a Discord bot assistant. Keep answers readable, direct, and useful. Do not claim actions you did not take. Avoid roleplay. Never ask users to expose secrets or tokens.";
const MAX_IMAGES_PER_PROMPT: usize = 4;

pub struct VertexClient {
    http: Client,
    service_account: ServiceAccountKey,
    location: String,
}

pub struct AiResponse {
    pub text: String,
}

pub struct PromptImage {
    mime_type: String,
    data_base64: String,
}

#[derive(Debug, Deserialize)]
struct ServiceAccountKey {
    project_id: String,
    private_key: String,
    client_email: String,
    token_uri: String,
}

#[derive(Debug, Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
}

impl VertexClient {
    pub fn from_env() -> Result<Self, Error> {
        let key_path = std::env::var("VERTEX_SERVICE_ACCOUNT_PATH")
            .unwrap_or_else(|_| "vertex_key.json".to_string());
        let location =
            std::env::var("VERTEX_LOCATION").unwrap_or_else(|_| "us-central1".to_string());
        let service_account: ServiceAccountKey =
            serde_json::from_str(&fs::read_to_string(key_path)?)?;

        Ok(Self {
            http: Client::new(),
            service_account,
            location,
        })
    }

    pub async fn generate_text(
        &self,
        model_preference: &str,
        prompt: &str,
        images: &[PromptImage],
    ) -> Result<AiResponse, Error> {
        let access_token = self.fetch_access_token().await?;
        let model = pick_chat_model(model_preference, prompt);
        let endpoint = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            self.location, self.service_account.project_id, self.location, model
        );
        let system_prompt = load_system_prompt();

        let mut request_body = serde_json::json!({
            "systemInstruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": build_contents(prompt, images),
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 1024
            }
        });

        request_body["tools"] = serde_json::json!([
            {
                "googleSearch": {}
            }
        ]);

        let response = self
            .http
            .post(endpoint)
            .bearer_auth(access_token)
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let body: Value = response.json().await?;
        if !status.is_success() {
            let message = body
                .get("error")
                .and_then(|error| error.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("Vertex AI request failed");
            return Err(message.to_string().into());
        }

        let text = extract_text(&body).ok_or_else(|| "Vertex AI returned no text".to_string())?;
        Ok(AiResponse {
            text: sanitize_discord_mentions(&text),
        })
    }

    async fn fetch_access_token(&self) -> Result<String, Error> {
        let now = unix_timestamp();
        let claims = JwtClaims {
            iss: &self.service_account.client_email,
            scope: GOOGLE_CLOUD_SCOPE,
            aud: &self.service_account.token_uri,
            iat: now,
            exp: now + Duration::from_secs(3600).as_secs() as usize,
        };
        let assertion = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())?,
        )?;

        let response = self
            .http
            .post(&self.service_account.token_uri)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type={}&assertion={}",
                url_encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
                url_encode(&assertion)
            ))
            .send()
            .await?;

        let status = response.status();
        let body: Value = response.json().await?;
        if !status.is_success() {
            let message = body
                .get("error_description")
                .and_then(Value::as_str)
                .or_else(|| body.get("error").and_then(Value::as_str))
                .unwrap_or("Failed to fetch a Google access token");
            return Err(message.to_string().into());
        }

        let token: OAuthTokenResponse = serde_json::from_value(body)?;
        Ok(token.access_token)
    }
}

pub async fn load_prompt_images(attachments: &[serenity::Attachment]) -> Vec<PromptImage> {
    let http = Client::new();
    let mut images = Vec::new();

    for attachment in attachments.iter().take(MAX_IMAGES_PER_PROMPT) {
        let Some(mime_type) = attachment
            .content_type
            .as_deref()
            .filter(|mime| mime.starts_with("image/"))
        else {
            continue;
        };

        let Ok(response) = http.get(&attachment.url).send().await else {
            continue;
        };
        let Ok(bytes) = response.bytes().await else {
            continue;
        };

        images.push(PromptImage {
            mime_type: mime_type.to_string(),
            data_base64: BASE64.encode(bytes),
        });
    }

    images
}

pub fn current_model_summary(model_preference: &str) -> String {
    match normalize_model_preference(model_preference) {
        "flash" => FLASH_MODEL.to_string(),
        "pro" => PRO_MODEL.to_string(),
        "image" => format!(
            "vision-ready chat on `{FLASH_MODEL}` with image tool `{IMAGE_MODEL}` available"
        ),
        _ => format!("adaptive (`{FLASH_MODEL}` or `{PRO_MODEL}`)"),
    }
}

pub fn sanitize_discord_mentions(input: &str) -> String {
    input
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
        .replace("<@&", "<@&\u{200B}")
        .replace("<@", "<@\u{200B}")
}

pub fn strip_bot_mention(content: &str, bot_user_id: u64) -> String {
    let plain = format!("<@{}>", bot_user_id);
    let nick = format!("<@!{}>", bot_user_id);
    content
        .replace(&plain, "")
        .replace(&nick, "")
        .trim()
        .to_string()
}

fn pick_chat_model(model_preference: &str, prompt: &str) -> &'static str {
    match normalize_model_preference(model_preference) {
        "flash" => FLASH_MODEL,
        "pro" => PRO_MODEL,
        _ => {
            if should_use_pro(prompt) {
                PRO_MODEL
            } else {
                FLASH_MODEL
            }
        }
    }
}

fn normalize_model_preference(value: &str) -> &str {
    match value.trim().to_lowercase().as_str() {
        "flash" | "gemini-2.5-flash" => "flash",
        "pro" | "gemini-2.5-pro" => "pro",
        "image" | "gemini-2.5-flash-image" => "image",
        _ => "adaptive",
    }
}

fn should_use_pro(prompt: &str) -> bool {
    let prompt = prompt.trim();
    let lowercase = prompt.to_lowercase();
    let complex_keywords = [
        "architecture",
        "analyze",
        "analysis",
        "compare",
        "debug",
        "design",
        "explain in detail",
        "refactor",
        "strategy",
        "tradeoff",
        "why",
    ];

    prompt.len() > 900
        || prompt.lines().count() > 12
        || prompt.contains("```")
        || complex_keywords
            .iter()
            .any(|keyword| lowercase.contains(keyword))
}

fn build_contents(prompt: &str, images: &[PromptImage]) -> Value {
    let mut parts = Vec::with_capacity(images.len() + 1);
    parts.push(serde_json::json!({ "text": prompt }));

    for image in images {
        parts.push(serde_json::json!({
            "inlineData": {
                "mimeType": image.mime_type,
                "data": image.data_base64
            }
        }));
    }

    serde_json::json!([
        {
            "role": "user",
            "parts": parts
        }
    ])
}

fn load_system_prompt() -> String {
    let path = std::env::var("AI_PROMPT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("prompts/ai_personality.txt"));

    fs::read_to_string(path)
        .map(|text| text.trim().to_string())
        .ok()
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| DEFAULT_PROMPT.to_string())
}

fn extract_text(body: &Value) -> Option<String> {
    let parts = body
        .get("candidates")?
        .as_array()?
        .first()?
        .get("content")?
        .get("parts")?
        .as_array()?;

    let text = parts
        .iter()
        .filter_map(|part| part.get("text").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join("\n");

    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

fn unix_timestamp() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize
}

fn url_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}
