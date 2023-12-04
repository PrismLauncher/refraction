use color_eyre::eyre::{Context as _, Result};
use poise::serenity_prelude::{Context, InteractionType, Reaction};

pub async fn handle(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let user = reaction
        .user(ctx)
        .await
        .wrap_err_with(|| "Couldn't fetch user from reaction!")?;

    let message = reaction
        .message(ctx)
        .await
        .wrap_err_with(|| "Couldn't fetch message from reaction!")?;

    if let Some(interaction) = &message.interaction {
        if interaction.kind == InteractionType::ApplicationCommand
            && interaction.user == user
            && reaction.emoji.unicode_eq("❌")
        {
            message.delete(ctx).await?;
        }
    }

    Ok(())
}
