use std::time::{Duration, Instant};

use crate::{Context, Error};

use log::trace;
use poise::CreateReply;

const PING_PREFIX: &str = "<:catstareback:1078622789885497414> Pong!";

/// Replies with pong!
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	trace!("Running ping command!");

	let start = Instant::now();
	let response = ctx.say(PING_PREFIX).await?;

	let rtt = start.elapsed().as_millis();
	let gateway_ping = match ctx.ping().await {
		Duration::ZERO => "Undetermined".to_string(),
		duration => format!("{}ms", duration.as_millis()),
	};

	response
		.edit(
			ctx,
			CreateReply::default().content(format!(
				"{PING_PREFIX}\n\nRTT: {rtt}ms\nGateway: {gateway_ping}",
			)),
		)
		.await?;

	Ok(())
}
