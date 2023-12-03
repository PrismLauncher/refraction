use crate::Context;

use color_eyre::eyre::{eyre, Result};
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use serenity::{CreateEmbed, Message};
use url::Url;

#[macro_use]
mod macros;

/*
 * chooses a random element from an array
 */
pub fn random_choice<const N: usize>(arr: [&str; N]) -> Result<String> {
    let mut rng = rand::thread_rng();
    let resp = arr
        .choose(&mut rng)
        .ok_or_else(|| eyre!("Couldn't choose random object from array:\n{arr:#?}!"))?;

    Ok((*resp).to_string())
}

// waiting for `round_char_boundary` to stabilize
pub fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let lower_bound = index.saturating_sub(3);
        let new_index = s.as_bytes()[lower_bound..=index]
            .iter()
            .rposition(|&b| (b as i8) >= -0x40); // b.is_utf8_char_boundary

        // Can be made unsafe but whatever
        lower_bound + new_index.unwrap()
    }
}

pub async fn send_url_as_embed(ctx: Context<'_>, url: String) -> Result<()> {
    let parsed = Url::parse(&url)?;

    let title = parsed
        .path_segments()
        .unwrap()
        .last()
        .unwrap_or("image")
        .replace("%20", " ");

    ctx.send(|c| c.embed(|e| e.title(title).image(&url).url(url)))
        .await?;

    Ok(())
}

pub async fn resolve_message_to_embed(ctx: &serenity::Context, msg: &Message) -> CreateEmbed {
    let truncation_point = floor_char_boundary(&msg.content, 700);
    let truncated_content = if msg.content.len() <= truncation_point {
        msg.content.to_string()
    } else {
        format!("{}...", &msg.content[..truncation_point])
    };

    let color = msg
        .member(ctx)
        .await
        .ok()
        .and_then(|m| m.highest_role_info(&ctx.cache))
        .and_then(|(role, _)| role.to_role_cached(&ctx.cache))
        .map(|role| role.colour);

    let attached_image = msg
        .attachments
        .iter()
        .filter(|a| {
            a.content_type
                .as_ref()
                .filter(|ct| ct.contains("image/"))
                .is_some()
        })
        .map(|a| &a.url)
        .next();

    let attachments_len = msg.attachments.len();

    let mut embed = msg
        .embeds
        .first()
        .map(|embed| CreateEmbed::from(embed.clone()))
        .unwrap_or_default();

    embed.author(|author| author.name(&msg.author.name).icon_url(&msg.author.face()));

    if let Some(color) = color {
        embed.color(color);
    }

    if let Some(attachment) = attached_image {
        embed.image(attachment);
    }

    if attachments_len > 1 {
        embed.footer(|footer| {
            // yes it will say '1 attachments' no i do not care
            footer.text(format!("{} attachments", attachments_len))
        });
    }

    embed.description(truncated_content);
    embed
}
