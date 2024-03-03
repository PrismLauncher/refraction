use crate::Context;

use eyre::Result;
use log::trace;

/// Replies with pong!
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
	trace!("Running ping command!");
	ctx.reply("Pong!").await?;
	Ok(())
}
