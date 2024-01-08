use crate::required_var;

use color_eyre::eyre::{Context as _, Result};
use log::*;
use poise::serenity_prelude::{ApplicationId, ChannelId};
use url::Url;

#[derive(Debug, Clone)]
pub struct RefractionOAuth2 {
	pub redirect_uri: Url,
	pub scope: String,
}

#[derive(Debug, Clone, Default)]
pub struct RefractionChannels {
	pub say_log_channel_id: Option<ChannelId>,
}

#[derive(Debug, Clone, Default)]
pub struct DiscordConfig {
	pub client_id: ApplicationId,
	pub client_secret: String,
	pub bot_token: String,
	pub oauth2: RefractionOAuth2,
	pub channels: RefractionChannels,
}

impl Default for RefractionOAuth2 {
	fn default() -> Self {
		Self {
			scope: "identify connections role_connections.write".to_string(),
			redirect_uri: Url::parse("https://google.com").unwrap(),
		}
	}
}

impl RefractionOAuth2 {
	pub fn new_from_env() -> Result<Self> {
		let unparsed = format!("{}/oauth2/callback", required_var!("PUBLIC_URI"));
		let redirect_uri = Url::parse(&unparsed)?;

		debug!("OAuth2 Redirect URI is {redirect_uri}");
		Ok(Self {
			redirect_uri,
			..Default::default()
		})
	}
}

impl RefractionChannels {
	pub fn new_from_env() -> Result<Self> {
		let unparsed = std::env::var("DISCORD_SAY_LOG_CHANNELID");
		if let Ok(unparsed) = unparsed {
			let id = unparsed.parse::<u64>()?;
			let channel_id = ChannelId::from(id);

			debug!("Log channel is {id}");
			Ok(Self {
				say_log_channel_id: Some(channel_id),
			})
		} else {
			warn!("DISCORD_SAY_LOG_CHANNELID is empty; this will disable logging in your server.");
			Ok(Self {
				say_log_channel_id: None,
			})
		}
	}
}

impl DiscordConfig {
	pub fn new_from_env() -> Result<Self> {
		let unparsed_client = required_var!("DISCORD_CLIENT_ID").parse::<u64>()?;
		let client_id = ApplicationId::from(unparsed_client);
		let client_secret = required_var!("DISCORD_CLIENT_SECRET");
		let bot_token = required_var!("DISCORD_BOT_TOKEN");
		let oauth2 = RefractionOAuth2::new_from_env()?;
		let channels = RefractionChannels::new_from_env()?;

		Ok(Self {
			client_id,
			client_secret,
			bot_token,
			oauth2,
			channels,
		})
	}
}
