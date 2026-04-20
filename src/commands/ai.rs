use crate::ai::{VertexClient, current_model_summary, load_prompt_images};
use crate::bot::{guild_settings, has_staff_access, reply_text};
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

// Prefix usage still exists for admins, but normal users can just tag the bot.
#[poise::command(slash_command, prefix_command, rename = "ask")]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Prompt for the AI"] prompt: String,
    #[description = "Optional image for the AI to inspect"] image: Option<serenity::Attachment>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return reply_text(ctx, "This command can only be used in a server.").await;
    };

    let settings = guild_settings(ctx.data(), guild_id).await;
    if !settings.ai_enabled {
        return reply_text(
            ctx,
            "AI is disabled for this server. Configure it with `!settings ai true`.",
        )
        .await;
    }

    if let Some(channel_id) = settings.ai_channel_id {
        let in_allowed_channel = ctx.channel_id() == serenity::ChannelId::new(channel_id);
        if !in_allowed_channel && !has_staff_access(&ctx).await {
            return reply_text(
                ctx,
                format!("AI commands are restricted to <#{channel_id}> on this server."),
            )
            .await;
        }
    }

    let prompt = prompt.trim();
    if prompt.is_empty() {
        return reply_text(ctx, "Prompt cannot be empty.").await;
    }
    let prompt = format!("{}: {}", ctx.author().name, prompt);

    let client = VertexClient::from_env()?;
    let images = match image {
        Some(image) => load_prompt_images(&[image]).await,
        None => Vec::new(),
    };

    match client
        .generate_text(&settings.ai_model, &prompt, &images)
        .await
    {
        Ok(answer) => reply_text(ctx, answer.text).await,
        Err(error) => reply_text(ctx, format!("Vertex AI request failed: {error}")).await,
    }
}

pub fn ai_mode_summary(mode: &str) -> String {
    current_model_summary(mode)
}
