use crate::{consts, utils};

use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{Context, Message};
use regex::Regex;

static ETA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\beta\b").unwrap());

pub async fn handle(ctx: &Context, msg: &Message) -> Result<()> {
    if !ETA_REGEX.is_match(&msg.content) {
        return Ok(());
    }

    let response = format!(
        "{} <:pofat:1031701005559144458>",
        utils::random_choice(consts::ETA_MESSAGES)?
    );

    msg.reply(ctx, response).await?;
    Ok(())
}
