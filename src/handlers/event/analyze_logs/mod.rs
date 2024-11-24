use crate::{consts::Colors, Data};

use eyre::Result;
use log::{debug, trace};
use poise::serenity_prelude::{
	Context, CreateAllowedMentions, CreateEmbed, CreateMessage, Message,
};

mod issues;
mod providers;

use providers::find_log;

pub async fn handle(ctx: &Context, message: &Message, data: &Data) -> Result<()> {
	trace!(
		"Checking message {} from {} for logs",
		message.id,
		message.author.id
	);
	let channel = message.channel_id;

	let log = find_log(&data.http_client, message).await;

	if log.is_err() {
		let embed = CreateEmbed::new()
			.title("Analysis failed!")
			.description("Couldn't download log");
		let allowed_mentions = CreateAllowedMentions::new().replied_user(true);
		let our_message = CreateMessage::new()
			.reference_message(message)
			.allowed_mentions(allowed_mentions)
			.embed(embed);

		channel.send_message(ctx, our_message).await?;

		return Ok(());
	}

	let Some(log) = log? else {
		debug!("No log found in message! Skipping analysis");
		return Ok(());
	};

	let log = log.replace("\r\n", "\n");

	let issues = issues::find(&log, data).await?;

	let embed = {
		let mut e = CreateEmbed::new().title("Log analysis");

		if issues.is_empty() {
			e = e
				.color(Colors::Green)
				.description("The automatic check didn't reveal any issues, but it's possible that some issues went undetected. Please wait for a volunteer to assist you.");
		} else {
			e = e.color(Colors::Red);

			for (title, description) in issues {
				e = e.field(title, description, false);
			}
		}

		e
	};

	let allowed_mentions = CreateAllowedMentions::new().replied_user(true);
	let message = CreateMessage::new()
		.reference_message(message)
		.allowed_mentions(allowed_mentions)
		.embed(embed);

	channel.send_message(ctx, message).await?;

	Ok(())
}
