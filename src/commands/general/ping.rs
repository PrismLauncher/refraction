use crate::{Context, Error};

use log::trace;

/// Replies with pong!
#[poise::command(slash_command, prefix_command, track_edits = true, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	trace!("Running ping command!");
	ctx.say("Pong!").await?;
	Ok(())
}
