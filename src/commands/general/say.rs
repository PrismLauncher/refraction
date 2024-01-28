use crate::Context;

use eyre::{OptionExt, Result};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateMessage};

/// Say something through the bot
#[poise::command(
	slash_command,
	prefix_command,
	ephemeral,
	default_member_permissions = "MODERATE_MEMBERS",
	required_permissions = "MODERATE_MEMBERS"
)]
pub async fn say(ctx: Context<'_>, #[description = "Just content?"] content: String) -> Result<()> {
	let guild = ctx.guild().ok_or_eyre("Couldn't get guild!")?.to_owned();
	let channel = ctx
		.guild_channel()
		.await
		.ok_or_eyre("Couldn't get channel!")?;

	ctx.defer_ephemeral().await?;
	channel.say(ctx, &content).await?;
	ctx.say("I said what you said!").await?;

	if let Some(channel_id) = ctx.data().config.discord.channels.say_log_channel_id {
		let log_channel = guild
			.channels
			.iter()
			.find(|c| c.0 == &channel_id)
			.ok_or_eyre("Couldn't get log channel from guild!")?;

		let author = CreateEmbedAuthor::new(ctx.author().tag())
			.icon_url(ctx.author().avatar_url().unwrap_or("Undefined".to_string()));

		let embed = CreateEmbed::default()
			.title("Say command used!")
			.description(content)
			.author(author);

		let message = CreateMessage::new().embed(embed);
		log_channel.1.send_message(ctx, message).await?;
	}

	Ok(())
}
