use crate::{Context, Error};
use serde_json::json;

pub async fn send_v2(ctx: Context<'_>, components: serde_json::Value) -> Result<(), Error> {
    let map = json!({
        "flags": 1 << 15, // IS_COMPONENTS_V2
        "components": components,
    });

    match ctx {
        poise::Context::Application(app_ctx) => {
            app_ctx
                .serenity_context
                .http
                .create_interaction_response(
                    app_ctx.interaction.id,
                    &app_ctx.interaction.token,
                    &json!({
                        "type": 4, // ChannelMessageWithSource
                        "data": map
                    }),
                    vec![],
                )
                .await?;
        }
        poise::Context::Prefix(prefix_ctx) => {
            let channel_id = prefix_ctx.msg.channel_id;
            prefix_ctx
                .serenity_context
                .http
                .send_message(channel_id, vec![], &map)
                .await?;
        }
    }

    Ok(())
}
