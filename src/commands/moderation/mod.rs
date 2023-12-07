use crate::Context;
use std::error::Error;

use color_eyre::eyre::{eyre, Result};
use poise::serenity_prelude::{ArgumentConvert, ChannelId, GuildId, Member};

mod actions;
use actions::{Ban, Kick, ModAction};

async fn split_argument<T>(
    ctx: &Context<'_>,
    guild_id: Option<GuildId>,
    channel_id: Option<ChannelId>,
    list: String,
) -> Result<Vec<T>>
where
    T: ArgumentConvert,
    T::Err: Error + Send + Sync + 'static,
{
    // yes i should be using something like `filter_map()` here. async closures
    // are unstable though so woooooo
    let mut res: Vec<T> = vec![];
    for item in list.split(',') {
        let item = T::convert(ctx.serenity_context(), guild_id, channel_id, item.trim()).await?;

        res.push(item);
    }

    Ok(res)
}

/// Ban a user
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    default_member_permissions = "BAN_MEMBERS",
    required_permissions = "BAN_MEMBERS",
    aliases("ban")
)]
pub async fn ban_user(
    ctx: Context<'_>,
    #[description = "User to ban"] user: Member,
    #[description = "Reason to ban"] reason: Option<String>,
    #[description = "Number of days to purge their messages from (defaults to 0)"]
    purge_messages_days: Option<u8>,
    #[description = "If true, the reply from the bot will be ephemeral"] quiet: Option<bool>,
    #[description = "If true, the affected user will be sent a DM"] dm_user: Option<bool>,
) -> Result<()> {
    let dmd = purge_messages_days.unwrap_or(0);

    let action = ModAction {
        reason,
        data: Ban {
            purge_messages_days: dmd,
        },
    };

    action.handle(&ctx, &user, quiet, dm_user, true).await?;

    Ok(())
}

/// Ban multiple users
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    default_member_permissions = "BAN_MEMBERS",
    required_permissions = "BAN_MEMBERS",
    aliases("ban_multi")
)]
pub async fn mass_ban(
    ctx: Context<'_>,
    #[description = "Comma separated list of users to ban"] users: String,
    #[description = "Reason to ban"] reason: Option<String>,
    #[description = "Number of days to purge their messages from (defaults to 0)"]
    purge_messages_days: Option<u8>,
    #[description = "If true, the reply from the bot will be ephemeral"] quiet: Option<bool>,
    #[description = "If true, the affected user will be sent a DM"] dm_user: Option<bool>,
) -> Result<()> {
    let gid = ctx
        .guild_id()
        .ok_or_else(|| eyre!("Couldn't get GuildId!"))?;

    let dmd = purge_messages_days.unwrap_or(0);
    let users: Vec<Member> = split_argument(&ctx, Some(gid), None, users).await?;

    for user in &users {
        let action = ModAction {
            reason: reason.clone(),
            data: Ban {
                purge_messages_days: dmd,
            },
        };

        action.handle(&ctx, user, quiet, dm_user, false).await?;
    }

    let resp = format!("{} users banned!", users.len());
    ctx.reply(resp).await?;

    Ok(())
}

/// Kick a user
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    default_member_permissions = "KICK_MEMBERS",
    required_permissions = "KICK_MEMBERS",
    aliases("kick")
)]
pub async fn kick_user(
    ctx: Context<'_>,
    #[description = "User to kick"] user: Member,
    #[description = "Reason to kick"] reason: Option<String>,
    #[description = "If true, the reply from the bot will be ephemeral"] quiet: Option<bool>,
    #[description = "If true, the affected user will be sent a DM"] dm_user: Option<bool>,
) -> Result<()> {
    let action = ModAction {
        reason,
        data: Kick {},
    };

    action.handle(&ctx, &user, quiet, dm_user, true).await?;

    Ok(())
}

/// Kick multiple users
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    default_member_permissions = "KICK_MEMBERS",
    required_permissions = "KICK_MEMBERS",
    aliases("multi_kick")
)]
pub async fn mass_kick(
    ctx: Context<'_>,
    #[description = "Comma separated list of users to kick"] users: String,
    #[description = "Reason to kick"] reason: Option<String>,
    #[description = "If true, the reply from the bot will be ephemeral"] quiet: Option<bool>,
    #[description = "If true, the affected user will be sent a DM"] dm_user: Option<bool>,
) -> Result<()> {
    let gid = ctx
        .guild_id()
        .ok_or_else(|| eyre!("Couldn't get GuildId!"))?;
    let users: Vec<Member> = split_argument(&ctx, Some(gid), None, users).await?;

    for user in &users {
        let action = ModAction {
            reason: reason.clone(),
            data: Kick {},
        };

        action.handle(&ctx, user, quiet, dm_user, false).await?;
    }

    let resp = format!("{} users kicked!", users.len());
    ctx.reply(resp).await?;

    Ok(())
}
