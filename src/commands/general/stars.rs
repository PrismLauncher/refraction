use crate::{api, consts, Context};

use eyre::Result;
use log::trace;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Returns GitHub stargazer count
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn stars(ctx: Context<'_>) -> Result<()> {
	trace!("Running stars command");

	ctx.defer().await?;

	let count = if let Some(storage) = &ctx.data().storage {
		if let Ok(count) = storage.get_launcher_stargazer_count().await {
			count
		} else {
			let count = api::github::get_prism_stargazers_count().await?;
			storage.cache_launcher_stargazer_count(count).await?;
			count
		}
	} else {
		trace!("Not caching launcher stargazer count, as we're running without a storage backend");
		api::github::get_prism_stargazers_count().await?
	};

	let embed = CreateEmbed::new()
		.title(format!("⭐ {count} total stars!"))
		.color(consts::COLORS["yellow"]);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;

	Ok(())
}
