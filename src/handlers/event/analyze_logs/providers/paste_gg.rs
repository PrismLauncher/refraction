use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const PASTE_GG: &str = "https://api.paste.gg/v1";
const PASTES_ENDPOINT: &str = "/pastes";
static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https://paste.gg/p/\w+/(\w+)").unwrap());

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PasteResponse {
    status: String,
    result: Option<Vec<PasteResult>>,
    error: Option<String>,
    message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PasteResult {
    id: String,
    name: Option<String>,
    description: Option<String>,
    visibility: Option<String>,
}

pub async fn find(content: &str) -> Result<Option<String>> {
    let Some(captures) = REGEX.captures(content) else {
        return Ok(None);
    };

    let paste_id = &captures[1];
    let files_url = format!("{PASTE_GG}{PASTES_ENDPOINT}/{paste_id}/files");

    let resp = REQWEST_CLIENT
        .execute(REQWEST_CLIENT.get(&files_url).build()?)
        .await?;
    let status = resp.status();

    if resp.status() != StatusCode::OK {
        return Err(eyre!(
            "Couldn't get paste {paste_id} from {PASTE_GG} with status {status}!"
        ));
    }

    let paste_files: PasteResponse = resp.json().await?;
    let file_id = &paste_files
        .result
        .ok_or_else(|| eyre!("Couldn't find any files associated with paste {paste_id}!"))?[0]
        .id;

    let raw_url = format!("{PASTE_GG}{PASTES_ENDPOINT}/{paste_id}/files/{file_id}/raw");

    let resp = REQWEST_CLIENT
        .execute(REQWEST_CLIENT.get(&raw_url).build()?)
        .await?;
    let status = resp.status();

    if status != StatusCode::OK {
        return Err(eyre!(
            "Couldn't get file {file_id} from paste {paste_id} with status {status}!"
        ));
    }

    let text = resp.text().await?;

    Ok(Some(text))
}
