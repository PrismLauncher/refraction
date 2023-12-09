use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Context, Result};
use log::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftPackageJson {
    pub format_version: u8,
    pub name: String,
    pub recommended: Vec<String>,
    pub uid: String,
}

const PRISM_META: &str = "https://meta.prismlauncher.org/v1";
const MINECRAFT_PACKAGEJSON_ENDPOINT: &str = "/net.minecraft/package.json";

pub async fn get_latest_minecraft_version() -> Result<String> {
    let req = REQWEST_CLIENT
        .get(format!("{PRISM_META}{MINECRAFT_PACKAGEJSON_ENDPOINT}"))
        .build()?;

    debug!("Making request to {}", req.url());
    let resp = REQWEST_CLIENT.execute(req).await?;
    let status = resp.status();

    if let StatusCode::OK = status {
        let data = resp
            .json::<MinecraftPackageJson>()
            .await
            .wrap_err_with(|| "Couldn't parse Minecraft versions!")?;

        let version = data
            .recommended
            .first()
            .ok_or_else(|| eyre!("Couldn't find latest version of Minecraft!"))?;

        Ok(version.clone())
    } else {
        Err(eyre!(
            "Failed to get latest Minecraft version from {PRISM_META}{MINECRAFT_PACKAGEJSON_ENDPOINT} with {status}",
        ))
    }
}
