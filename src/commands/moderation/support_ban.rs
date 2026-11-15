#![allow(clippy::unreadable_literal)]

use poise::serenity_prelude::{Member, RoleId};

use crate::{Context, Error};

const ROLE_ID: RoleId = RoleId::new(1149435921301786634);

#[poise::command(slash_command, ephemeral, guild_only)]
pub async fn support_ban(ctx: Context<'_>, member: Member) -> Result<(), Error> {
	if member.roles.contains(&ROLE_ID) {
		ctx.say(format!(
			"❌ `{}` is already banned from support.",
			member.user.tag()
		))
		.await?;

		return Ok(());
	}

	member.add_role(ctx.http(), ROLE_ID).await?;
	ctx.say(format!("✅ Banned `{}` from support!", member.user.tag()))
		.await?;

	Ok(())
}

#[poise::command(slash_command, ephemeral, guild_only)]
pub async fn support_unban(ctx: Context<'_>, member: Member) -> Result<(), Error> {
	if !member.roles.contains(&ROLE_ID) {
		ctx.say(format!(
			"❌ `{}` is not banned from support.",
			member.user.tag()
		))
		.await?;

		return Ok(());
	}

	member.remove_role(ctx.http(), ROLE_ID).await?;
	ctx.say(format!("✅ Unbanned `{}` from support!", member.user.tag()))
		.await?;

	Ok(())
}
