use crate::api::dadjoke;
use crate::Context;

use eyre::Result;

/// It's a joke
#[poise::command(slash_command, prefix_command)]
pub async fn joke(ctx: Context<'_>) -> Result<()> {
	let joke = dadjoke::get_joke().await?;

	ctx.reply(joke).await?;
	Ok(())
}
