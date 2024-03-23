use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;

use crate::utils;

pub struct Attachment;

impl super::LogProvider for Attachment {
	async fn find_match(&self, message: &Message) -> Option<String> {
		trace!("Checking if message {} has text attachments", message.id);

		message
			.attachments
			.iter()
			.filter_map(|a| {
				a.content_type
					.as_ref()
					.and_then(|ct| ct.starts_with("text/").then_some(a.url.clone()))
			})
			.nth(0)
	}

	async fn fetch(&self, content: &str) -> Result<String> {
		let attachment = utils::bytes_from_url(content).await?;
		let log = String::from_utf8(attachment)?;
		Ok(log)
	}
}
