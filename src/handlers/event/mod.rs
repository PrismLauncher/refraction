use crate::Data;

use color_eyre::eyre::{Report, Result};
use poise::serenity_prelude::Context;
use poise::{Event, FrameworkContext};

mod eta;

pub async fn handle(
    ctx: &Context,
    event: &Event<'_>,
    framework: FrameworkContext<'_, Data, Report>,
    data: &Data,
) -> Result<()> {
    match event {
        Event::Ready { data_about_bot } => {
            log::info!("Logged in as {}!", data_about_bot.user.name)
        }

        Event::Message { new_message } => eta::handle_eta(ctx, new_message).await?,

        _ => {}
    }

    Ok(())
}
