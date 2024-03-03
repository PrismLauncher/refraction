use crate::Data;

use eyre::{eyre, Context as _, OptionExt, Report, Result};
use log::{debug, trace};
use poise::serenity_prelude::{
	ChannelType, Context, CreateAllowedMentions, CreateMessage, GuildChannel,
};
use poise::FrameworkContext;

pub async fn handle(
	ctx: &Context,
	thread: &GuildChannel,
	framework: FrameworkContext<'_, Data, Report>,
) -> Result<()> {
	if thread.kind != ChannelType::PublicThread {
		trace!("Not doing support onboard in non-public thread channel");
		return Ok(());
	}

	// TODO @getchoo: it seems like we can get multiple ThreadCreate events
	// should probably figure out a better way to not repeat ourselves here
	if thread
		.members(ctx)
		.wrap_err_with(|| {
			format!(
				"Couldn't fetch members from thread {}! Not sending a support onboard message.",
				thread.id
			)
		})?
		.iter()
		.any(|member| member.user.id == framework.bot_id)
	{
		debug!("Not sending support onboard message...I think i've been here before :p");
		return Ok(());
	}

	if thread
		.parent_id
		.ok_or_else(|| eyre!("Couldn't get parent ID from thread {}!", thread.name))?
		.name(ctx)
		.await
		.unwrap_or_default()
		!= "support"
	{
		debug!("Not posting onboarding message to threads outside of support");
		return Ok(());
	}

	let owner = thread
		.owner_id
		.ok_or_eyre("Couldn't get owner of thread!")?;

	let msg = format!(
    "<@{}> We've received your support ticket! {} {}",
    owner,
    "Please upload your logs and post the link here if possible (run `tag log` to find out how).",
    "Please don't ping people for support questions, unless you have their permission."
    );

	let allowed_mentions = CreateAllowedMentions::new()
		.replied_user(true)
		.users(Vec::from([owner]));

	let message = CreateMessage::new()
		.content(msg)
		.allowed_mentions(allowed_mentions);

	thread.send_message(ctx, message).await?;

	Ok(())
}
