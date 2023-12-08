use crate::consts::COLORS;
use crate::Data;

use color_eyre::eyre::{eyre, Result};
use log::debug;
use poise::serenity_prelude::{
    ChannelId, Colour, Http, Message, MessageId, MessageUpdateEvent, User,
};

#[allow(unused_variables)]
pub async fn log_msg<T>(user: &User, content: String, color: T) -> Result<()>
where
    T: Into<Colour>,
{
    todo!()
}

pub async fn handle_create(data: &Data, msg: &Message) -> Result<()> {
    let channel_id = msg.channel_id;
    let message_id = msg.id;
    let content = &msg.content;
    let author_id = msg.author.id;

    debug!("Logging message {message_id}");
    data.storage
        .store_message(&channel_id, &message_id, content.to_string(), author_id)
        .await?;

    Ok(())
}

pub async fn handle_update(data: &Data, event: &MessageUpdateEvent) -> Result<()> {
    let stored = data
        .storage
        .get_message(&event.channel_id, &event.id)
        .await?;

    let new_content = event.content.as_ref().ok_or_else(|| {
        eyre!("Couldn't get content from event! Is the MESSAGE_CONTENT intent enabled?")
    })?;

    let author = event
        .author
        .as_ref()
        .ok_or_else(|| eyre!("Couldn't get author from message!"))?;

    if new_content != &stored.content {
        log_msg(author, new_content.to_string(), COLORS["yellow"]).await?;

        debug!("Updating message {}", event.id);
        data.storage
            .store_message(
                &event.channel_id,
                &event.id,
                new_content.to_string(),
                author.id,
            )
            .await?;
    }

    Ok(())
}

pub async fn handle_delete(
    http: impl AsRef<Http>,
    data: &Data,
    channel_id: &ChannelId,
    message_id: &MessageId,
) -> Result<()> {
    let stored = data.storage.get_message(channel_id, message_id).await?;
    let user = http.as_ref().get_user(*stored.author.as_u64()).await?;

    log_msg(&user, stored.content, COLORS["red"]).await?;

    debug!("Deleting message {message_id}");
    data.storage.delete_message(channel_id, message_id).await?;

    Ok(())
}
