mod discord;
use discord::DiscordConfig;

#[derive(Debug, Clone)]
pub struct Config {
	pub discord: DiscordConfig,
	pub redis_url: String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			discord: DiscordConfig::default(),
			redis_url: "redis://localhost:6379".to_string(),
		}
	}
}

impl Config {
	pub fn new_from_env() -> Self {
		let discord = DiscordConfig::new_from_env();

		Self {
			discord,
			..Default::default()
		}
	}
}
