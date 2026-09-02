use std::sync::LazyLock;

use crate::api::{HttpClient, HttpClientExt};

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

const PASTEBIN: &str = "https://pastebin.com";
const RAW: &str = "/raw";

pub struct PasteBin;

impl super::LogProvider for PasteBin {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: LazyLock<Regex> =
			LazyLock::new(|| Regex::new(r"https://pastebin\.com(?:/raw)?/(\w+)").unwrap());

		trace!("Checking if message {} is a pastebin paste", message.id);
		super::get_first_capture(&REGEX, &message.content)
	}

	async fn fetch(&self, http: &HttpClient, content: &str) -> Result<String> {
		let url = format!("{PASTEBIN}{RAW}/{content}");
		let log = http.get_request(&url).await?.text().await?;

		Ok(log)
	}
}
