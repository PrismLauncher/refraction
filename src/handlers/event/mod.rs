use crate::{api, Data};

use color_eyre::eyre::{Report, Result};
use log::*;
use poise::serenity_prelude::{Activity, Context, OnlineStatus};
use poise::{Event, FrameworkContext};

mod analyze_logs;
mod delete_on_reaction;
mod eta;
mod expand_link;
mod message_logger;
pub mod pluralkit;
mod support_onboard;

pub async fn handle(
    ctx: &Context,
    event: &Event<'_>,
    _framework: FrameworkContext<'_, Data, Report>,
    data: &Data,
) -> Result<()> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("Logged in as {}!", data_about_bot.user.name);

            let latest_minecraft_version = api::prism_meta::get_latest_minecraft_version().await?;
            let activity = Activity::playing(format!("Minecraft {}", latest_minecraft_version));

            info!("Setting presence to activity {activity:#?}");
            ctx.set_presence(Some(activity), OnlineStatus::Online).await;
        }

        Event::Message { new_message } => {
            // ignore new messages from bots
            // NOTE: the webhook_id check allows us to still respond to PK users
            if new_message.author.bot && new_message.webhook_id.is_none() {
                debug!("Ignoring message {} from bot", new_message.id);
                return Ok(());
            }

            // detect PK users first to make sure we don't respond to unproxied messages
            pluralkit::handle(ctx, new_message, data).await?;

            if data.storage.is_user_plural(new_message.author.id).await?
                && pluralkit::is_message_proxied(new_message).await?
            {
                debug!("Not replying to unproxied PluralKit message");
                return Ok(());
            }

            // store all new messages to monitor edits and deletes
            message_logger::handle_create(data, new_message).await?;

            eta::handle(ctx, new_message).await?;
            expand_link::handle(ctx, new_message).await?;
            analyze_logs::handle(ctx, new_message).await?;
        }

        Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id: _,
        } => message_logger::handle_delete(ctx, data, channel_id, deleted_message_id).await?,

        Event::MessageUpdate {
            old_if_available: _,
            new: _,
            event,
        } => {
            message_logger::handle_update(data, event).await?;
        }

        Event::ReactionAdd { add_reaction } => {
            delete_on_reaction::handle(ctx, add_reaction).await?
        }

        Event::ThreadCreate { thread } => support_onboard::handle(ctx, thread).await?,

        _ => {}
    }

    Ok(())
}
