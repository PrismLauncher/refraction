use crate::api;

use std::sync::OnceLock;

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

const HASTE: &str = "https://hst.sh";
const RAW: &str = "/raw";

pub struct Haste;

impl super::LogProvider for Haste {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: OnceLock<Regex> = OnceLock::new();
		let regex =
			REGEX.get_or_init(|| Regex::new(r"https://hst\.sh(?:/raw)?/(\w+(?:\.\w*)?)").unwrap());

		trace!("Checking if message {} is a hst.sh paste", message.id);
		super::get_first_capture(regex, &message.content)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let url = format!("{HASTE}{RAW}/{content}");
		let log = api::text_from_url(&url).await?;

		Ok(log)
	}
}
