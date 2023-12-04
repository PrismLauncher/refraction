#![allow(non_camel_case_types, clippy::upper_case_acronyms)]
use crate::tags::Tag;
use crate::{consts, Context};
use std::env;

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;
use poise::serenity_prelude::{Color, User};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
static TAGS: Lazy<Vec<Tag>> = Lazy::new(|| serde_json::from_str(env!("TAGS")).unwrap());

/// Send a tag
#[poise::command(slash_command)]
pub async fn tag(
    ctx: Context<'_>,
    #[description = "the copypasta you want to send"] name: TagChoice,
    user: Option<User>,
) -> Result<()> {
    let tag_file = name.as_str();
    let tag = TAGS
        .iter()
        .find(|t| t.file_name == tag_file)
        .ok_or_else(|| eyre!("Tried to get non-existent tag: {tag_file}"))?;

    let frontmatter = &tag.frontmatter;

    ctx.send(|m| {
        if let Some(user) = user {
            m.content(format!("<@{}>", user.id));
        }

        m.embed(|e| {
            if let Some(color) = &frontmatter.color {
                let color = *consts::COLORS
                    .get(color.as_str())
                    .unwrap_or(&Color::default());
                e.color(color);
            }

            if let Some(image) = &frontmatter.image {
                e.image(image);
            }

            if let Some(fields) = &frontmatter.fields {
                for field in fields {
                    e.field(&field.name, &field.value, field.inline);
                }
            }

            e
        })
    })
    .await?;

    Ok(())
}
