mod discord;

#[derive(Debug, Clone)]
pub struct Config {
	pub discord: discord::Config,
	pub redis_url: String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			discord: discord::Config::default(),
			redis_url: "redis://localhost:6379".to_string(),
		}
	}
}

impl Config {
	pub fn new_from_env() -> Self {
		let discord = discord::Config::new_from_env();

		Self {
			discord,
			..Default::default()
		}
	}
}
