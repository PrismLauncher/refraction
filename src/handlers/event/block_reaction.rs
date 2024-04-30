use crate::Data;

use chrono::Duration;
use eyre::{Context as _, Result};
use log::{debug, trace};
use poise::serenity_prelude::{Context, Reaction, Timestamp};

pub async fn handle(ctx: &Context, reaction: &Reaction, data: &Data) -> Result<()> {
	let reaction_type = reaction.emoji.clone();
	let reactor = reaction.user_id;
	let message = reaction.message(ctx).await.wrap_err_with(|| {
		format!(
			"Couldn't get message {} from reaction! We won't be able to check if it's old",
			reaction.message_id
		)
	})?;

	let time_sent = message.timestamp.to_utc();
	let age = Timestamp::now().signed_duration_since(time_sent);
	let max_days = Duration::days(data.config.discord.days_to_delete_reaction);

	if age >= max_days {
		// NOTE: if we for some reason **didn't** get the user_id associated with the reaction,
		// this will clear **all** reactions of this type. this is intentional as older reactions
		// being removed > harmful reactions being kept
		debug!(
			"Removing reaction {reaction_type} from message {}",
			message.id
		);
		message.delete_reaction(ctx, reactor, reaction_type).await?;
	} else {
		trace!(
			"Keeping reaction {reaction_type} for message {}",
			message.id
		);
	}

	Ok(())
}
