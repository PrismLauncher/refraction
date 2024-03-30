use crate::api;

use std::sync::OnceLock;

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

const PASTEBIN: &str = "https://pastebin.com";
const RAW: &str = "/raw";

pub struct PasteBin;

impl super::LogProvider for PasteBin {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: OnceLock<Regex> = OnceLock::new();
		let regex =
			REGEX.get_or_init(|| Regex::new(r"https://pastebin\.com(?:/raw)?/(\w+)").unwrap());

		trace!("Checking if message {} is a pastebin paste", message.id);
		super::get_first_capture(regex, &message.content)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let url = format!("{PASTEBIN}{RAW}/{content}");
		let log = api::text_from_url(&url).await?;

		Ok(log)
	}
}
