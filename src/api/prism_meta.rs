use super::{HttpClient, HttpClientExt};

use eyre::{OptionExt, Result};
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

pub async fn latest_minecraft_version(http: &HttpClient) -> Result<String> {
	let url = format!("{META}{MINECRAFT_PACKAGEJSON}");
	let data: MinecraftPackageJson = http.get_request(&url).await?.json().await?;

	let version = data
		.recommended
		.first()
		.ok_or_eyre("Couldn't find latest version of Minecraft!")?;

	Ok(version.clone())
}
