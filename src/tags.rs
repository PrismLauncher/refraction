use poise::serenity_prelude::EmbedField;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub const TAG_DIR: &str = "tags";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TagFrontmatter {
	pub title: String,
	pub color: Option<String>,
	pub image: Option<String>,
	pub fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
	pub content: String,
	pub id: String,
	pub frontmatter: TagFrontmatter,
}
