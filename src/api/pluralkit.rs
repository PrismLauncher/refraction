use crate::api::REQWEST_CLIENT;

use eyre::{Context, Result};
use log::debug;
use poise::serenity_prelude::{MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
	pub sender: String,
}

const PLURAL_KIT: &str = "https://api.pluralkit.me/v2";
const MESSAGES: &str = "/messages";

pub async fn get_sender(message_id: MessageId) -> Result<UserId> {
	let url = format!("{PLURAL_KIT}{MESSAGES}/{message_id}");

	debug!("Making request to {url}");
	let resp = REQWEST_CLIENT.get(url).send().await?;
	resp.error_for_status_ref()?;

	let data: Message = resp.json().await?;
	let id: u64 =
		data.sender.parse().wrap_err_with(|| {
			format!("Couldn't parse response from PluralKit as a UserId! Here's the response:\n{data:#?}")
		})?;
	let sender = UserId::from(id);

	Ok(sender)
}
