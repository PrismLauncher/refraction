use crate::{api::dadjoke, Context, Error};

use eyre::Result;
use log::trace;

/// It's a joke
#[poise::command(slash_command)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
	trace!("Running joke command");

	ctx.defer().await?;
	let joke = dadjoke::get_joke(&ctx.data().http_client).await?;
	ctx.say(joke).await?;

	Ok(())
}
