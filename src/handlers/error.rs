use crate::{consts::Colors, Data, Error};

use std::fmt::Write;

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

pub async fn handle(error: FrameworkError<'_, Data, Error>) {
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
				.color(Colors::Red);

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

		FrameworkError::CommandCheckFailed { error, ctx, .. } => {
			if let Some(error) = error {
				// just log - it's probably best if people don't find out when they're breaking the perm checking
				log::error!(
					"Error checking permissions for {}:\n{error:?}",
					ctx.command().name
				);
			} else if let poise::Context::Application(ctx) = ctx {
				// only show for application commands - for prefix commands there is no way to hide the response and avoid spam
				ctx.send(
					poise::CreateReply::default()
						.content("❌ You're not allowed to use this command.")
						.ephemeral(true),
				)
				.await
				.ok();
			}
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("Unhandled error occurred:\n{e:#?}");
			}
		}
	}
}
