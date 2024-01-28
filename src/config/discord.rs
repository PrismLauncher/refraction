use std::str::FromStr;

use log::{info, warn};
use poise::serenity_prelude::ChannelId;

#[derive(Debug, Clone, Default)]
pub struct RefractionChannels {
	pub say_log_channel_id: Option<ChannelId>,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
	pub channels: RefractionChannels,
}

impl RefractionChannels {
	pub fn new_from_env() -> Self {
		let say_log_channel_id = Self::get_channel_from_env("DISCORD_SAY_LOG_CHANNELID");

		if let Some(channel_id) = say_log_channel_id {
			info!("Log channel is {channel_id}");
		} else {
			warn!("DISCORD_SAY_LOG_CHANNELID is empty; this will disable logging in your server.");
		}

		Self { say_log_channel_id }
	}

	fn get_channel_from_env(var: &str) -> Option<ChannelId> {
		std::env::var(var)
			.ok()
			.and_then(|env_var| ChannelId::from_str(&env_var).ok())
	}
}

impl Config {
	pub fn new_from_env() -> Self {
		let channels = RefractionChannels::new_from_env();

		Self { channels }
	}
}
