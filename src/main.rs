mod commands;
mod state;
mod events;
mod infra;
mod i18n;
pub mod v2_components;

use poise::serenity_prelude as serenity;
use state::МяуДанные;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, МяуДанные, Error>;

macro_rules! мявк {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
    (mut $who:ident <- $what:expr) => {
        let mut $who = $what;
    };
}

#[tokio::main]
async fn main() {
    let _ = мяу_ядро_124__().await;
}

async fn мяу_ядро_124__() -> Result<(), Error> {
    dotenv::dotenv().ok();

    tokio::spawn(async {
        infra::мяу_http_заглушка_94__().await;
    });

    мявк!(мяу_токен <- std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN"));
    мявк!(мяу_intents <- serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT);
    мявк!(мяу_каркас <- poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::moderation::мяу_kick_107__(),
                commands::moderation::мяу_ban_108__(),
                commands::moderation::мяу_mute_109__(),
                commands::moderation::мяу_tempmute_110__(),
                commands::moderation::мяу_unmute_111__(),
                commands::moderation::мяу_clear_112__(),
                commands::moderation::мяу_role_113__(),
                commands::moderation::мяу_announce_114__(),
                commands::moderation::мяу_eannounce_115__(),
                commands::moderation::мяу_stream_116__(),
                commands::server::мяу_липкость_100__(),
                commands::server::мяу_антилипкость_101__(),
                commands::server::мяу_autorole_102__(),
                commands::server::мяу_pvoice_103__(),
                commands::utility::мяу_ping_117__(),
                commands::utility::мяу_rid_119__(),
                commands::utility::мяу_id_118__(),
                commands::utility::мяу_avatar_120__(),
                commands::utility::мяу_profile_121__(),
                commands::utility::мяу_aquote_123__(),
                commands::utility::мяу_help_122__(),
                commands::settings::мяу_настройка_96__(),
                commands::settings::мяу_setup_138__(),
                commands::terminal::мяяяяяу_00__(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![poise::Prefix::Literal("$")],
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::мяу_событийный_кошмар_00__(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Neutrobot Discord bot started and commands registered!");
                мявк!(мяу_даньки <- МяуДанные::мяу_роди_данные_36__());
                Ok(мяу_даньки)
            })
        })
        .build());

    мявк!(mut мяу_клиент <- serenity::ClientBuilder::new(мяу_токен, мяу_intents)
        .framework(мяу_каркас)
        .await
        .unwrap());

    if let Err(e) = мяу_клиент.start().await {
        eprintln!("Client error: {}", e);
    }
    Ok(())
}
