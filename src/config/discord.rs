use std::str::FromStr;

use log::{info, warn};
use poise::serenity_prelude::ChannelId;

#[derive(Clone, Copy, Debug, Default)]
pub struct RefractionChannels {
	log_channel_id: Option<ChannelId>,
	welcome_channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
	channels: RefractionChannels,
}

impl RefractionChannels {
	pub fn new(log_channel_id: Option<ChannelId>, welcome_channel_id: Option<ChannelId>) -> Self {
		Self {
			log_channel_id,
			welcome_channel_id,
		}
	}

	pub fn new_from_env() -> Self {
		let log_channel_id = Self::get_channel_from_env("DISCORD_LOG_CHANNEL_ID");
		if let Some(channel_id) = log_channel_id {
			info!("Log channel is {channel_id}");
		} else {
			warn!("DISCORD_LOG_CHANNEL_ID is empty; this will disable logging in your server.");
		}

		let welcome_channel_id = Self::get_channel_from_env("DISCORD_WELCOME_CHANNEL_ID");
		if let Some(channel_id) = welcome_channel_id {
			info!("Welcome channel is {channel_id}");
		} else {
			warn!("DISCORD_WELCOME_CHANNEL_ID is empty; this will disable welcome channel features in your server");
		}

		Self::new(log_channel_id, welcome_channel_id)
	}

	fn get_channel_from_env(var: &str) -> Option<ChannelId> {
		std::env::var(var)
			.ok()
			.and_then(|env_var| ChannelId::from_str(&env_var).ok())
	}

	pub fn log_channel_id(self) -> Option<ChannelId> {
		self.log_channel_id
	}

	pub fn welcome_channel_id(self) -> Option<ChannelId> {
		self.welcome_channel_id
	}
}

impl Config {
	pub fn new(channels: RefractionChannels) -> Self {
		Self { channels }
	}

	pub fn new_from_env() -> Self {
		let channels = RefractionChannels::new_from_env();

		Self::new(channels)
	}

	pub fn channels(self) -> RefractionChannels {
		self.channels
	}
}
