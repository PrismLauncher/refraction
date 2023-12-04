use crate::Data;

use color_eyre::eyre::{Report, Result};
use poise::serenity_prelude::Context;
use poise::{Event, FrameworkContext};

mod delete;
mod eta;
mod expand_link;
mod support_onboard;

pub async fn handle(
    ctx: &Context,
    event: &Event<'_>,
    _framework: FrameworkContext<'_, Data, Report>,
    _data: &Data,
) -> Result<()> {
    match event {
        Event::Ready { data_about_bot } => {
            log::info!("Logged in as {}!", data_about_bot.user.name)
        }

        Event::Message { new_message } => {
            // ignore new messages from bots
            // NOTE: the webhook_id check allows us to still respond to PK users
            if new_message.author.bot && new_message.webhook_id.is_none() {
                return Ok(());
            }

            eta::handle(ctx, new_message).await?;
            expand_link::handle(ctx, new_message).await?;
        }

        Event::ReactionAdd { add_reaction } => delete::handle(ctx, add_reaction).await?,

        Event::ThreadCreate { thread } => support_onboard::handle(ctx, thread).await?,

        _ => {}
    }

    Ok(())
}
