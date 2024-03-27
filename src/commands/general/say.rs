use crate::Context;

use eyre::{OptionExt, Result};
use log::trace;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateMessage};

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
	let guild = ctx.guild().ok_or_eyre("Couldn't get guild!")?.to_owned();
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
		.say_log_channel_id()
	{
		let log_channel = guild
			.channels
			.iter()
			.find(|c| c.0 == &channel_id)
			.ok_or_eyre("Couldn't get log channel from guild!")?;

		let author = CreateEmbedAuthor::new(ctx.author().tag()).icon_url(
			ctx.author()
				.avatar_url()
				.unwrap_or_else(|| ctx.author().default_avatar_url()),
		);

		let embed = CreateEmbed::default()
			.title("Say command used!")
			.description(content)
			.author(author);

		let message = CreateMessage::new().embed(embed);
		log_channel.1.send_message(ctx, message).await?;
	} else {
		trace!("Not sending /say log as no channel is set");
	}

	Ok(())
}
