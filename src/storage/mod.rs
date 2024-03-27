use std::fmt::Debug;

use eyre::Result;
use log::debug;
use poise::serenity_prelude::UserId;
use redis::{AsyncCommands, Client, ConnectionLike};

const PK_KEY: &str = "pluralkit-v1";
const LAUNCHER_VERSION_KEY: &str = "launcher-version-v1";
const LAUNCHER_STARGAZER_KEY: &str = "launcher-stargazer-v1";

#[derive(Clone, Debug)]
pub struct Storage {
	client: Client,
}

impl Storage {
	pub fn new(client: Client) -> Self {
		Self { client }
	}

	pub fn from_url(redis_url: &str) -> Result<Self> {
		let client = Client::open(redis_url)?;

		Ok(Self::new(client))
	}

	pub fn has_connection(mut self) -> bool {
		self.client.check_connection()
	}

	pub async fn store_user_plurality(&self, sender: UserId) -> Result<()> {
		debug!("Marking {sender} as a PluralKit user");
		let key = format!("{PK_KEY}:{sender}");

		let mut con = self.client.get_async_connection().await?;
		// Just store some value. We only care about the presence of this key
		con.set_ex(key, 0, 7 * 24 * 60 * 60).await?; // 1 week

		Ok(())
	}

	pub async fn is_user_plural(&self, user_id: UserId) -> Result<bool> {
		debug!("Checking if user {user_id} is plural");
		let key = format!("{PK_KEY}:{user_id}");

		let mut con = self.client.get_async_connection().await?;
		let exists = con.exists(key).await?;

		Ok(exists)
	}

	pub async fn cache_launcher_version(&self, version: &str) -> Result<()> {
		debug!("Caching launcher version as {version}");

		let mut con = self.client.get_async_connection().await?;
		con.set_ex(LAUNCHER_VERSION_KEY, version, 24 * 60 * 60)
			.await?; // 1 day

		Ok(())
	}

	pub async fn get_launcher_version(&self) -> Result<String> {
		debug!("Fetching launcher version");

		let mut con = self.client.get_async_connection().await?;
		let res = con.get(LAUNCHER_VERSION_KEY).await?;

		Ok(res)
	}

	pub async fn cache_launcher_stargazer_count(&self, stargazers: u32) -> Result<()> {
		debug!("Caching stargazer count as {stargazers}");

		let mut con = self.client.get_async_connection().await?;
		con.set_ex(LAUNCHER_STARGAZER_KEY, stargazers, 60 * 60)
			.await?;

		Ok(())
	}

	pub async fn get_launcher_stargazer_count(&self) -> Result<u32> {
		debug!("Fetching launcher stargazer count");

		let mut con = self.client.get_async_connection().await?;
		let res: u32 = con.get(LAUNCHER_STARGAZER_KEY).await?;

		Ok(res)
	}
}
