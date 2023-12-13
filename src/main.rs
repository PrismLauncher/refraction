use std::{sync::Arc, time::Duration};

use color_eyre::eyre::{Context as _, Report, Result};
use config::Config;
use log::*;
use poise::{
    serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use storage::Storage;

mod api;
mod commands;
mod config;
mod consts;
mod handlers;
mod storage;
mod tags;
mod utils;

type Context<'a> = poise::Context<'a, Data, Report>;

#[derive(Clone)]
pub struct Data {
    config: Config,
    storage: Storage,
    octocrab: Arc<octocrab::Octocrab>,
}

impl Data {
    pub fn new() -> Result<Self> {
        let config = Config::new_from_env()?;
        let storage = Storage::new(&config.redis_url)?;
        let octocrab = octocrab::instance();

        Ok(Self {
            config,
            storage,
            octocrab,
        })
    }
}

async fn setup(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    framework: &Framework<Data, Report>,
) -> Result<Data> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    info!("Registered global commands!");

    let data = Data::new()?;

    Ok(data)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install()?;
    env_logger::init();

    let token = std::env::var("DISCORD_BOT_TOKEN")
        .wrap_err_with(|| "Couldn't find bot token in environment!")?;

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let options = FrameworkOptions {
        commands: commands::to_global_commands(),

        on_error: |error| Box::pin(handlers::handle_error(error)),

        command_check: Some(|ctx| {
            Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
        }),

        event_handler: |ctx, event, framework, data| {
            Box::pin(handlers::handle_event(ctx, event, framework, data))
        },

        prefix_options: PrefixFrameworkOptions {
            prefix: Some("r".into()),
            edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },

        ..Default::default()
    };

    let framework = Framework::builder()
        .token(token)
        .intents(intents)
        .options(options)
        .setup(|ctx, ready, framework| Box::pin(setup(ctx, ready, framework)));

    tokio::select! {
        result = framework.run() => { result.map_err(Report::from) },
        _ = tokio::signal::ctrl_c() => {
            info!("Interrupted! Exiting...");
            std::process::exit(130);
        }
    }
}
