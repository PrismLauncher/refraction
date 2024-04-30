use crate::{api::HttpClient, utils};

use eyre::Result;
use poise::serenity_prelude::{Context, CreateAllowedMentions, CreateMessage, Message};

pub async fn handle(ctx: &Context, http: &HttpClient, message: &Message) -> Result<()> {
	let embeds = utils::messages::from_message(ctx, http, message).await?;

	if !embeds.is_empty() {
		let allowed_mentions = CreateAllowedMentions::new().replied_user(false);
		let reply = CreateMessage::new()
			.reference_message(message)
			.embeds(embeds)
			.allowed_mentions(allowed_mentions);

		message.channel_id.send_message(ctx, reply).await?;
	}

	Ok(())
}
