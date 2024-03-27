#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]

use std::{sync::Arc, time::Duration};

use eyre::{bail, Context as _, Report, Result};
use log::{info, trace, warn};

use poise::{
	serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use owo_colors::OwoColorize;

use tokio::signal::ctrl_c;
#[cfg(target_family = "unix")]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(target_family = "windows")]
use tokio::signal::windows::ctrl_close;

mod api;
mod commands;
mod config;
mod consts;
mod handlers;
mod storage;
mod tags;
mod utils;

use config::Config;
use storage::Storage;

type Context<'a> = poise::Context<'a, Data, Report>;

#[derive(Clone)]
pub struct Data {
	config: Config,
	storage: Option<Storage>,
}

impl Data {
	#[must_use]
	pub fn new(config: Config, storage: Option<Storage>) -> Self {
		Self { config, storage }
	}
}

async fn setup(
	ctx: &serenity::Context,
	_: &serenity::Ready,
	framework: &Framework<Data, Report>,
) -> Result<Data> {
	let config = Config::new_from_env();

	let storage = if let Some(url) = &config.clone().bot_config().redis_url() {
		Some(Storage::from_url(url)?)
	} else {
		None
	};

	if let Some(storage) = storage.as_ref() {
		if !storage.clone().has_connection() {
			bail!("You specified a storage backend but there's no connection! Is it running?")
		}

		trace!("Redis connection looks good!");
	}

	let data = Data::new(config, storage);

	poise::builtins::register_globally(ctx, &framework.options().commands).await?;
	info!("Registered global commands!");

	Ok(data)
}

async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("{}", "Everything is shutdown. Goodbye!".green());
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let token =
		std::env::var("DISCORD_BOT_TOKEN").wrap_err("Couldn't find bot token in environment!")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: commands::get(),

		on_error: |error| Box::pin(handlers::handle_error(error)),

		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),

		event_handler: |ctx, event, framework, data| {
			Box::pin(handlers::handle_event(ctx, event, framework, data))
		},

		prefix_options: PrefixFrameworkOptions {
			prefix: Some(".".into()),
			edit_tracker: Some(Arc::from(EditTracker::for_timespan(Duration::from_secs(
				3600,
			)))),
			..Default::default()
		},

		..Default::default()
	};

	let framework = Framework::builder()
		.options(options)
		.setup(|ctx, ready, framework| Box::pin(setup(ctx, ready, framework)))
		.build();

	let mut client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await?;

	let shard_manager = client.shard_manager.clone();

	#[cfg(target_family = "unix")]
	let mut sigterm = signal(SignalKind::terminate())?;
	#[cfg(target_family = "windows")]
	let mut sigterm = ctrl_close()?;

	tokio::select! {
		result = client.start() => result.map_err(Report::from),
		_ = sigterm.recv() => {
			handle_shutdown(shard_manager, "Received SIGTERM").await;
			std::process::exit(0);
		}
		_ = ctrl_c() => {
			handle_shutdown(shard_manager, "Interrupted").await;
			std::process::exit(130);
		}
	}
}
