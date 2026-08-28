use eyre::Result;
use log::debug;
use poise::serenity_prelude::{Context, EditMember, Member, Message, MessageType, RoleId};

use crate::storage::Storage;

const REQUIRED_CHATTINESS: i64 = 200;
const MAX_PER_DAY: i64 = 25;
const REGULAR_ROLE: RoleId = RoleId::new(1334298718362407063);

pub async fn handle_member(ctx: &Context, storage: &Storage, member: &Member) -> Result<()> {
	if member.roles.contains(&REGULAR_ROLE) {
		return Ok(());
	}

	let member_id = member.user.id;

	let chattiness = storage.chattiness(member_id.get()).await?;

	if chattiness < REQUIRED_CHATTINESS {
		debug!("Not granting regular role for {member_id}: need {REQUIRED_CHATTINESS} but have {chattiness}");
		return Ok(());
	}

	let mut new_roles = member.roles.clone();
	new_roles.push(REGULAR_ROLE);

	debug!("Granting regular role for {member_id}");
	member
		.guild_id
		.edit_member(
			&ctx.http,
			member.user.id,
			EditMember::new().roles(new_roles),
		)
		.await?;

	Ok(())
}

pub async fn handle_message(ctx: &Context, storage: &Storage, message: &Message) -> Result<()> {
	if message.kind != MessageType::Regular && message.kind != MessageType::InlineReply {
		return Ok(());
	}

	let Some(guild_id) = message.guild_id else {
		return Ok(());
	};
	let Some(ref member) = message.member else {
		return Ok(());
	};

	if member.roles.contains(&REGULAR_ROLE) {
		return Ok(());
	}

	let author_id = message.author.id;

	let daily_messages = storage.increase_daily_messages(author_id.get()).await?;

	if daily_messages > MAX_PER_DAY {
		debug!("Not increasing chattiness for {author_id}: already have {MAX_PER_DAY} messages");
		return Ok(());
	}

	let chattiness = storage.increase_chattiness(author_id.get()).await?;

	if chattiness < REQUIRED_CHATTINESS {
		debug!("Not granting regular role for {author_id}: need {REQUIRED_CHATTINESS} but have {chattiness}");
		return Ok(());
	}

	let mut new_roles = member.roles.clone();
	new_roles.push(REGULAR_ROLE);

	debug!("Granting regular role for {author_id}");
	guild_id
		.edit_member(
			&ctx.http,
			message.author.id,
			EditMember::new().roles(new_roles),
		)
		.await?;

	Ok(())
}
