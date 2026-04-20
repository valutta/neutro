mod ai;
mod bot;
mod commands;
mod events;
mod i18n;
mod infra;
mod state;
pub mod v2_components;

use poise::serenity_prelude as serenity;
use state::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tokio::spawn(async {
        infra::spawn_http_stub().await;
    });

    let token = std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN");

    // Non-privileged intents plus message content for prefix commands and chat monitoring
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::moderation::kick(),
                commands::moderation::ban(),
                commands::moderation::mute(),
                commands::moderation::tempmute(),
                commands::moderation::unmute(),
                commands::moderation::clear(),
                commands::moderation::role(),
                commands::moderation::announce(),
                commands::moderation::eannounce(),
                commands::moderation::stream(),
                commands::server::sticky(),
                commands::server::dsticky(),
                commands::server::ar(),
                commands::server::pvoice(),
                commands::utility::ping(),
                commands::utility::rid(),
                commands::utility::id(),
                commands::utility::avatar(),
                commands::utility::profile(),
                commands::utility::aquote(),
                commands::utility::help(),
                commands::ai::ask(),
                commands::settings::settings(),
                commands::terminal::terminal(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![poise::Prefix::Literal("$")],
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handle_event(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register slash commands globally
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Neutrobot Discord bot started and commands registered!");
                let data = Data::new();

                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        eprintln!("Client error: {}", e);
    }
}
