use std::sync::OnceLock;

use eyre::Result;
use log::trace;
use poise::serenity_prelude::{Context, Message};
use rand::seq::SliceRandom;
use regex::Regex;

fn regex() -> &'static Regex {
	static REGEX: OnceLock<Regex> = OnceLock::new();
	REGEX.get_or_init(|| Regex::new(r"\beta\b").unwrap())
}

const MESSAGES: [&str; 16] = [
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
	if !regex().is_match(&message.content) {
		trace!(
			"The message '{}' (probably) doesn't say ETA",
			message.content
		);
		return Ok(());
	}

	let response = format!(
		"{} <:pofat:1031701005559144458>",
		MESSAGES
			.choose(&mut rand::thread_rng())
			.unwrap_or(&"sometime")
	);

	message.reply(ctx, response).await?;
	Ok(())
}
