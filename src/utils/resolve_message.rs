use std::str::FromStr;

use eyre::{eyre, Context as _, Result};
use log::{debug, trace};
use once_cell::sync::Lazy;
use poise::serenity_prelude::{
	ChannelId, ChannelType, Colour, Context, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
	Message, MessageId,
};
use regex::Regex;

static MESSAGE_PATTERN: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"/(https?:\/\/)?(?:canary\.|ptb\.)?discord(?:app)?\.com\/channels\/(?<serverId>\d+)\/(?<channelId>\d+)\/(?<messageId>\d+)/g;").unwrap()
});

fn find_first_image(msg: &Message) -> Option<String> {
	msg.attachments
		.iter()
		.find(|a| {
			a.content_type
				.as_ref()
				.unwrap_or(&String::new())
				.starts_with("image/")
		})
		.map(|res| res.url.clone())
}

pub async fn resolve(ctx: &Context, msg: &Message) -> Result<Vec<CreateEmbed>> {
	let matches = MESSAGE_PATTERN
		.captures_iter(&msg.content)
		.map(|capture| capture.extract());

	let mut embeds: Vec<CreateEmbed> = vec![];

	for (url, [_server_id, channel_id, message_id]) in matches {
		trace!("Attempting to resolve message {message_id} from URL {url}");

		let channel = ChannelId::from_str(channel_id)
			.wrap_err_with(|| format!("Couldn't parse channel ID {channel_id}!"))?
			.to_channel_cached(ctx.as_ref())
			.ok_or_else(|| eyre!("Couldn't find Guild Channel from {channel_id}!"))?
			.to_owned();

		let author_can_view = if channel.kind == ChannelType::PublicThread
			|| channel.kind == ChannelType::PrivateThread
		{
			let thread_members = channel
				.id
				.get_thread_members(ctx)
				.await
				.wrap_err("Couldn't get members from thread!")?;

			thread_members
				.iter()
				.any(|member| member.user_id == msg.author.id)
		} else {
			channel
				.members(ctx)
				.wrap_err_with(|| format!("Couldn't get members for channel {channel_id}!"))?
				.iter()
				.any(|member| member.user.id == msg.author.id)
		};

		if !author_can_view {
			debug!("Not resolving message for author who can't see it");
		}

		let original_message = channel
			.message(
				ctx,
				MessageId::from_str(message_id)
					.wrap_err_with(|| format!("Couldn't parse message ID {message_id}!"))?,
			)
			.await
			.wrap_err_with(|| {
				format!("Couldn't get message from ID {message_id} in channel {channel_id}!")
			})?;

		let author = CreateEmbedAuthor::new(original_message.author.tag())
			.icon_url(original_message.author.default_avatar_url());
		let footer = CreateEmbedFooter::new(format!("#{}", channel.name));

		let mut embed = CreateEmbed::new()
			.author(author)
			.color(Colour::BLITZ_BLUE)
			.timestamp(original_message.timestamp)
			.footer(footer)
			.description(format!(
				"{}\n\n[Jump to original message]({})",
				original_message.content,
				original_message.link()
			));

		if !original_message.attachments.is_empty() {
			embed = embed.fields(original_message.attachments.iter().map(|a| {
				(
					"Attachments".to_string(),
					format!("[{}]({})", a.filename, a.url),
					false,
				)
			}));

			if let Some(image) = find_first_image(msg) {
				embed = embed.image(image);
			}
		}

		embeds.push(embed);
	}

	Ok(embeds)
}
