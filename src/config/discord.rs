use std::str::FromStr;

use log::{info, warn};
use poise::serenity_prelude::ChannelId;

#[derive(Clone, Copy, Debug, Default)]
pub struct RefractionChannels {
	say_log_channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
	channels: RefractionChannels,
}

impl RefractionChannels {
	pub fn new(say_log_channel_id: Option<ChannelId>) -> Self {
		Self { say_log_channel_id }
	}

	pub fn new_from_env() -> Self {
		let say_log_channel_id = Self::get_channel_from_env("DISCORD_SAY_LOG_CHANNELID");

		if let Some(channel_id) = say_log_channel_id {
			info!("Log channel is {channel_id}");
		} else {
			warn!("DISCORD_SAY_LOG_CHANNELID is empty; this will disable logging in your server.");
		}

		Self::new(say_log_channel_id)
	}

	fn get_channel_from_env(var: &str) -> Option<ChannelId> {
		std::env::var(var)
			.ok()
			.and_then(|env_var| ChannelId::from_str(&env_var).ok())
	}

	pub fn say_log_channel_id(self) -> Option<ChannelId> {
		self.say_log_channel_id
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
