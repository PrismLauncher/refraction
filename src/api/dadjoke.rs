use crate::api::REQWEST_CLIENT;

use eyre::{eyre, Result};
use log::debug;
use reqwest::StatusCode;

const DADJOKE: &str = "https://icanhazdadjoke.com";

pub async fn get_joke() -> Result<String> {
	let req = REQWEST_CLIENT
		.get(DADJOKE)
		.header("Accept", "text/plain")
		.build()?;

	debug!("Making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await?;
	let status = resp.status();

	if let StatusCode::OK = status {
		Ok(resp.text().await?)
	} else {
		Err(eyre!("Couldn't get a joke!"))
	}
}
