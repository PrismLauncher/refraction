use std::sync::OnceLock;

use eyre::Result;
use log::debug;
use reqwest::{Client, Response};

pub mod dadjoke;
pub mod github;
pub mod paste_gg;
pub mod pluralkit;
pub mod prism_meta;
pub mod rory;

pub fn client() -> &'static reqwest::Client {
	static CLIENT: OnceLock<Client> = OnceLock::new();
	CLIENT.get_or_init(|| {
		let version = option_env!("CARGO_PKG_VERSION").unwrap_or("development");
		let user_agent = format!("refraction/{version}");
		Client::builder()
			.user_agent(user_agent)
			.build()
			.unwrap_or_default()
	})
}

pub async fn get_url(url: &str) -> Result<Response> {
	debug!("Making request to {url}");
	let resp = client().get(url).send().await?;
	resp.error_for_status_ref()?;

	Ok(resp)
}

pub async fn text_from_url(url: &str) -> Result<String> {
	let resp = get_url(url).await?;

	let text = resp.text().await?;
	Ok(text)
}

pub async fn bytes_from_url(url: &str) -> Result<Vec<u8>> {
	let resp = get_url(url).await?;

	let bytes = resp.bytes().await?;
	Ok(bytes.to_vec())
}
