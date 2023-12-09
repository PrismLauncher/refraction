use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Context, Result};
use log::*;
use poise::serenity_prelude::{MessageId, UserId};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluralKitMessage {
    pub sender: String,
}

const PLURAL_KIT: &str = "https://api.pluralkit.me/v2";
const MESSAGES_ENDPOINT: &str = "/messages";

pub async fn get_sender(message_id: MessageId) -> Result<UserId> {
    let req = REQWEST_CLIENT
        .get(format!("{PLURAL_KIT}{MESSAGES_ENDPOINT}/{message_id}"))
        .build()?;

    debug!("Making request to {}", req.url());
    let resp = REQWEST_CLIENT.execute(req).await?;
    let status = resp.status();

    if let StatusCode::OK = status {
        let data = resp.json::<PluralKitMessage>().await?;
        let id: u64 = data.sender.parse().wrap_err_with(|| format!("Couldn't parse response from PluralKit as a UserId! Here's the response:\n{data:#?}"))?;
        let sender = UserId::from(id);

        Ok(sender)
    } else {
        Err(eyre!(
            "Failed to get PluralKit message information from {PLURAL_KIT}{MESSAGES_ENDPOINT}/{message_id} with {status}",
        ))
    }
}
