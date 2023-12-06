use crate::{consts::COLORS, Context};

use color_eyre::eyre::{eyre, Result};
use poise::serenity_prelude::{
    futures::TryFutureExt, CreateEmbed, CreateMessage, FutureExt, Guild, Timestamp, User, UserId,
};

struct Action {
    reason: String,
    data: ActionData,
}

enum ActionData {
    Kick,
    Ban { purge: u8 },
    Timeout { until: Timestamp },
}

fn build_dm<'a, 'b>(
    message: &'b mut CreateMessage<'a>,
    guild: &Guild,
    action: &Action,
) -> &'b mut CreateMessage<'a> {
    let description = match &action.data {
        ActionData::Kick => "kicked from".to_string(),
        ActionData::Ban { purge: _ } => "banned from".to_string(),
        ActionData::Timeout { until } => {
            format!("timed out until <t:{}> in", until.unix_timestamp())
        }
    };
    let guild_name = &guild.name;
    let reason = &action.reason;
    message.content(format!(
        "You have been {description} {guild_name}.\nReason: {reason}"
    ))
}

async fn moderate(
    ctx: &Context<'_>,
    users: &Vec<UserId>,
    action: &Action,
    quiet: bool,
) -> Result<()> {
    let guild = ctx
        .guild()
        .ok_or_else(|| eyre!("Couldn't get guild from message!"))?;
    let reason = &action.reason;

    let mut count = 0;

    for user in users {
        if quiet {
            if let Ok(channel) = user.create_dm_channel(ctx.http()).await {
                let _ = channel
                    .send_message(ctx.http(), |message| build_dm(message, &guild, action))
                    .await;
            }
        }

        let success = match action.data {
            ActionData::Kick => guild
                .kick_with_reason(ctx.http(), user, reason)
                .await
                .is_ok(),

            ActionData::Ban { purge } => guild
                .ban_with_reason(ctx.http(), user, purge, reason)
                .await
                .is_ok(),

            ActionData::Timeout { until } => guild
                .edit_member(ctx.http(), user, |member| {
                    member.disable_communication_until_datetime(until)
                })
                .await
                .is_ok(),
        };
        if success {
            count += 1;
        }
    }

    let total = users.len();
    if count == total {
        ctx.reply("✅ Done!").await?;
    } else {
        ctx.reply(format!("⚠️ {count}/{total} succeeded!"))
            .await?;
    }

    Ok(())
}

/// Ban a user
#[poise::command(
    slash_command,
    prefix_command,
    default_member_permissions = "BAN_MEMBERS",
    required_permissions = "BAN_MEMBERS",
    aliases("ban")
)]
pub async fn ban(
    ctx: Context<'_>,
    users: Vec<UserId>,
    purge: Option<u8>,
    reason: Option<String>,
    quiet: Option<bool>,
) -> Result<()> {
    moderate(
        &ctx,
        &users,
        &Action {
            reason: reason.unwrap_or_default(),
            data: ActionData::Ban {
                purge: purge.unwrap_or(0),
            },
        },
        quiet.unwrap_or(false),
    )
    .await
}

/// Kick a user
#[poise::command(
    slash_command,
    prefix_command,
    default_member_permissions = "KICK_MEMBERS",
    required_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    users: Vec<UserId>,
    reason: Option<String>,
    quiet: Option<bool>,
) -> Result<()> {
    moderate(
        &ctx,
        &users,
        &Action {
            reason: reason.unwrap_or_default(),
            data: ActionData::Kick {},
        },
        quiet.unwrap_or(false),
    )
    .await
}
