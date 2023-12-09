use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::StatusCode;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https://0x0\.st/\w*\.\w*").unwrap());

pub async fn find(content: &str) -> Result<Option<String>> {
    let Some(url) = REGEX.find(content).map(|m| &content[m.range()]) else {
        return Ok(None);
    };

    let request = REQWEST_CLIENT.get(url).build()?;
    let response = REQWEST_CLIENT.execute(request).await?;
    let status = response.status();

    if let StatusCode::OK = status {
        Ok(Some(response.text().await?))
    } else {
        Err(eyre!("Failed to fetch paste from {url} with {status}",))
    }
}
