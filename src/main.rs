#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]

use std::{sync::Arc, time::Duration};

use eyre::{eyre, Context as _, Report, Result};
use log::{info, trace, warn};

use octocrab::Octocrab;
use poise::{
	serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use owo_colors::OwoColorize;
use redis::ConnectionLike;

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
	storage: Storage,
	octocrab: Arc<octocrab::Octocrab>,
}

impl Data {
	pub fn new(config: Config, storage: Storage, octocrab: Arc<Octocrab>) -> Result<Self> {
		Ok(Self {
			config,
			storage,
			octocrab,
		})
	}
}

async fn setup(
	ctx: &serenity::Context,
	_: &serenity::Ready,
	framework: &Framework<Data, Report>,
) -> Result<Data> {
	let config = Config::new_from_env();
	let storage = Storage::from_url(&config.redis_url)?;
	let octocrab = octocrab::instance();
	let data = Data::new(config, storage, octocrab)?;

	// test redis connection
	let mut client = data.storage.client().clone();

	if !client.check_connection() {
		return Err(eyre!(
			"Couldn't connect to storage! Is your daemon running?"
		));
	}
	trace!("Redis connection looks good!");

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
