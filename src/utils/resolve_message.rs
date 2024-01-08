use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{ChannelType, Colour, Context, CreateEmbed, Message};
use regex::Regex;

static MESSAGE_PATTERN: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"/(https?:\/\/)?(?:canary\.|ptb\.)?discord(?:app)?\.com\/channels\/(?<serverId>\d+)\/(?<channelId>\d+)\/(?<messageId>\d+)/g;").unwrap()
});

pub fn find_first_image(msg: &Message) -> Option<String> {
	msg.attachments
		.iter()
		.find(|a| {
			a.content_type
				.as_ref()
				.unwrap_or(&"".to_string())
				.starts_with("image/")
		})
		.map(|res| res.url.clone())
}

pub async fn resolve(ctx: &Context, msg: &Message) -> Result<Vec<CreateEmbed>> {
	let matches = MESSAGE_PATTERN.captures_iter(&msg.content);
	let mut embeds: Vec<CreateEmbed> = vec![];

	for captured in matches.take(3) {
		// don't leak messages from other servers
		if let Some(server_id) = captured.get(0) {
			let other_server: u64 = server_id.as_str().parse().unwrap_or_default();
			let current_id = msg.guild_id.unwrap_or_default();

			if &other_server != current_id.as_u64() {
				debug!("Not resolving message of other guild.");
				continue;
			}
		} else {
			warn!("Couldn't find server_id from Discord link! Not resolving message to be safe");
			continue;
		}

		if let Some(channel_id) = captured.get(1) {
			let parsed: u64 = channel_id.as_str().parse().unwrap_or_default();
			let req_channel = ctx
				.cache
				.channel(parsed)
				.ok_or_else(|| eyre!("Couldn't get channel_id from Discord regex!"))?
				.guild()
				.ok_or_else(|| {
					eyre!("Couldn't convert to GuildChannel from channel_id {parsed}!")
				})?;

			if !req_channel.is_text_based() {
				debug!("Not resolving message is non-text-based channel.");
				continue;
			}

			if req_channel.kind == ChannelType::PrivateThread {
				if let Some(id) = req_channel.parent_id {
					let parent = ctx.cache.guild_channel(id).ok_or_else(|| {
						eyre!("Couldn't get parent channel {id} for thread {req_channel}!")
					})?;
					let parent_members = parent.members(ctx).await.unwrap_or_default();

					if !parent_members.iter().any(|m| m.user.id == msg.author.id) {
						debug!("Not resolving message for user not a part of a private thread.");
						continue;
					}
				}
			} else if req_channel
				.members(ctx)
				.await?
				.iter()
				.any(|m| m.user.id == msg.author.id)
			{
				debug!("Not resolving for message for user not a part of a channel");
				continue;
			}

			let message_id: u64 = captured
				.get(2)
				.ok_or_else(|| eyre!("Couldn't get message_id from Discord regex!"))?
				.as_str()
				.parse()
				.wrap_err_with(|| {
					eyre!("Couldn't parse message_id from Discord regex as a MessageId!")
				})?;

			let original_message = req_channel.message(ctx, message_id).await?;
			let mut embed = CreateEmbed::default();
			embed
				.author(|a| {
					a.name(original_message.author.tag())
						.icon_url(original_message.author.default_avatar_url())
				})
				.color(Colour::BLITZ_BLUE)
				.timestamp(original_message.timestamp)
				.footer(|f| f.text(format!("#{}", req_channel.name)))
				.description(format!(
					"{}\n\n[Jump to original message]({})",
					original_message.content,
					original_message.link()
				));

			if !original_message.attachments.is_empty() {
				embed.fields(original_message.attachments.iter().map(|a| {
					(
						"Attachments".to_string(),
						format!("[{}]({})", a.filename, a.url),
						false,
					)
				}));

				if let Some(image) = find_first_image(msg) {
					embed.image(image);
				}
			}

			embeds.push(embed);
		}
	}

	Ok(embeds)
}
