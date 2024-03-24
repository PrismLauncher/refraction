use crate::{api, Data};
use std::time::Duration;

use eyre::Result;
use log::{debug, trace};
use poise::serenity_prelude::{Context, Message};
use tokio::time::sleep;

const PK_DELAY: Duration = Duration::from_secs(1);

pub async fn is_message_proxied(message: &Message) -> Result<bool> {
	trace!(
		"Waiting on PluralKit API for {} seconds",
		PK_DELAY.as_secs()
	);
	sleep(PK_DELAY).await;

	let proxied = api::pluralkit::get_sender(message.id).await.is_ok();

	Ok(proxied)
}

pub async fn handle(_: &Context, msg: &Message, data: &Data) -> Result<()> {
	if msg.webhook_id.is_none() {
		return Ok(());
	}

	debug!(
		"Message {} has a webhook ID. Checking if it was sent through PluralKit",
		msg.id
	);

	trace!(
		"Waiting on PluralKit API for {} seconds",
		PK_DELAY.as_secs()
	);
	sleep(PK_DELAY).await;

	if let Ok(sender) = api::pluralkit::get_sender(msg.id).await {
		data.storage.store_user_plurality(sender).await?;
	}

	Ok(())
}
