use eyre::Result;

mod bot;
mod discord;

#[derive(Debug, Clone, Default)]
pub struct Config {
	pub bot: bot::Config,
	pub discord: discord::Config,
}

impl Config {
	pub fn new(bot_config: bot::Config, discord_config: discord::Config) -> Self {
		Self {
			bot: bot_config,
			discord: discord_config,
		}
	}

	pub fn from_env() -> Result<Self> {
		let bot = bot::Config::from_env();
		let discord = discord::Config::from_env()?;

		Ok(Self::new(bot, discord))
	}
}
