use eyre::{Context as _, Result};
use log::trace;
use poise::serenity_prelude::{Context, MessageInteractionMetadata, Reaction};

pub async fn handle(ctx: &Context, reaction: &Reaction) -> Result<()> {
	let user = reaction
		.user(ctx)
		.await
		.wrap_err("Couldn't fetch user from reaction!")?;

	let message = reaction
		.message(ctx)
		.await
		.wrap_err("Couldn't fetch message from reaction!")?;

	if let Some(MessageInteractionMetadata::Command(metadata)) =
		message.interaction_metadata.as_deref()
	{
		if metadata.user == user && reaction.emoji.unicode_eq("❌") {
			trace!("Deleting our own message at the request of {}", user.tag());
			message.delete(ctx).await?;
		}
	}

	Ok(())
}
