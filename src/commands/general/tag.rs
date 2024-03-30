#![allow(non_camel_case_types, clippy::upper_case_acronyms)]
use crate::{consts::Colors, tags::Tag, Context};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

use eyre::{eyre, Result};
use log::trace;
use poise::serenity_prelude::{Color, CreateEmbed, User};
use poise::CreateReply;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
fn tags() -> &'static Vec<Tag> {
	static TAGS: OnceLock<Vec<Tag>> = OnceLock::new();
	TAGS.get_or_init(|| serde_json::from_str(env!("TAGS")).unwrap())
}

/// Send a tag
#[poise::command(
	slash_command,
	prefix_command,
	track_edits = true,
	help_text_fn = help
)]
pub async fn tag(
	ctx: Context<'_>,
	#[description = "the tag to send"] name: Choice,
	#[description = "a user to mention"] user: Option<User>,
) -> Result<()> {
	trace!("Running tag command");

	let tag_id = name.as_str();
	let tag = tags()
		.iter()
		.find(|t| t.id == tag_id)
		.ok_or_else(|| eyre!("Tried to get non-existent tag: {tag_id}"))?;

	let frontmatter = &tag.frontmatter;

	let embed = {
		let mut e = CreateEmbed::new();

		if let Some(color) = &frontmatter.color {
			let color = Colors::from_str(color.as_str())
				.map(Color::from)
				.unwrap_or_default();

			e = e.color(color);
		}

		if let Some(image) = &frontmatter.image {
			e = e.image(image);
		}

		if let Some(fields) = &frontmatter.fields {
			for field in fields {
				e = e.field(&field.name, &field.value, field.inline);
			}
		}

		e = e.title(&frontmatter.title);
		e = e.description(&tag.content);

		e
	};

	let reply = {
		let mut r = CreateReply::default();

		if let Some(user) = user {
			r = r.content(format!("<@{}>", user.id));
		}

		r.embed(embed)
	};

	ctx.send(reply).await?;

	Ok(())
}

fn help() -> String {
	let tag_list = tags()
		.iter()
		.map(|tag| format!("`{}`", tag.id))
		.collect::<Vec<String>>()
		.join(", ");

	format!("Available tags: {tag_list}")
}
