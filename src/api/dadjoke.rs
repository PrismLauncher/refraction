use crate::api::REQWEST_CLIENT;

use eyre::Result;
use log::debug;

const DADJOKE: &str = "https://icanhazdadjoke.com";

pub async fn get_joke() -> Result<String> {
	debug!("Making request to {DADJOKE}");

	let resp = REQWEST_CLIENT
		.get(DADJOKE)
		.header("Accept", "text/plain")
		.send()
		.await?;
	resp.error_for_status_ref()?;

	let joke = resp.text().await?;
	Ok(joke)
}
