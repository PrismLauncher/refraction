use color_eyre::eyre::{eyre, Context as _, Result};
use poise::serenity_prelude::{Context, Message};

use crate::utils;

pub async fn handle(ctx: &Context, message: &Message) -> Result<()> {
    let embeds = utils::resolve_message(ctx, message).await?;

    // TOOD getchoo: actually reply to user
    // ...not sure why Message doesn't give me a builder in reply() or equivalents
    let our_channel = message
        .channel(ctx)
        .await
        .wrap_err_with(|| "Couldn't get channel from message!")?
        .guild()
        .ok_or_else(|| eyre!("Couldn't convert to GuildChannel!"))?;

    if !embeds.is_empty() {
        our_channel
            .send_message(ctx, |m| {
                m.set_embeds(embeds)
                    .allowed_mentions(|am| am.replied_user(false))
            })
            .await?;
    }

    Ok(())
}
