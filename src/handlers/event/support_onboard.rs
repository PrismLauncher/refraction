use color_eyre::eyre::{eyre, Result};
use log::*;
use poise::serenity_prelude::{ChannelType, Context, GuildChannel};

pub async fn handle(ctx: &Context, thread: &GuildChannel) -> Result<()> {
    if thread.kind != ChannelType::PublicThread {
        return Ok(());
    }

    let parent_id = thread
        .parent_id
        .ok_or_else(|| eyre!("Couldn't get parent ID from thread {}!", thread.name))?;

    let parent_channel = ctx
        .cache
        .guild_channel(parent_id)
        .ok_or_else(|| eyre!("Couldn't get GuildChannel {}!", parent_id))?;

    if parent_channel.name != "support" {
        debug!("Not posting onboarding message to threads outside of support");
        return Ok(());
    }

    let owner = thread
        .owner_id
        .ok_or_else(|| eyre!("Couldn't get owner of thread!"))?;

    let msg = format!(
    "<@{}> We've received your support ticket! {} {}",
    owner,
    "Please upload your logs and post the link here if possible (run `tag log` to find out how).",
    "Please don't ping people for support questions, unless you have their permission."
    );

    thread
        .send_message(ctx, |m| {
            m.content(msg)
                .allowed_mentions(|am| am.replied_user(true).users(Vec::from([owner])))
        })
        .await?;

    Ok(())
}
