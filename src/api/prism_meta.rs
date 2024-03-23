use crate::api::REQWEST_CLIENT;

use eyre::{OptionExt, Result};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftPackageJson {
	pub format_version: u8,
	pub name: String,
	pub recommended: Vec<String>,
	pub uid: String,
}

const META: &str = "https://meta.prismlauncher.org/v1";
const MINECRAFT_PACKAGEJSON: &str = "/net.minecraft/package.json";

pub async fn get_latest_minecraft_version() -> Result<String> {
	let url = format!("{META}{MINECRAFT_PACKAGEJSON}");

	debug!("Making request to {url}");
	let resp = REQWEST_CLIENT.get(url).send().await?;
	resp.error_for_status_ref()?;

	let data: MinecraftPackageJson = resp.json().await?;

	let version = data
		.recommended
		.first()
		.ok_or_eyre("Couldn't find latest version of Minecraft!")?;

	Ok(version.clone())
}
