use poise::serenity_prelude::{InteractionType, Message, Permissions};

use crate::{Context, Error};

#[poise::command(context_menu_command = "Delete command", ephemeral)]
pub async fn delete_interaction(ctx: Context<'_>, message: Message) -> Result<(), Error> {
	const NO_COMMAND: &str = "❌ This message does not contain a command";

	let Some(interaction) = &message.interaction else {
		ctx.say(NO_COMMAND).await?;
		return Ok(());
	};

	if interaction.kind != InteractionType::Command {
		ctx.say(NO_COMMAND).await?;
		return Ok(());
	}

	let can_manage = interaction
		.member
		.as_ref()
		.and_then(|m| m.permissions)
		.and_then(|p| p.contains(Permissions::MANAGE_MESSAGES).then_some(true))
		.is_some();

	if interaction.user.id != ctx.author().id && !can_manage {
		ctx.say("❌ You cannot delete commands run by other users")
			.await?;
		return Ok(());
	}

	message.delete(ctx).await?;
	ctx.say("🗑️ Deleted command!").await?;
	Ok(())
}
