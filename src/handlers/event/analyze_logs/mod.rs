use crate::consts::COLORS;
use crate::Data;

use eyre::Result;
use log::debug;
use poise::serenity_prelude::{
	Context, CreateAllowedMentions, CreateEmbed, CreateMessage, Message,
};

mod issues;
mod providers;

use providers::find_log;

pub async fn handle(ctx: &Context, message: &Message, data: &Data) -> Result<()> {
	let channel = message.channel_id;

	let log = find_log(message).await;

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

	let issues = issues::find(&log, data).await?;

	let embed = {
		let mut e = CreateEmbed::new().title("Log analysis");

		if issues.is_empty() {
			e = e
				.color(COLORS["green"])
				.description("No issues found automatically");
		} else {
			e = e.color(COLORS["red"]);

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
