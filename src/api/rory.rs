use crate::api::REQWEST_CLIENT;

use eyre::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
	pub id: u64,
	pub url: String,
	pub error: Option<String>,
}

const RORY: &str = "https://rory.cat";
const PURR: &str = "/purr";

pub async fn get(id: Option<u64>) -> Result<Response> {
	let target = id.map(|id| id.to_string()).unwrap_or_default();
	let url = format!("{RORY}{PURR}/{target}");

	debug!("Making request to {url}");

	let resp = REQWEST_CLIENT
		.get(format!("{RORY}{PURR}/{target}"))
		.send()
		.await?;
	resp.error_for_status_ref()?;

	let data: Response = resp
		.json()
		.await
		.wrap_err("Couldn't parse the rory response!")?;

	Ok(data)
}
