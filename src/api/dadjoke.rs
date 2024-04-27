use super::{HttpClient, HttpClientExt};

use eyre::Result;

const DADJOKE: &str = "https://icanhazdadjoke.com";

pub async fn get_joke(http: &HttpClient) -> Result<String> {
	let joke = http.get_request(DADJOKE).await?.text().await?;

	Ok(joke)
}
