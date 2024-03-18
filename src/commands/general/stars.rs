use crate::{consts, Context};

use eyre::{Context as _, Result};
use log::trace;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Returns GitHub stargazer count
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn stars(ctx: Context<'_>) -> Result<()> {
	trace!("Running stars command");

	ctx.defer().await?;

	let prismlauncher = ctx
		.data()
		.octocrab
		.repos("PrismLauncher", "PrismLauncher")
		.get()
		.await
		.wrap_err("Couldn't get PrismLauncher/PrismLauncher from GitHub!")?;

	let count = if let Some(count) = prismlauncher.stargazers_count {
		count.to_string()
	} else {
		"undefined".to_string()
	};

	let embed = CreateEmbed::new()
		.title(format!("⭐ {count} total stars!"))
		.color(consts::COLORS["yellow"]);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;

	Ok(())
}
