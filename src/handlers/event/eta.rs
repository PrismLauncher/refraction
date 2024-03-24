use eyre::Result;
use log::trace;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{Context, Message};
use rand::seq::SliceRandom;
use regex::Regex;

static ETA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\beta\b").unwrap());

const ETA_MESSAGES: [&str; 16] = [
	"Sometime",
	"Some day",
	"Not far",
	"The future",
	"Never",
	"Perhaps tomorrow?",
	"There are no ETAs",
	"No",
	"Nah",
	"Yes",
	"Yas",
	"Next month",
	"Next year",
	"Next week",
	"In Prism Launcher 2.0.0",
	"At the appropriate juncture, in due course, in the fullness of time",
];

pub async fn handle(ctx: &Context, message: &Message) -> Result<()> {
	if !ETA_REGEX.is_match(&message.content) {
		trace!(
			"The message '{}' (probably) doesn't say ETA",
			message.content
		);
		return Ok(());
	}

	let response = format!(
		"{} <:pofat:1031701005559144458>",
		ETA_MESSAGES
			.choose(&mut rand::thread_rng())
			.unwrap_or(&"sometime")
	);

	message.reply(ctx, response).await?;
	Ok(())
}
