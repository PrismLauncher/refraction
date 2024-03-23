use crate::api::REQWEST_CLIENT;

use eyre::Result;
use log::debug;
use reqwest::Response;

pub mod resolve_message;

pub async fn get_url(url: &str) -> Result<Response> {
	debug!("Making request to {url}");
	let resp = REQWEST_CLIENT.get(url).send().await?;
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
