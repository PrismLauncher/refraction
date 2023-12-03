use color_eyre::eyre::Result;

mod discord;
mod github;

pub use discord::*;
pub use github::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub github: GithubConfig,
    pub http_port: u16,
    pub redis_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: DiscordConfig::default(),
            github: GithubConfig::default(),
            http_port: 3000,
            redis_url: "redis://localhost:6379".to_string(),
        }
    }
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        let discord = DiscordConfig::new_from_env()?;
        let github = GithubConfig::new_from_env()?;

        Ok(Self {
            discord,
            github,
            ..Default::default()
        })
    }
}
