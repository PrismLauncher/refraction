use super::{HttpClient, HttpClientExt};

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
	pub id: u64,
	pub url: String,
	pub error: Option<String>,
}

const RORY: &str = "https://rory.cat";
const PURR: &str = "/purr";

pub async fn get(http: &HttpClient, id: Option<u64>) -> Result<Response> {
	let target = id.map(|id| id.to_string()).unwrap_or_default();
	let url = format!("{RORY}{PURR}/{target}");

	let data: Response = http
		.get_request(&url)
		.await?
		.json()
		.await
		.wrap_err("Couldn't parse the rory response!")?;

	Ok(data)
}
