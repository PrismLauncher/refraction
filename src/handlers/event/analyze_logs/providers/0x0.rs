use std::sync::LazyLock;

use crate::api::{HttpClient, HttpClientExt};

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

pub struct _0x0;

impl super::LogProvider for _0x0 {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: LazyLock<Regex> =
			LazyLock::new(|| Regex::new(r"https://0x0\.st/\w*.\w*").unwrap());

		trace!("Checking if message {} is a 0x0 paste", message.id);
		REGEX
			.find_iter(&message.content)
			.map(|m| m.as_str().to_string())
			.nth(0)
	}

	async fn fetch(&self, http: &HttpClient, content: &str) -> Result<String> {
		let log = http.get_request(content).await?.text().await?;

		Ok(log)
	}
}
