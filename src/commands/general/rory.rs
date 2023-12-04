use crate::api::rory::get_rory;
use crate::Context;

use color_eyre::eyre::Result;

/// Gets a Rory photo!
#[poise::command(slash_command, prefix_command)]
pub async fn rory(
    ctx: Context<'_>,
    #[description = "specify a Rory ID"] id: Option<u64>,
) -> Result<()> {
    let resp = get_rory(id).await?;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Rory :3")
                .url(&resp.url)
                .image(resp.url)
                .footer(|f| f.text(format!("ID {}", resp.id)))
        })
    })
    .await?;

    Ok(())
}
