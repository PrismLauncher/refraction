use crate::{config::Config, consts::Colors, Data};

use chrono::Duration;
use eyre::Result;
use log::{debug, trace};
use poise::serenity_prelude::{
	Context, CreateEmbed, CreateMessage, Mentionable, Reaction, Timestamp,
};

async fn log_old_react(ctx: &Context, config: &Config, reaction: &Reaction) -> Result<()> {
	let Some(log_channel) = config.discord.channels.log_channel_id else {
		debug!("Not logging old reaction; no log channel is set!");
		return Ok(());
	};

	let message_link = reaction
		.message_id
		.link(reaction.channel_id, reaction.guild_id);

	let mut embed = CreateEmbed::new()
		.title("Old message reaction!")
		.color(Colors::Red);

	if let Some(reactor) = reaction.user_id {
		embed = embed.description(format!(
			"{} just reacted to {message_link}!",
			reactor.mention(),
		));
	} else {
		embed = embed.description(format!(
			"Someone (or something...) just reacted to {message_link}!"
		));
	}

	let message = CreateMessage::new().embed(embed);
	log_channel.send_message(ctx, message).await?;

	Ok(())
}

pub async fn handle(ctx: &Context, reaction: &Reaction, data: &Data) -> Result<()> {
	let reaction_type = reaction.emoji.clone();
	let message_id = reaction.message_id;
	trace!("Checking if we should block reaction on {message_id}");

	let time_sent = message_id.created_at().to_utc();
	let age = Timestamp::now().signed_duration_since(time_sent);
	let max_days = Duration::days(data.config.discord.days_to_delete_reaction);

	if age >= max_days {
		debug!(
			"Removing reaction {reaction_type} from message {}",
			message_id
		);

		reaction.delete(ctx).await?;
		log_old_react(ctx, &data.config, reaction).await?;
	} else {
		trace!(
			"Keeping reaction {reaction_type} for message {}",
			message_id
		);
	}

	Ok(())
}
