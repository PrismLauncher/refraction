use std::str::FromStr;

use eyre::{Context, Result};
use log::{info, warn};
use poise::serenity_prelude::ChannelId;

#[derive(Clone, Copy, Debug, Default)]
pub struct RefractionChannels {
	pub log_channel_id: Option<ChannelId>,
	pub welcome_channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
	pub channels: RefractionChannels,
	pub days_to_delete_reaction: i64,
}

impl RefractionChannels {
	pub fn new(log_channel_id: Option<ChannelId>, welcome_channel_id: Option<ChannelId>) -> Self {
		Self {
			log_channel_id,
			welcome_channel_id,
		}
	}

	pub fn from_env() -> Self {
		let log_channel_id = Self::channel_from_env("DISCORD_LOG_CHANNEL_ID");
		if let Some(channel_id) = log_channel_id {
			info!("Log channel is {channel_id}");
		} else {
			warn!("DISCORD_LOG_CHANNEL_ID is empty; this will disable logging in your server.");
		}

		let welcome_channel_id = Self::channel_from_env("DISCORD_WELCOME_CHANNEL_ID");
		if let Some(channel_id) = welcome_channel_id {
			info!("Welcome channel is {channel_id}");
		} else {
			warn!("DISCORD_WELCOME_CHANNEL_ID is empty; this will disable welcome channel features in your server");
		}

		Self::new(log_channel_id, welcome_channel_id)
	}

	fn channel_from_env(var: &str) -> Option<ChannelId> {
		std::env::var(var)
			.ok()
			.and_then(|env_var| ChannelId::from_str(&env_var).ok())
	}
}

impl Config {
	pub fn new(channels: RefractionChannels, days_to_delete_reaction: i64) -> Self {
		Self {
			channels,
			days_to_delete_reaction,
		}
	}

	pub fn from_env() -> Result<Self> {
		let channels = RefractionChannels::from_env();
		let days_to_delete_reaction = std::env::var("DISCORD_DAYS_TO_DELETE_REACTION")
			.wrap_err("DISCORD_DAYS_TO_DELETE_REACTION is empty! This variable is required.")?
			.parse()
			.wrap_err("DISCORD_DAYS_TO_DELETE_REACTION is not a number!")?;

		info!("Reactions will be deleted on messages older than {days_to_delete_reaction} days");

		Ok(Self::new(channels, days_to_delete_reaction))
	}
}
