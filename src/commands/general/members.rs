use crate::{consts, Context};

use eyre::{eyre, Result};
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Returns the number of members in the server
#[poise::command(slash_command, prefix_command)]
pub async fn members(ctx: Context<'_>) -> Result<()> {
	let guild = ctx
		.guild()
		.ok_or_else(|| eyre!("Couldn't fetch guild!"))?
		.to_owned();

	let count = guild.member_count;
	let online = if let Some(count) = guild.approximate_presence_count {
		count.to_string()
	} else {
		"Undefined".to_string()
	};

	let embed = CreateEmbed::new()
		.title(format!("{count} total members!"))
		.description(format!("{online} online members"))
		.color(consts::COLORS["blue"]);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;
	Ok(())
}
