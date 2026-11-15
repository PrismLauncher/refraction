use crate::{api, consts::Colors, Context, Error};

use log::trace;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Returns GitHub stargazer count
#[poise::command(slash_command)]
pub async fn stars(ctx: Context<'_>) -> Result<(), Error> {
	trace!("Running stars command");
	let octocrab = &ctx.data().octocrab;

	ctx.defer().await?;

	let count = if let Some(storage) = &ctx.data().storage {
		if let Ok(count) = storage.launcher_stargazer_count().await {
			count
		} else {
			let count = api::github::get_prism_stargazers_count(octocrab).await?;
			storage.cache_launcher_stargazer_count(count).await?;
			count
		}
	} else {
		trace!("Not caching launcher stargazer count, as we're running without a storage backend");
		api::github::get_prism_stargazers_count(octocrab).await?
	};

	let embed = CreateEmbed::new()
		.title(format!("⭐ {count} total stars!"))
		.color(Colors::Yellow);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;

	Ok(())
}
