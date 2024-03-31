use crate::{api::dadjoke, Context, Error};

use eyre::Result;
use log::trace;

/// It's a joke
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
	trace!("Running joke command");

	ctx.defer().await?;
	let joke = dadjoke::get_joke().await?;
	ctx.say(joke).await?;

	Ok(())
}
