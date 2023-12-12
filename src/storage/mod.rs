use std::fmt::Debug;

use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{ChannelId, MessageId, UserId};
use redis::{AsyncCommands as _, Client, FromRedisValue, ToRedisArgs};

pub mod message_logger;
use message_logger::*;

const PK_KEY: &str = "pluralkit-v1";
const LAUNCHER_VERSION_KEY: &str = "launcher-version-v1";

#[derive(Clone, Debug)]
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;

        Ok(Self { client })
    }

    /*
      these are mainly light abstractions to avoid the `let mut con`
      boilerplate, as well as not require the caller to format the
      strings for keys
    */

    async fn get_key<T>(&self, key: &str) -> Result<T>
    where
        T: FromRedisValue,
    {
        debug!("Getting key {key}");

        let mut con = self.client.get_async_connection().await?;
        let res: T = con.get(key).await?;

        Ok(res)
    }

    async fn set_key<'a>(
        &self,
        key: &str,
        value: impl ToRedisArgs + Debug + Send + Sync + 'a,
    ) -> Result<()> {
        debug!("Creating key {key}:\n{value:#?}");

        let mut con = self.client.get_async_connection().await?;
        con.set(key, value).await?;

        Ok(())
    }

    async fn key_exists(&self, key: &str) -> Result<bool> {
        debug!("Checking if key {key} exists");

        let mut con = self.client.get_async_connection().await?;
        let exists: u64 = con.exists(key).await?;

        Ok(exists > 0)
    }

    async fn delete_key(&self, key: &str) -> Result<()> {
        debug!("Deleting key {key}");

        let mut con = self.client.get_async_connection().await?;
        con.del(key).await?;

        Ok(())
    }

    async fn expire_key(&self, key: &str, expire_seconds: usize) -> Result<()> {
        debug!("Expiring key {key} in {expire_seconds}");

        let mut con = self.client.get_async_connection().await?;
        con.expire(key, expire_seconds).await?;

        Ok(())
    }

    pub async fn store_user_plurality(&self, sender: UserId) -> Result<()> {
        info!("Marking {sender} as a PluralKit user");
        let key = format!("{PK_KEY}:{sender}");

        // Just store some value. We only care about the presence of this key
        self.set_key(&key, 0).await?;
        self.expire_key(&key, 7 * 24 * 60 * 60).await?;

        Ok(())
    }

    pub async fn is_user_plural(&self, user_id: UserId) -> Result<bool> {
        let key = format!("{PK_KEY}:{user_id}");
        self.key_exists(&key).await
    }

    pub async fn store_message(
        &self,
        channel_id: &ChannelId,
        message_id: &MessageId,
        content: String,
        author: UserId,
    ) -> Result<()> {
        let key = format!("{MSG_LOG_KEY}:{channel_id}:{message_id}");

        let val = MessageLog { author, content };

        self.set_key(&key, val).await?;
        self.expire_key(&key, 30 * 24 * 60 * 60).await?; // only store for 30 days

        Ok(())
    }

    pub async fn get_message(
        &self,
        channel_id: &ChannelId,
        message_id: &MessageId,
    ) -> Result<MessageLog> {
        let key = format!("{MSG_LOG_KEY}:{channel_id}:{message_id}");
        let res = self.get_key(&key).await?;

        Ok(res)
    }

    pub async fn delete_message(
        &self,
        channel_id: &ChannelId,
        message_id: &MessageId,
    ) -> Result<()> {
        let key = format!("{MSG_LOG_KEY}:{channel_id}:{message_id}");
        self.delete_key(&key).await?;

        Ok(())
    }

    pub async fn cache_launcher_version(&self, version: &str) -> Result<()> {
        self.set_key(LAUNCHER_VERSION_KEY, version).await?;

        Ok(())
    }

    pub async fn get_launcher_version(&self) -> Result<String> {
        let res = self.get_key(LAUNCHER_VERSION_KEY).await?;

        Ok(res)
    }

    pub async fn launcher_version_is_cached(&self) -> Result<bool> {
        let res = self.key_exists(LAUNCHER_VERSION_KEY).await?;

        Ok(res)
    }
}
