use crate::{consts, Context};

use color_eyre::eyre::{eyre, Result};

/// Returns the number of members in the server
#[poise::command(slash_command, prefix_command)]
pub async fn members(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().ok_or_else(|| eyre!("Couldn't fetch guild!"))?;

    let count = guild.member_count;
    let online = if let Some(count) = guild.approximate_presence_count {
        count.to_string()
    } else {
        "Undefined".to_string()
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("{count} total members!"))
                .description(format!("{online} online members"))
                .color(consts::COLORS["blue"])
        })
    })
    .await?;
    Ok(())
}
