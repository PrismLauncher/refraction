use std::str::FromStr;

use eyre::Result;
use log::debug;
use poise::serenity_prelude::{
	ComponentInteraction, Context, CreateEmbed, CreateInteractionResponseFollowup, RoleId,
};

pub async fn handle(ctx: &Context, component_interaction: &ComponentInteraction) -> Result<()> {
	let Some(guild_id) = component_interaction.guild_id else {
		debug!("Ignoring component interaction not from guild!");
		return Ok(());
	};

	let Ok(role_id) = RoleId::from_str(&component_interaction.data.custom_id) else {
		debug!("Ignoring component interaction that doesn't contain a role as it's ID");
		return Ok(());
	};

	component_interaction.defer_ephemeral(ctx).await?;

	let mut followup = CreateInteractionResponseFollowup::new().ephemeral(true);
	if let Some(role) = guild_id.roles(ctx).await?.get(&role_id) {
		let guild_member = guild_id.member(ctx, component_interaction.user.id).await?;

		let mut embed = CreateEmbed::new();
		if guild_member.roles.contains(&role_id) {
			guild_member.remove_role(ctx, role_id).await?;
			embed = embed.description(format!("❌ Removed `{}`", role.name));
		} else {
			guild_member.add_role(ctx, role_id).await?;
			embed = embed.description(format!("✅ Added `{}`", role.name));
		}

		followup = followup.add_embed(embed);
	} else {
		followup = followup.content(format!(
			"Role ID {role_id} doesn't seem to exist. Please let the moderators know!"
		));
	}

	component_interaction.create_followup(ctx, followup).await?;

	Ok(())
}
