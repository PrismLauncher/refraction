use std::sync::Arc;

use eyre::Context as _;
use log::{info, trace, warn};
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
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

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone, Debug, Default)]
pub struct Data {
	config: Config,
	storage: Option<Storage>,
	http_client: api::HttpClient,
	octocrab: Arc<octocrab::Octocrab>,
}

async fn setup(
	ctx: &serenity::Context,
	_: &serenity::Ready,
	framework: &Framework<Data, Error>,
) -> Result<Data, Error> {
	let config = Config::new_from_env();

	let storage = if let Some(url) = &config.bot.redis_url {
		Some(Storage::from_url(url)?)
	} else {
		None
	};

	if let Some(storage) = storage.as_ref() {
		if !storage.clone().has_connection() {
			return Err(
				"You specified a storage backend but there's no connection! Is it running?".into(),
			);
		}

		trace!("Redis connection looks good!");
	}

	let http_client = api::HttpClient::default();
	let octocrab = octocrab::instance();

	let data = Data {
		config,
		storage,
		http_client,
		octocrab,
	};

	poise::builtins::register_globally(ctx, &framework.options().commands).await?;
	info!("Registered global commands!");

	Ok(data)
}

async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("Everything is shutdown. Goodbye!");
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let token =
		std::env::var("DISCORD_BOT_TOKEN").wrap_err("Couldn't find bot token in environment!")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: commands::all(),

		on_error: |error| Box::pin(handlers::handle_error(error)),

		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),

		event_handler: |ctx, event, framework, data| {
			Box::pin(handlers::handle_event(ctx, event, framework, data))
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
		result = client.start() => result.map_err(eyre::Report::from),
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
