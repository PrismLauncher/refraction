use crate::api::{HttpClient, HttpClientExt};

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;

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

	async fn fetch(&self, http: &HttpClient, content: &str) -> Result<String> {
		let attachment = http.get_request(content).await?.bytes().await?.to_vec();
		let log = String::from_utf8(attachment)?;

		Ok(log)
	}
}
