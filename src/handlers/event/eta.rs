use crate::{consts, utils};

use color_eyre::eyre::Result;
use poise::serenity_prelude::{Context, Message};

pub async fn handle_eta(ctx: &Context, message: &Message) -> Result<()> {
    if !message.content.contains(" eta ") {
        return Ok(());
    }

    let response = format!(
        "{} <:pofat:1031701005559144458>",
        utils::random_choice(consts::ETA_MESSAGES)?
    );

    message.reply(ctx, response).await?;
    Ok(())
}
