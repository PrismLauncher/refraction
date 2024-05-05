use eyre::Result;
use poise::serenity_prelude::Message;

use crate::{Context, Error};

#[poise::command(context_menu_command = "Delete command", ephemeral)]
pub async fn delete_interaction(ctx: Context<'_>, message: Message) -> Result<(), Error> {
	let Some(interaction) = &message.interaction else {
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
