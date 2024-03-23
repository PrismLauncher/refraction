use crate::utils;

use eyre::Result;
use log::trace;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Message;
use regex::Regex;

pub struct _0x0;

impl super::LogProvider for _0x0 {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https://0x0\.st/\w*\.\w*").unwrap());

		trace!("Checking if message {} is a 0x0 paste", message.id);
		REGEX
			.find_iter(&message.content)
			.map(|m| m.as_str().to_string())
			.nth(0)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let log = utils::text_from_url(content).await?;

		Ok(log)
	}
}
