use eyre::Result;
use poise::serenity_prelude::{Context, CreateAllowedMentions, CreateMessage, Message};

use crate::utils;

pub async fn handle(ctx: &Context, message: &Message) -> Result<()> {
	let embeds = utils::resolve_message(ctx, message).await?;

	// TODO getchoo: actually reply to user
	// ...not sure why Message doesn't give me a builder in reply() or equivalents
	if !embeds.is_empty() {
		let allowed_mentions = CreateAllowedMentions::new().replied_user(false);
		let reply = CreateMessage::new()
			.embeds(embeds)
			.allowed_mentions(allowed_mentions);

		message.channel_id.send_message(ctx, reply).await?;
	}

	Ok(())
}
