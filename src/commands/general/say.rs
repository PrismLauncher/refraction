use crate::Context;

use color_eyre::eyre::{eyre, Result};

#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn say(ctx: Context<'_>, content: String) -> Result<()> {
    let guild = ctx.guild().ok_or_else(|| eyre!("Couldn't get guild!"))?;
    let channel = ctx
        .guild_channel()
        .await
        .ok_or_else(|| eyre!("Couldn't get channel!"))?;

    channel.say(ctx, &content).await?;
    ctx.say("I said what you said!").await?;

    if let Some(channel_id) = ctx.data().config.discord.channels.say_log_channel_id {
        let log_channel = guild
            .channels
            .iter()
            .find(|c| c.0 == &channel_id)
            .ok_or_else(|| eyre!("Couldn't get log channel from guild!"))?;

        log_channel
            .1
            .clone()
            .guild()
            .ok_or_else(|| eyre!("Couldn't cast channel we found from guild as GuildChannel?????"))?
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Say command used!")
                        .description(content)
                        .author(|a| {
                            a.name(ctx.author().tag()).icon_url(
                                ctx.author().avatar_url().unwrap_or("undefined".to_string()),
                            )
                        })
                })
            })
            .await?;
    }

    Ok(())
}
