use crate::{Context, Error};

use log::trace;
use poise::samples::HelpConfiguration;

/// View the help menu
#[poise::command(slash_command, prefix_command, track_edits = true)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Provide information about a specific command"] command: Option<String>,
) -> Result<(), Error> {
	trace!("Running help command");

	let configuration = HelpConfiguration {
		extra_text_at_bottom: "Use /help for more information about a specific command!",
		..Default::default()
	};

	poise::builtins::help(ctx, command.as_deref(), configuration).await?;

	Ok(())
}
