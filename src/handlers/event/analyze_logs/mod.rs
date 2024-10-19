use std::{
	collections::HashSet,
	sync::{Mutex, OnceLock},
};

use crate::{
	api::{mclogs, HttpClientExt},
	consts::Colors,
	utils::first_text_attachment,
	Data,
};

use color_eyre::owo_colors::OwoColorize;
use eyre::{eyre, OptionExt, Result};
use log::{debug, trace};
use poise::serenity_prelude::{
	ButtonStyle, ComponentInteraction, Context, CreateAllowedMentions, CreateButton, CreateEmbed,
	CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EditMessage,
	Message, MessageId, MessageType,
};

mod heuristics;
mod issues;
mod providers;

use providers::find_log;

const BUTTON_UPLOAD_YES: &str = "log-upload-yes";
const BUTTON_UPLOAD_NO: &str = "log-upload-no";

pub async fn handle_message(ctx: &Context, message: &Message, data: &Data) -> Result<()> {
	trace!(
		"Checking message {} from {} for logs",
		message.id,
		message.author.id
	);
	let channel = message.channel_id;

	let Ok(log) = find_log(&data.http_client, message).await else {
		let embed = CreateEmbed::new()
			.title("Analysis failed!")
			.description("Couldn't download log");
		let allowed_mentions = CreateAllowedMentions::new().replied_user(true);
		let our_message = CreateMessage::new()
			.reference_message(message)
			.allowed_mentions(allowed_mentions)
			.embed(embed);

		channel.send_message(ctx, our_message).await?;

		return Ok(());
	};

	let attachment = first_text_attachment(message);

	let log = match log {
		Some(log) => log,
		None => match attachment {
			Some(attachment) => {
				data.http_client
					.get_request(&attachment.url)
					.await?
					.text()
					.await?
			}
			None => {
				debug!("No log found in message! Skipping analysis");
				return Ok(());
			}
		},
	};

	let issues = issues::find(&log, data).await?;
	let launcher_log = heuristics::looks_like_launcher_log(&log);
	let mc_log = !launcher_log && heuristics::looks_like_mc_log(&log);

	debug!("Detections: mc_log = {mc_log}, launcher_log = {launcher_log}");

	let show_analysis = !issues.is_empty() || mc_log;
	let show_upload_prompt = attachment.is_some() && (mc_log || launcher_log);

	if !show_analysis && !show_upload_prompt {
		debug!("Found log but there is nothing to respond with");
		return Ok(());
	}

	let allowed_mentions = CreateAllowedMentions::new().replied_user(true);

	let mut message = CreateMessage::new()
		.reference_message(message)
		.allowed_mentions(allowed_mentions);

	if show_analysis {
		message = message.add_embed({
			let mut e = CreateEmbed::new().title("Log analysis");

			if issues.is_empty() {
				e = e
					.color(Colors::Green)
					.description("The automatic check didn't reveal any issues, but it's possible that some issues went undetected. Please wait for a volunteer to assist you.");
			} else {
				e = e.color(Colors::Red);

				for (title, description) in issues {
					e = e.field(title, description, false);
				}
			}

			e
		});
	}

	if show_upload_prompt {
		message = message.add_embed(
			CreateEmbed::new()
				.title("Upload log?")
				.color(Colors::Blue)
				.description("Discord attachments make it difficult for volunteers to view logs. Would you like me to upload your log to [mclo.gs](https://mclo.gs/)?"),
		);
		message = message
			.button(
				CreateButton::new(BUTTON_UPLOAD_NO)
					.style(ButtonStyle::Secondary)
					.label("No"),
			)
			.button(
				CreateButton::new(BUTTON_UPLOAD_YES)
					.style(ButtonStyle::Primary)
					.label("Yes"),
			);
	}

	channel.send_message(ctx, message).await?;

	Ok(())
}

pub async fn handle_component_interaction(
	ctx: &Context,
	interaction: &ComponentInteraction,
	data: &Data,
) -> Result<()> {
	if interaction.message.kind != MessageType::InlineReply {
		debug!("Ignoring component interaction on message which is not a reply");
		return Ok(());
	}

	let yes = interaction.data.custom_id == BUTTON_UPLOAD_YES;

	if !yes && interaction.data.custom_id != BUTTON_UPLOAD_NO {
		debug!(
			"Ignoring component interaction without ID {BUTTON_UPLOAD_YES} or {BUTTON_UPLOAD_NO}"
		);
		return Ok(());
	}

	let mut embeds: Vec<CreateEmbed> = interaction
		.message
		.embeds
		.iter()
		.map(|embed| CreateEmbed::from(embed.to_owned()))
		.collect();

	// for some reason Discord never sends us the referenced message, only its id
	let message_reference = interaction
		.message
		.message_reference
		.as_ref()
		.ok_or_eyre("Missing message reference")?;

	let referenced_message = ctx
		.http
		.get_message(
			message_reference.channel_id,
			message_reference
				.message_id
				.ok_or_eyre("Reference missing message ID")?,
		)
		.await;

	let Ok(referenced_message) = referenced_message else {
		// TODO: make the bot delete its response when the initial message is deleted
		debug!("Ignoring component interaction on reply to deleted message");
		return Ok(());
	};

	// prevent other members from clicking the buttons
	if interaction.user.id != referenced_message.author.id {
		debug!("Ignoring component interaction by {} on reply to message by {}", interaction.user.id, referenced_message.author.id);
		return Ok(());
	}

	if yes {
		let first_attachment = first_text_attachment(&referenced_message)
			.ok_or_eyre("Log attachment disappeared - should not be possible!")?;
		let body = data
			.http_client
			.get_request(&first_attachment.url)
			.await?
			.text()
			.await?;

		let response = mclogs::upload_log(&data.http_client, &body).await?;

		if !response.success {
			let error = response
				.error
				.ok_or_else(|| eyre!("mclo.gs gave us an error but with no message!"))?;

			interaction
				.create_response(
					&ctx.http,
					CreateInteractionResponse::Message(
						CreateInteractionResponseMessage::new()
							.ephemeral(true)
							.embed(
								CreateEmbed::new()
									.title("Upload failed")
									.color(Colors::Red)
									.description(&error),
							),
					),
				)
				.await?;

			return Err(eyre!("Failed to upload log: {}", &error));
		}

		let url = &response
			.url
			.ok_or_eyre("Missing URL in mclo.gs response!")?;

		let length = embeds.len();

		embeds[length - 1] = CreateEmbed::new()
			.title("Uploaded log")
			.color(Colors::Blue)
			.description(url);
	} else {
		embeds.pop();
	}

	interaction
		.create_response(ctx, CreateInteractionResponse::Acknowledge)
		.await?;

	if embeds.len() == 0 {
		interaction.message.delete(ctx).await?;
	} else {
		ctx.http
			.edit_message(
				interaction.channel_id,
				interaction.message.id,
				&EditMessage::new().embeds(embeds).components(vec![]),
				vec![],
			)
			.await?;
	}

	Ok(())
}
