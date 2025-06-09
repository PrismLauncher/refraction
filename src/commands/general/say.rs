use crate::{utils, Context, Error};

use log::trace;
use poise::serenity_prelude::{CreateEmbed, CreateMessage};

/// Say something through the bot
#[poise::command(
	slash_command,
	ephemeral,
	default_member_permissions = "MODERATE_MEMBERS",
	required_permissions = "MODERATE_MEMBERS",
	guild_only
)]
pub async fn say(
	ctx: Context<'_>,
	#[description = "the message content"] content: String,
) -> Result<(), Error> {
	let channel = ctx.channel_id();
	let content = content.replace("\\n", "\n");
	let message = channel.say(ctx, &content).await?;
	ctx.say("I said what you said!").await?;

	if let Some(channel_id) = ctx.data().config.discord.channels.log_channel_id {
		let author = utils::embed_author_from_user(ctx.author());

		let embed = CreateEmbed::default()
			.title("Say command used!")
			.description(format!(
				"{}\n\n[Jump to message]({})",
				content,
				message.link()
			))
			.author(author);

		let message = CreateMessage::new().embed(embed);
		channel_id.send_message(ctx, message).await?;
	} else {
		trace!("Not sending /say log as no channel is set");
	}

	Ok(())
}
