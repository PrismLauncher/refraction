use crate::consts;
use crate::Data;
use std::fmt::Write;

use eyre::Report;
use log::error;
use poise::serenity_prelude::{CreateEmbed, Timestamp};
use poise::{CreateReply, FrameworkError};

// getchoo: i like writeln! and don't like
macro_rules! writelne {
	($dst:expr, $($arg:tt)*) => {
		if let Err(why) = writeln!($dst, $($arg)*) {
			error!("We somehow cannot write to what should be on the heap. What are you using this macro with? Anyways, here's the error:\n{why:#?}");
		}
	}
}

pub async fn handle(error: FrameworkError<'_, Data, Report>) {
	match error {
		FrameworkError::Setup {
			error, framework, ..
		} => {
			error!("Error setting up client! Bailing out");
			framework.shard_manager().shutdown_all().await;

			panic!("{error}")
		}

		FrameworkError::Command { error, ctx, .. } => {
			error!("Error in command {}:\n{error:?}", ctx.command().name);

			let embed = CreateEmbed::new()
				.title("Something went wrong!")
				.description("oopsie")
				.timestamp(Timestamp::now())
				.color(consts::COLORS["red"]);

			let reply = CreateReply::default().embed(embed);

			ctx.send(reply).await.ok();
		}

		FrameworkError::EventHandler {
			error,
			ctx: _,
			event,
			framework: _,
			..
		} => {
			error!(
				"Error while handling event {}:\n{error:?}",
				event.snake_case_name()
			);
		}

		FrameworkError::ArgumentParse {
			error, input, ctx, ..
		} => {
			let mut response = String::new();

			if let Some(input) = input {
				writelne!(
					&mut response,
					"**Cannot parse `{input}` as argument: {error}**\n"
				);
			} else {
				writelne!(&mut response, "**{error}**\n");
			}

			if let Some(help_text) = ctx.command().help_text.as_ref() {
				writelne!(&mut response, "{help_text}\n");
			}

			if ctx.command().invoke_on_edit {
				writelne!(
					&mut response,
					"**Tip:** Edit your message to update the response."
				);
			}

			writelne!(
				&mut response,
				"For more information, refer to /help {}.",
				ctx.command().name
			);

			if let Err(why) = ctx.say(response).await {
				error!("Unhandled error displaying ArgumentParse error\n{why:#?}");
			}
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("Unhandled error occurred:\n{e:#?}");
			}
		}
	}
}
