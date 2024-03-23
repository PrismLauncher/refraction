use crate::utils;

use eyre::Result;
use log::trace;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Message;
use regex::Regex;

const MCLOGS: &str = "https://api.mclo.gs/1";
const RAW: &str = "/raw";

pub struct MCLogs;

impl super::LogProvider for MCLogs {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https://mclo\.gs/(\w+)").unwrap());

		trace!("Checking if message {} is an mclo.gs paste", message.id);
		super::get_first_capture(&REGEX, &message.content)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let url = format!("{MCLOGS}{RAW}/{content}");
		let log = utils::text_from_url(&url).await?;

		Ok(log)
	}
}
