use poise::serenity_prelude::UserId;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub const MSG_LOG_KEY: &str = "message-log-v1";

#[derive(Clone, Debug, Deserialize, Serialize, FromRedisValue, ToRedisArgs)]
pub struct MessageLog {
    pub author: UserId,
    pub content: String,
}
