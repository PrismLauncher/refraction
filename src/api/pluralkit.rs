use super::{HttpClient, HttpClientExt};

use eyre::{Context, Result};
use poise::serenity_prelude::{MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
	pub sender: String,
}

const PLURAL_KIT: &str = "https://api.pluralkit.me/v2";
const MESSAGES: &str = "/messages";

pub async fn sender_from(http: &HttpClient, message_id: MessageId) -> Result<UserId> {
	let url = format!("{PLURAL_KIT}{MESSAGES}/{message_id}");
	let resp: Message = http.get_request(&url).await?.json().await?;

	let id: u64 =
		resp.sender.parse().wrap_err_with(|| {
			format!("Couldn't parse response from PluralKit as a UserId! Here's the response:\n{resp:#?}")
		})?;
	let sender = UserId::from(id);

	Ok(sender)
}
