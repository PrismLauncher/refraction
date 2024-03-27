use std::slice::Iter;

use enum_dispatch::enum_dispatch;
use eyre::Result;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Message;
use regex::Regex;

use self::{
	_0x0::_0x0 as _0x0st, attachment::Attachment, haste::Haste, mclogs::MCLogs, paste_gg::PasteGG,
	pastebin::PasteBin,
};

#[path = "0x0.rs"]
mod _0x0;
mod attachment;
mod haste;
mod mclogs;
mod paste_gg;
mod pastebin;

#[enum_dispatch]
pub trait LogProvider {
	async fn find_match(&self, message: &Message) -> Option<String>;
	async fn fetch(&self, content: &str) -> Result<String>;
}

fn get_first_capture(regex: &Lazy<Regex>, string: &str) -> Option<String> {
	regex
		.captures_iter(string)
		.find_map(|c| c.get(1).map(|c| c.as_str().to_string()))
}

#[enum_dispatch(LogProvider)]
enum Provider {
	_0x0st,
	Attachment,
	Haste,
	MCLogs,
	PasteGG,
	PasteBin,
}

impl Provider {
	pub fn interator() -> Iter<'static, Provider> {
		static PROVIDERS: [Provider; 6] = [
			Provider::_0x0st(_0x0st),
			Provider::Attachment(Attachment),
			Provider::Haste(Haste),
			Provider::MCLogs(MCLogs),
			Provider::PasteBin(PasteBin),
			Provider::PasteGG(PasteGG),
		];
		PROVIDERS.iter()
	}
}

pub async fn find_log(message: &Message) -> Result<Option<String>> {
	let providers = Provider::interator();

	for provider in providers {
		if let Some(found) = provider.find_match(message).await {
			let log = provider.fetch(&found).await?;
			return Ok(Some(log));
		}
	}

	Ok(None)
}
