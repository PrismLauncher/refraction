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

	pub fn new_from_env() -> Self {
		let bot = bot::Config::from_env();
		let discord = discord::Config::from_env();

		Self::new(bot, discord)
	}
}
