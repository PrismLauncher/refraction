use eyre::{OptionExt, Result, WrapErr};
use log::debug;
use octocrab::Octocrab;

pub async fn get_latest_prism_version(octocrab: &Octocrab) -> Result<String> {
	debug!("Fetching the latest version of Prism Launcher");

	let version = octocrab
		.repos("PrismLauncher", "PrismLauncher")
		.releases()
		.get_latest()
		.await?
		.tag_name;

	Ok(version)
}

pub async fn get_prism_stargazers_count(octocrab: &Octocrab) -> Result<u32> {
	debug!("Fetching Prism Launcher's stargazer count");

	let stargazers_count = octocrab
		.repos("PrismLauncher", "PrismLauncher")
		.get()
		.await
		.wrap_err("Couldn't fetch PrismLauncher/PrismLauncher!")?
		.stargazers_count
		.ok_or_eyre("Couldn't retrieve stargazers_coutn from GitHub!")?;

	Ok(stargazers_count)
}
