use crate::consts::COLORS;
use crate::Data;

use color_eyre::eyre::Report;
use log::*;
use poise::serenity_prelude::Timestamp;
use poise::FrameworkError;

pub async fn handle(error: FrameworkError<'_, Data, Report>) {
    match error {
        FrameworkError::Setup { error, .. } => error!("Error setting up client!\n{error:#?}"),

        FrameworkError::Command { error, ctx } => {
            error!("Error in command {}:\n{error:?}", ctx.command().name);
            ctx.send(|c| {
                c.embed(|e| {
                    e.title("Something went wrong!")
                        .description("oopsie")
                        .timestamp(Timestamp::now())
                        .color(COLORS["red"])
                })
            })
            .await
            .ok();
        }

        FrameworkError::EventHandler {
            error,
            ctx: _,
            event,
            framework: _,
        } => {
            error!("Error while handling event {}:\n{error:?}", event.name());
        }

        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Unhandled error occured:\n{e:#?}");
            }
        }
    }
}
