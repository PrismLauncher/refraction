use crate::{consts::COLORS, Context};

use color_eyre::eyre::{Context as _, Result};

/// Returns GitHub stargazer count
#[poise::command(slash_command, prefix_command)]
pub async fn stars(ctx: Context<'_>) -> Result<()> {
	let prismlauncher = ctx
		.data()
		.octocrab
		.repos("PrismLauncher", "PrismLauncher")
		.get()
		.await
		.wrap_err_with(|| "Couldn't get PrismLauncher/PrismLauncher from GitHub!")?;

	let count = if let Some(count) = prismlauncher.stargazers_count {
		count.to_string()
	} else {
		"undefined".to_string()
	};

	ctx.send(|m| {
		m.embed(|e| {
			e.title(format!("⭐ {count} total stars!"))
				.color(COLORS["yellow"])
		})
	})
	.await?;

	Ok(())
}
