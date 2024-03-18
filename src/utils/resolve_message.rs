use std::str::FromStr;

use eyre::{eyre, Context as _, Result};
use log::{debug, trace};
use once_cell::sync::Lazy;
use poise::serenity_prelude::{
	Cache, CacheHttp, ChannelId, ChannelType, Colour, Context, CreateEmbed, CreateEmbedAuthor,
	CreateEmbedFooter, GuildChannel, Member, Message, MessageId, Permissions,
};
use regex::Regex;

fn find_first_image(message: &Message) -> Option<String> {
	message
		.attachments
		.iter()
		.find(|a| {
			a.content_type
				.as_ref()
				.unwrap_or(&String::new())
				.starts_with("image/")
		})
		.map(|res| res.url.clone())
}

async fn member_can_view_channel(
	ctx: impl CacheHttp + AsRef<Cache>,
	member: &Member,
	channel: &GuildChannel,
) -> Result<bool> {
	static REQUIRED_PERMISSIONS: Lazy<Permissions> =
		Lazy::new(|| Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY);

	let guild = ctx.http().get_guild(channel.guild_id).await?;

	let channel_to_check = match &channel.kind {
		ChannelType::PublicThread => {
			let parent_id = channel
				.parent_id
				.ok_or_else(|| eyre!("Couldn't get parent of thread {}", channel.id))?;
			parent_id
				.to_channel(ctx)
				.await?
				.guild()
				.ok_or_else(|| eyre!("Couldn't get GuildChannel from ChannelID {parent_id}!"))?
		}

		ChannelType::Text | ChannelType::News => channel.to_owned(),

		_ => return Ok(false),
	};

	let can_view = guild
		.user_permissions_in(&channel_to_check, member)
		.contains(*REQUIRED_PERMISSIONS);
	Ok(can_view)
}

pub async fn to_embed(
	ctx: impl CacheHttp + AsRef<Cache>,
	message: &Message,
) -> Result<CreateEmbed> {
	let author = CreateEmbedAuthor::new(message.author.tag()).icon_url(
		message
			.author
			.avatar_url()
			.unwrap_or_else(|| message.author.default_avatar_url()),
	);

	let footer = CreateEmbedFooter::new(format!(
		"#{}",
		message.channel(ctx).await?.guild().unwrap_or_default().name
	));

	let mut embed = CreateEmbed::new()
		.author(author)
		.color(Colour::BLITZ_BLUE)
		.timestamp(message.timestamp)
		.footer(footer)
		.description(format!(
			"{}\n\n[Jump to original message]({})",
			message.content,
			message.link()
		));

	if !message.attachments.is_empty() {
		embed = embed.fields(message.attachments.iter().map(|a| {
			(
				"Attachments".to_string(),
				format!("[{}]({})", a.filename, a.url),
				false,
			)
		}));

		if let Some(image) = find_first_image(message) {
			embed = embed.image(image);
		}
	}

	Ok(embed)
}

pub async fn from_message(ctx: &Context, msg: &Message) -> Result<Vec<CreateEmbed>> {
	static MESSAGE_PATTERN: Lazy<Regex> = Lazy::new(|| {
		Regex::new(r"(?:https?:\/\/)?(?:canary\.|ptb\.)?discord(?:app)?\.com\/channels\/(?<server_id>\d+)\/(?<channel_id>\d+)\/(?<message_id>\d+)").unwrap()
	});

	let Some(guild_id) = msg.guild_id else {
		debug!("Not resolving message in DM");
		return Ok(Vec::new());
	};
	let author = guild_id.member(ctx, msg.author.id).await?;

	let matches = MESSAGE_PATTERN
		.captures_iter(&msg.content)
		.map(|capture| capture.extract());

	let mut embeds: Vec<CreateEmbed> = vec![];

	for (url, [target_guild_id, target_channel_id, target_message_id]) in matches {
		if target_guild_id != guild_id.to_string() {
			debug!("Not resolving message from other server");
			continue;
		}
		trace!("Attempting to resolve message {target_message_id} from URL {url}");

		let target_channel = ChannelId::from_str(target_channel_id)?
			.to_channel(ctx)
			.await?
			.guild()
			.ok_or_else(|| {
				eyre!("Couldn't find GuildChannel from ChannelId {target_channel_id}!")
			})?;

		if !member_can_view_channel(ctx, &author, &target_channel).await? {
			debug!("Not resolving message for author who can't see it");
			continue;
		}

		let target_message_id = MessageId::from_str(target_message_id)?;
		let target_message = target_channel
			.message(ctx, target_message_id)
			.await
			.wrap_err_with(|| {
				eyre!("Couldn't find channel message from ID {target_message_id}!")
			})?;

		let embed = to_embed(ctx, &target_message).await?;

		embeds.push(embed);
	}

	Ok(embeds)
}
