use poise::serenity_prelude::{Message, MessageInteractionMetadata::Command};

use crate::{Context, Error};

#[poise::command(context_menu_command = "Delete command", ephemeral)]
pub async fn delete_interaction(ctx: Context<'_>, message: Message) -> Result<(), Error> {
	let Some(Command(interaction)) = message.interaction_metadata.as_deref() else {
		ctx.say("❌ This message does not contain a command")
			.await?;
		return Ok(());
	};

	if interaction.user.id != ctx.author().id {
		ctx.say("❌ You cannot delete commands run by other users")
			.await?;
		return Ok(());
	}

	message.delete(ctx).await?;
	ctx.say("🗑️ Deleted command!").await?;
	Ok(())
}
