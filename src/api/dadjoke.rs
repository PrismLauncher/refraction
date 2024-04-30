use super::{HttpClient, HttpClientExt};

use eyre::Result;

const DADJOKE: &str = "https://icanhazdadjoke.com";

pub async fn get_joke(http: &HttpClient) -> Result<String> {
	let joke = http
		.get(DADJOKE)
		.header("Accept", "text/plain")
		.send()
		.await?
		.text()
		.await?;

	Ok(joke)
}
