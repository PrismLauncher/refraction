use crate::consts::COLORS;
use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{Context, Message};

mod issues;
mod providers;

use issues::find_issues;
use providers::find_log;

pub async fn handle(ctx: &Context, message: &Message) -> Result<()> {
    let channel = message.channel_id;

    let log = find_log(message).await;

    if log.is_err() {
        channel
            .send_message(ctx, |m| {
                m.reference_message(message)
                    .allowed_mentions(|am| am.replied_user(true))
                    .embed(|e| {
                        e.title("Analyze failed!")
                            .description("Couldn't download log")
                    })
            })
            .await?;

        return Ok(());
    }

    let Some(log) = log? else {
        debug!("No log found in message! Skipping analysis");
        return Ok(());
    };

    let issues = find_issues(&log);

    channel
        .send_message(ctx, |m| {
            m.reference_message(message)
                .allowed_mentions(|am| am.replied_user(true))
                .embed(|e| {
                    e.title("Log analysis");

                    if issues.is_empty() {
                        e.color(COLORS["green"]).field(
                            "Analyze failed!",
                            "No issues found automatically",
                            false,
                        );
                    } else {
                        e.color(COLORS["red"]);

                        for (title, description) in issues {
                            e.field(title, description, false);
                        }
                    }

                    e
                })
        })
        .await?;

    Ok(())
}
