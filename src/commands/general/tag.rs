#![allow(non_camel_case_types, clippy::upper_case_acronyms)]
use crate::tags::Tag;
use crate::{consts, Context};
use std::env;

use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use poise::serenity_prelude::{Color, CreateEmbed, User};
use poise::CreateReply;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
static TAGS: Lazy<Vec<Tag>> = Lazy::new(|| serde_json::from_str(env!("TAGS")).unwrap());

/// Send a tag
#[poise::command(slash_command, prefix_command)]
pub async fn tag(
	ctx: Context<'_>,
	#[description = "the copypasta you want to send"] name: Choice,
	user: Option<User>,
) -> Result<()> {
	let tag_file = name.as_str();
	let tag = TAGS
		.iter()
		.find(|t| t.file_name == tag_file)
		.ok_or_else(|| eyre!("Tried to get non-existent tag: {tag_file}"))?;

	let frontmatter = &tag.frontmatter;

	let embed = {
		let mut e = CreateEmbed::new();

		if let Some(color) = &frontmatter.color {
			let color = *consts::COLORS
				.get(color.as_str())
				.unwrap_or(&Color::default());
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
