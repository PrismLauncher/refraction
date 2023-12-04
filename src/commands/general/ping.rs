use crate::Context;

use color_eyre::eyre::Result;

/// Replies with pong!
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.reply("Pong!").await?;
    Ok(())
}
