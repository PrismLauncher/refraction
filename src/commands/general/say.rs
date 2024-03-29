use crate::{utils, Context};

use eyre::{OptionExt, Result};
use log::trace;
use poise::serenity_prelude::{CreateEmbed, CreateMessage};

/// Say something through the bot
#[poise::command(
	slash_command,
	prefix_command,
	ephemeral,
	default_member_permissions = "MODERATE_MEMBERS",
	required_permissions = "MODERATE_MEMBERS",
	guild_only = true
)]
pub async fn say(
	ctx: Context<'_>,
	#[description = "the message content"] content: String,
) -> Result<()> {
	let channel = ctx
		.guild_channel()
		.await
		.ok_or_eyre("Couldn't get channel!")?;

	if let Context::Prefix(prefix) = ctx {
		// ignore error, we might not have perm
		let _ = prefix.msg.delete(ctx).await;
	}

	ctx.defer_ephemeral().await?;
	channel.say(ctx, &content).await?;

	if let Context::Application(_) = ctx {
		ctx.say("I said what you said!").await?;
	}

	if let Some(channel_id) = ctx
		.data()
		.config
		.clone()
		.discord_config()
		.channels()
		.log_channel_id()
	{
		let author = utils::embed_author_from_user(ctx.author());

		let embed = CreateEmbed::default()
			.title("Say command used!")
			.description(content)
			.author(author);

		let message = CreateMessage::new().embed(embed);
		channel_id.send_message(ctx, message).await?;
	} else {
		trace!("Not sending /say log as no channel is set");
	}

	Ok(())
}
