use std::sync::Arc;

use eyre::{Context, OptionExt, Result};
use log::debug;
use octocrab::Octocrab;
use once_cell::sync::Lazy;

static OCTOCRAB: Lazy<Arc<Octocrab>> = Lazy::new(octocrab::instance);

pub async fn get_latest_prism_version() -> Result<String> {
	debug!("Fetching the latest version of Prism Launcher");

	let version = OCTOCRAB
		.repos("PrismLauncher", "PrismLauncher")
		.releases()
		.get_latest()
		.await?
		.tag_name;

	Ok(version)
}

pub async fn get_prism_stargazers_count() -> Result<u32> {
	debug!("Fetching Prism Launcher's stargazer count");

	let stargazers_count = OCTOCRAB
		.repos("PrismLauncher", "PrismLauncher")
		.get()
		.await
		.wrap_err("Couldn't fetch PrismLauncher/PrismLauncher!")?
		.stargazers_count
		.ok_or_eyre("Couldn't retrieve stargazers_coutn from GitHub!")?;

	Ok(stargazers_count)
}
