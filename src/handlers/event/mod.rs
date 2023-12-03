use crate::Data;

use color_eyre::eyre::{Report, Result};
use poise::serenity_prelude as serenity;
use poise::{Event, FrameworkContext};

pub async fn handle(
    ctx: &serenity::Context,
    event: &Event<'_>,
    framework: FrameworkContext<'_, Data, Report>,
    data: &Data,
) -> Result<()> {
    match event {
        Event::Ready { data_about_bot } => {
            log::info!("Logged in as {}!", data_about_bot.user.name)
        }

        Event::Message { new_message } => {}

        _ => {}
    }

    Ok(())
}
