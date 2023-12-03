use color_eyre::eyre::{Context as _, Result};

use crate::required_var;

#[derive(Debug, Clone)]
pub struct RefractionRepo {
    pub owner: String,
    pub repo: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GithubConfig {
    pub token: String,
    pub repos: Vec<RefractionRepo>,
    pub cache_sec: u16,
    pub update_cron_job: String,
}

impl Default for GithubConfig {
    fn default() -> Self {
        let owner = "PrismLauncher".to_string();
        let repos = Vec::<RefractionRepo>::from([
            RefractionRepo {
                owner: owner.clone(),
                repo: "PrismLauncher".to_string(),
                key: "launcher".to_string(),
                name: "Launcher contributor".to_string(),
            },
            RefractionRepo {
                owner: owner.clone(),
                repo: "prismlauncher.org".to_string(),

                key: "website".to_string(),
                name: "Web developer".to_string(),
            },
            RefractionRepo {
                owner: owner.clone(),
                repo: "Translations".to_string(),

                key: "translations".to_string(),
                name: "Translator".to_string(),
            },
        ]);

        Self {
            repos,
            cache_sec: 3600,
            update_cron_job: "0 */10 * * * *".to_string(), // every 10 minutes
            token: String::default(),
        }
    }
}

impl GithubConfig {
    pub fn new_from_env() -> Result<Self> {
        let token = required_var!("GITHUB_TOKEN");

        Ok(Self {
            token,
            ..Default::default()
        })
    }
}
