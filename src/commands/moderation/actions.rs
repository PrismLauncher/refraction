use crate::{consts::COLORS, Context};

use async_trait::async_trait;
use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use poise::serenity_prelude::{CacheHttp, Http, Member, Timestamp};

type Fields<'a> = Vec<(&'a str, String, bool)>;

#[async_trait]
pub trait ModActionInfo {
    fn to_fields(&self) -> Option<Fields>;
    fn description(&self) -> String;
    async fn run_action(
        &self,
        http: impl CacheHttp + AsRef<Http>,
        user: &Member,
        reason: String,
    ) -> Result<()>;
}

pub struct ModAction<T>
where
    T: ModActionInfo,
{
    pub reason: Option<String>,
    pub data: T,
}

impl<T: ModActionInfo> ModAction<T> {
    fn get_all_fields(&self) -> Fields {
        let mut fields = vec![];

        if let Some(reason) = self.reason.clone() {
            fields.push(("Reason:", reason, false));
        }

        if let Some(mut action_fields) = self.data.to_fields() {
            fields.append(&mut action_fields);
        }

        fields
    }

    /// internal mod logging
    pub async fn log_action(&self, ctx: &Context<'_>) -> Result<()> {
        let channel_id = ctx
            .data()
            .config
            .discord
            .channels
            .say_log_channel_id
            .ok_or_else(|| eyre!("Couldn't find say_log_channel_id! Unable to log mod action"))?;

        let channel = ctx
          .http()
          .get_channel(channel_id.into())
          .await
          .wrap_err_with(|| "Couldn't resolve say_log_channel_id as a Channel! Are you sure you sure you used the right one?")?;

        let channel = channel
          .guild()
          .ok_or_else(|| eyre!("Couldn't resolve say_log_channel_id as a GuildChannel! Are you sure you used the right one?"))?;

        let fields = self.get_all_fields();
        let title = format!("{} user!", self.data.description());

        channel
            .send_message(ctx, |m| {
                m.embed(|e| e.title(title).fields(fields).color(COLORS["red"]))
            })
            .await?;

        Ok(())
    }

    /// public facing message
    pub async fn reply(
        &self,
        ctx: &Context<'_>,
        user: &Member,
        dm_user: Option<bool>,
    ) -> Result<()> {
        let mut resp = format!("{} {}!", self.data.description(), user.user.name);

        if dm_user.unwrap_or_default() {
            resp = format!("{resp} (user notified with direct message)");
        }

        ctx.reply(resp).await?;

        Ok(())
    }

    pub async fn dm_user(&self, ctx: &Context<'_>, user: &Member) -> Result<()> {
        let guild = ctx.http().get_guild(*user.guild_id.as_u64()).await?;
        let title = format!("{} from {}!", self.data.description(), guild.name);

        user.user
            .dm(ctx, |m| {
                m.embed(|e| {
                    e.title(title).color(COLORS["red"]);

                    if let Some(reason) = &self.reason {
                        e.description(format!("Reason: {}", reason));
                    }

                    e
                })
            })
            .await?;

        Ok(())
    }

    pub async fn handle(
        &self,
        ctx: &Context<'_>,
        user: &Member,
        quiet: Option<bool>,
        dm_user: Option<bool>,
        handle_reply: bool,
    ) -> Result<()> {
        let actual_reason = self.reason.clone().unwrap_or("".to_string());
        self.data.run_action(ctx, user, actual_reason).await?;

        if quiet.unwrap_or_default() {
            ctx.defer_ephemeral().await?;
        } else {
            ctx.defer().await?;
        }

        self.log_action(ctx).await?;

        if dm_user.unwrap_or_default() {
            self.dm_user(ctx, user).await?;
        }

        if handle_reply {
            self.reply(ctx, user, dm_user).await?;
        }

        Ok(())
    }
}

pub struct Ban {
    pub purge_messages_days: u8,
}

#[async_trait]
impl ModActionInfo for Ban {
    fn to_fields(&self) -> Option<Fields> {
        let fields = vec![(
            "Purged messages:",
            format!("Last {} day(s)", self.purge_messages_days),
            false,
        )];

        Some(fields)
    }

    fn description(&self) -> String {
        "Banned".to_string()
    }

    async fn run_action(
        &self,
        http: impl CacheHttp + AsRef<Http>,
        user: &Member,
        reason: String,
    ) -> Result<()> {
        debug!("Banning user {user} with reason: \"{reason}\"");

        user.ban_with_reason(http, self.purge_messages_days, reason)
            .await?;

        Ok(())
    }
}

pub struct Timeout {
    pub time_until: Timestamp,
}

#[async_trait]
impl ModActionInfo for Timeout {
    fn to_fields(&self) -> Option<Fields> {
        let fields = vec![("Timed out until:", self.time_until.to_string(), false)];

        Some(fields)
    }

    fn description(&self) -> String {
        "Timed out".to_string()
    }

    #[allow(unused_variables)]
    async fn run_action(
        &self,
        http: impl CacheHttp + AsRef<Http>,
        user: &Member,
        reason: String,
    ) -> Result<()> {
        todo!()
    }
}

pub struct Kick {}

#[async_trait]
impl ModActionInfo for Kick {
    fn to_fields(&self) -> Option<Fields> {
        None
    }

    fn description(&self) -> String {
        "Kicked".to_string()
    }

    async fn run_action(
        &self,
        http: impl CacheHttp + AsRef<Http>,
        user: &Member,
        reason: String,
    ) -> Result<()> {
        debug!("Kicked user {user} with reason: \"{reason}\"");

        user.kick_with_reason(http, &reason).await?;

        Ok(())
    }
}
