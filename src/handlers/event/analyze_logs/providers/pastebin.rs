use crate::utils;

use eyre::Result;
use log::trace;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Message;
use regex::Regex;

const PASTEBIN: &str = "https://pastebin.com";
const RAW: &str = "/raw";

pub struct PasteBin;

impl super::LogProvider for PasteBin {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: Lazy<Regex> =
			Lazy::new(|| Regex::new(r"https://pastebin\.com(?:/raw)?/(\w+)").unwrap());

		trace!("Checking if message {} is a pastebin paste", message.id);
		super::get_first_capture(&REGEX, &message.content)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let url = format!("{PASTEBIN}{RAW}/{content}");
		let log = utils::text_from_url(&url).await?;

		Ok(log)
	}
}
