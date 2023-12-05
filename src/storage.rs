use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::UserId;
use redis::{AsyncCommands as _, Client};

pub const USER_KEY: &str = "users-v1";

#[derive(Clone, Debug)]
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;

        Ok(Self { client })
    }

    pub async fn store_user_plurality(&self, sender: UserId) -> Result<()> {
        let mut con = self.client.get_async_connection().await?;

        info!("Marking {sender} as a PluralKit user");
        let key = format!("{USER_KEY}:{sender}:pk");

        // Just store some value. We only care about the presence of this key
        con.set(&key, 0).await?;
        con.expire(key, 7 * 24 * 60 * 60).await?;

        Ok(())
    }

    pub async fn is_user_plural(&self, user_id: UserId) -> Result<bool> {
        let key = format!("{USER_KEY}:{user_id}:pk");
        let mut con = self.client.get_async_connection().await?;

        let exists: bool = con.exists(key).await?;
        Ok(exists)
    }
}
