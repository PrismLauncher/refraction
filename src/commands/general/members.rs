use crate::{consts, Context};

use eyre::{eyre, Context as _, OptionExt, Result};
use log::trace;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Returns the number of members in the server
#[poise::command(slash_command, prefix_command, guild_only = true, track_edits = true)]
pub async fn members(ctx: Context<'_>) -> Result<()> {
	trace!("Running members command");

	ctx.defer().await?;

	let guild_id = ctx.guild_id().ok_or_eyre("Couldn't get guild ID!")?;
	let guild = ctx
		.http()
		.get_guild_with_counts(guild_id)
		.await
		.wrap_err_with(|| format!("Couldn't fetch guild {guild_id} with counts!"))?;

	let member_count = guild
		.approximate_member_count
		.ok_or_else(|| eyre!("Couldn't get member count for guild {guild_id}!"))?;
	let online_count = guild
		.approximate_presence_count
		.ok_or_else(|| eyre!("Couldn't get online count for guild {guild_id}!"))?;

	let embed = CreateEmbed::new()
		.title(format!("{member_count} total members!",))
		.description(format!("{online_count} online members",))
		.color(consts::COLORS["blue"]);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;
	Ok(())
}
