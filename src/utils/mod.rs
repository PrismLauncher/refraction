use poise::serenity_prelude::{CreateEmbedAuthor, User};

pub mod messages;

pub fn embed_author_from_user(user: &User) -> CreateEmbedAuthor {
	CreateEmbedAuthor::new(user.tag()).icon_url(
		user.avatar_url()
			.unwrap_or_else(|| user.default_avatar_url()),
	)
}

pub fn semver_split(version: &str) -> Vec<u32> {
	version
		.split('.')
		.map(|s| s.parse().ok())
		.flatten()
		.collect::<Vec<u32>>()
}
