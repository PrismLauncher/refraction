use std::{fmt::Write, str::FromStr};

use crate::{api, utils, Context, Error};

use eyre::Result;
use log::trace;
use poise::serenity_prelude::{
	futures::TryStreamExt, Attachment, CreateActionRow, CreateButton, CreateEmbed, CreateMessage,
	Mentionable, Message, ReactionType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WelcomeEmbed {
	title: String,
	description: Option<String>,
	url: Option<String>,
	hex_color: Option<String>,
	image: Option<String>,
}

impl From<WelcomeEmbed> for CreateMessage {
	fn from(val: WelcomeEmbed) -> Self {
		let mut embed = CreateEmbed::new();

		embed = embed.title(val.title);
		if let Some(description) = val.description {
			embed = embed.description(description);
		}

		if let Some(url) = val.url {
			embed = embed.url(url);
		}

		if let Some(color) = val.hex_color {
			let hex = i32::from_str_radix(&color, 16).unwrap();
			embed = embed.color(hex);
		}

		if let Some(image) = val.image {
			embed = embed.image(image);
		}

		Self::new().embed(embed)
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WelcomeRole {
	title: String,
	id: u64,
	emoji: Option<String>,
}

impl From<WelcomeRole> for CreateButton {
	fn from(value: WelcomeRole) -> Self {
		let mut button = Self::new(value.id.to_string()).label(value.title);
		if let Some(emoji) = value.emoji {
			button = button.emoji(ReactionType::from_str(&emoji).unwrap());
		}

		button
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WelcomeRoleCategory {
	title: String,
	description: Option<String>,
	roles: Vec<WelcomeRole>,
}

impl From<WelcomeRoleCategory> for CreateMessage {
	fn from(value: WelcomeRoleCategory) -> Self {
		let mut content = format!("**{}**", value.title);
		if let Some(description) = value.description {
			write!(content, "\n{description}").ok();
		}

		let buttons: Vec<CreateButton> = value
			.roles
			.iter()
			.map(|role| CreateButton::from(role.clone()))
			.collect();

		let components = vec![CreateActionRow::Buttons(buttons)];
		Self::new().content(content).components(components)
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WelcomeLayout {
	embeds: Vec<WelcomeEmbed>,
	messages: Vec<String>,
	roles: Vec<WelcomeRoleCategory>,
}

/// Sets your welcome channel info
#[poise::command(
	slash_command,
	guild_only,
	ephemeral,
	default_member_permissions = "MANAGE_GUILD",
	required_permissions = "MANAGE_GUILD"
)]
pub async fn set_welcome(
	ctx: Context<'_>,
	#[description = "A file to use"] file: Option<Attachment>,
	#[description = "A URL for a file to use"] url: Option<String>,
) -> Result<(), Error> {
	trace!("Running set_welcome command!");

	let configured_channels = ctx.data().config.discord.channels;
	let Some(channel_id) = configured_channels.welcome_channel_id else {
		ctx.say("You don't have a welcome channel ID set, so I can't do anything :(")
			.await?;
		return Ok(());
	};

	ctx.defer_ephemeral().await?;

	// download attachment from discord or URL
	let file = if let Some(attachment) = file {
		let Some(content_type) = &attachment.content_type else {
			return Err("Welcome channel attachment was sent without a content type!".into());
		};

		if !content_type.starts_with("application/json;") {
			trace!("Not attempting to read non-json content type {content_type}");
			ctx.say("Invalid file! Please only send json").await?;
			return Ok(());
		}

		let downloaded = attachment.download().await?;
		String::from_utf8(downloaded)?
	} else if let Some(url) = url {
		api::text_from_url(&url).await?
	} else {
		ctx.say("A text file or URL must be provided!").await?;
		return Ok(());
	};

	// parse and create messages from file
	let welcome_layout: WelcomeLayout = serde_json::from_str(&file)?;
	let embed_messages: Vec<CreateMessage> = welcome_layout
		.embeds
		.iter()
		.map(|e| CreateMessage::from(e.clone()))
		.collect();
	let roles_messages: Vec<CreateMessage> = welcome_layout
		.roles
		.iter()
		.map(|c| CreateMessage::from(c.clone()))
		.collect();

	// clear previous messages
	let prev_messages: Vec<Message> = channel_id.messages_iter(ctx).try_collect().await?;
	channel_id.delete_messages(ctx, prev_messages).await?;

	// send our new ones
	for embed in embed_messages {
		channel_id.send_message(ctx, embed).await?;
	}

	for message in roles_messages {
		channel_id.send_message(ctx, message).await?;
	}

	for message in welcome_layout.messages {
		channel_id.say(ctx, message).await?;
	}

	if let Some(log_channel) = configured_channels.log_channel_id {
		let author = utils::embed_author_from_user(ctx.author());
		let embed = CreateEmbed::new()
			.title("set_welcome command used!")
			.author(author);
		let message = CreateMessage::new().embed(embed);

		log_channel.send_message(ctx, message).await?;
	} else {
		trace!("Not sending /set_welcome log as no channel is set");
	}

	ctx.reply(format!("Updated {}!", channel_id.mention()))
		.await?;

	Ok(())
}
