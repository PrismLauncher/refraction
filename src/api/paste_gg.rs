use eyre::{eyre, OptionExt, Result};
use log::debug;
use serde::{Deserialize, Serialize};

const PASTE_GG: &str = "https://api.paste.gg/v1";
const PASTES: &str = "/pastes";

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Status {
	#[serde(rename = "success")]
	Success,
	#[serde(rename = "error")]
	Error,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response<T> {
	pub status: Status,
	pub result: Option<Vec<T>>,
	pub error: Option<String>,
	pub message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Files {
	pub id: String,
	pub name: Option<String>,
}

pub async fn files_from(id: &str) -> Result<Response<Files>> {
	let url = format!("{PASTE_GG}{PASTES}/{id}/files");
	debug!("Making request to {url}");

	let resp: Response<Files> = super::json_from_url(&url).await?;

	if resp.status == Status::Error {
		let message = resp
			.error
			.ok_or_eyre("Paste.gg gave us an error but with no message!")?;

		Err(eyre!(message))
	} else {
		Ok(resp)
	}
}

pub async fn get_raw_file(paste_id: &str, file_id: &str) -> eyre::Result<String> {
	let url = format!("{PASTE_GG}{PASTES}/{paste_id}/files/{file_id}/raw");
	let text = super::text_from_url(&url).await?;

	Ok(text)
}
