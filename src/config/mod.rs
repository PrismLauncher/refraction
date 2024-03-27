mod bot;
mod discord;

#[derive(Debug, Clone, Default)]
pub struct Config {
	bot: bot::Config,
	discord: discord::Config,
}

impl Config {
	pub fn new(bot_config: bot::Config, discord_config: discord::Config) -> Self {
		Self {
			bot: bot_config,
			discord: discord_config,
		}
	}

	pub fn new_from_env() -> Self {
		let bot = bot::Config::new_from_env();
		let discord = discord::Config::new_from_env();

		Self::new(bot, discord)
	}

	pub fn bot_config(self) -> bot::Config {
		self.bot
	}

	pub fn discord_config(self) -> discord::Config {
		self.discord
	}
}
