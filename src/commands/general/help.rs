use crate::{commands::Command, consts::Colors, Context, Error};

use log::trace;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// View the help menu
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Provide information about a specific command"] command: Option<String>,
) -> Result<(), Error> {
	trace!("Running help command");

	let commands = &ctx.framework().options().commands;
	let entries = collect_help_entries(commands);

	let embed = if let Some(command_query) = command {
		build_single_command_embed(&entries, &command_query)
	} else {
		build_all_commands_embed(&entries)
	};

	ctx.send(CreateReply::default().embed(embed)).await?;

	Ok(())
}

#[derive(Clone, Debug)]
struct HelpEntry {
	name: String,
	description: String,
	help_text: Option<String>,
}

fn collect_help_entries(commands: &[Command]) -> Vec<HelpEntry> {
	let mut entries = Vec::new();
	collect_help_entries_recursive(commands, None, &mut entries);
	entries.sort_by(|left, right| left.name.cmp(&right.name));
	entries
}

fn collect_help_entries_recursive(
	commands: &[Command],
	parent: Option<&str>,
	entries: &mut Vec<HelpEntry>,
) {
	for command in commands {
		if command.hide_in_help {
			continue;
		}

		let full_name = match parent {
			Some(parent_name) => format!("{parent_name} {}", command.name),
			None => command.name.clone(),
		};

		let can_invoke = command.prefix_action.is_some() || command.slash_action.is_some();

		if can_invoke {
			entries.push(HelpEntry {
				name: full_name.clone(),
				description: command
					.description
					.clone()
					.unwrap_or_else(|| "No description provided.".to_string()),
				help_text: command.help_text.clone(),
			});
		}

		if !command.subcommands.is_empty() {
			collect_help_entries_recursive(&command.subcommands, Some(&full_name), entries);
		}
	}
}

fn build_all_commands_embed(entries: &[HelpEntry]) -> CreateEmbed {
	let description = if entries.is_empty() {
		"No commands are available.".to_string()
	} else {
		entries
			.iter()
			.map(|entry| format!("`/{}` — {}", entry.name, entry.description))
			.collect::<Vec<String>>()
			.join("\n")
	};

	CreateEmbed::new()
		.title("Refraction Help")
		.description(description)
		.color(Colors::Blue)
		.footer(poise::serenity_prelude::CreateEmbedFooter::new(
			"Use /help <command> for details about one command.",
		))
}

fn build_single_command_embed(entries: &[HelpEntry], query: &str) -> CreateEmbed {
	let normalized_query = query.trim().trim_start_matches('/').to_ascii_lowercase();

	if let Some(entry) = entries
		.iter()
		.find(|entry| entry.name.to_ascii_lowercase() == normalized_query)
	{
		let mut embed = CreateEmbed::new()
			.title(format!("📘 /{}", entry.name))
			.color(Colors::Blue)
			.field("Description", &entry.description, false);

		if let Some(help_text) = &entry.help_text {
			embed = embed.field("Details", help_text, false);
		}

		embed
	} else {
		CreateEmbed::new()
			.title("Command not found")
			.description(format!(
				"I couldn't find a command named `/{}`. Try `/help` to see all commands.",
				normalized_query
			))
			.color(Colors::Red)
	}
}
