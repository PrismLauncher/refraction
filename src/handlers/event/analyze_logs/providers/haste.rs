use crate::api::{HttpClient, HttpClientExt};

use std::sync::LazyLock;

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

const HASTE: &str = "https://hst.sh";
const RAW: &str = "/raw";

pub struct Haste;

impl super::LogProvider for Haste {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: LazyLock<Regex> =
			LazyLock::new(|| Regex::new(r"https://hst\.sh(?:/raw)?/(\w+(?:\.\w*)?)").unwrap());

		trace!("Checking if message {} is a hst.sh paste", message.id);
		super::get_first_capture(&REGEX, &message.content)
	}

	async fn fetch(&self, http: &HttpClient, content: &str) -> Result<String> {
		let url = format!("{HASTE}{RAW}/{content}");
		let log = http.get_request(&url).await?.text().await?;

		Ok(log)
	}
}
