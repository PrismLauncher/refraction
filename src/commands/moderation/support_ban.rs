#![allow(clippy::unreadable_literal)]

use poise::serenity_prelude::{Member, RoleId};

use crate::{Context, Error};

const ROLE_ID: RoleId = RoleId::new(1149435921301786634);
const HELPER_ID: RoleId = RoleId::new(1391519658917761106);

#[poise::command(
	slash_command,
	prefix_command,
	check = "require_helper",
	ephemeral,
	guild_only
)]
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
	ctx.say(format!("✅ Banned `{}` from support!", &member.user.tag()))
		.await?;

	Ok(())
}

#[poise::command(
	slash_command,
	prefix_command,
	check = "require_helper",
	ephemeral,
	guild_only
)]
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
	ctx.say(format!(
		"✅ Unbanned `{}` from support!",
		&member.user.tag()
	))
	.await?;

	Ok(())
}

async fn require_helper(ctx: Context<'_>) -> Result<bool, Error> {
	let Some(member) = ctx.author_member().await else {
		return Ok(false);
	};

	let is_helper = member.roles.contains(&HELPER_ID);
	let is_moderator = member.permissions.unwrap_or_default().manage_roles();

	Ok(is_helper || is_moderator)
}
