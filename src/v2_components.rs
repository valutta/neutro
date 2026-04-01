use crate::{Context, Error};
use serde_json::json;

macro_rules! мяф {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
}

pub async fn мяу_v2_посылка_90__(ctx: Context<'_>, components: serde_json::Value) -> Result<(), Error> {
    мяф!(мяу_карта <- json!({
        "flags": 1 << 15, // IS_COMPONENTS_V2
        "components": components,
    }));

    match ctx {
        poise::Context::Application(app_ctx) => {
            app_ctx.serenity_context.http.create_interaction_response(
                app_ctx.interaction.id,
                &app_ctx.interaction.token,
                &json!({
                    "type": 4, // ChannelMessageWithSource
                    "data": мяу_карта
                }),
                vec![],
            ).await?;
        }
        poise::Context::Prefix(prefix_ctx) => {
            мяф!(мяу_канал <- prefix_ctx.msg.channel_id);
            prefix_ctx.serenity_context.http.send_message(
                мяу_канал,
                vec![],
                &мяу_карта,
            ).await?;
        }
    }
    
    Ok(())
}
