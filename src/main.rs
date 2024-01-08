use std::sync::Arc;
use std::time::Duration;

use color_eyre::eyre::{eyre, Context as _, Report, Result};
use color_eyre::owo_colors::OwoColorize;

use log::*;

use poise::{
	serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use serenity::ShardManager;

use redis::ConnectionLike;

use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Mutex;

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
	let data = Data::new()?;

	// test redis connection
	let mut client = data.storage.client.clone();

	if !client.check_connection() {
		return Err(eyre!(
			"Couldn't connect to storage! Is your daemon running?"
		));
	}

	poise::builtins::register_globally(ctx, &framework.options().commands).await?;
	info!("Registered global commands!");

	Ok(data)
}

async fn handle_shutdown(shard_manager: Arc<Mutex<ShardManager>>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.lock().await.shutdown_all().await;
	println!("{}", "Everything is shutdown. Goodbye!".green())
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
		.setup(|ctx, ready, framework| Box::pin(setup(ctx, ready, framework)))
		.build()
		.await
		.wrap_err_with(|| "Failed to build framework!")?;

	let shard_manager = framework.shard_manager().clone();
	let mut sigterm = signal(SignalKind::terminate())?;

	tokio::select! {
		result = framework.start() => result.map_err(Report::from),
		_ = sigterm.recv() => {
			handle_shutdown(shard_manager, "Recieved SIGTERM").await;
			std::process::exit(0);
		}
		_ = ctrl_c() => {
			handle_shutdown(shard_manager, "Interrupted").await;
			std::process::exit(130);
		}
	}
}
