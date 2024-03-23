use crate::api::dadjoke;
use crate::Context;

use eyre::Result;
use log::trace;

/// It's a joke
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn joke(ctx: Context<'_>) -> Result<()> {
	trace!("Running joke command");

	ctx.defer().await?;
	let joke = dadjoke::get_joke().await?;
	ctx.say(joke).await?;

	Ok(())
}
