use color_eyre::eyre::Result;
use poise::serenity_prelude::Message;

pub async fn find(message: &Message) -> Result<Option<String>> {
    // find first uploaded text file
    if let Some(attachment) = message.attachments.iter().find(|a| {
        a.content_type
            .as_ref()
            .and_then(|ct| ct.starts_with("text/").then_some(true))
            .is_some()
    }) {
        let bytes = attachment.download().await?;
        let res = String::from_utf8(bytes)?;
        Ok(Some(res))
    } else {
        Ok(None)
    }
}
