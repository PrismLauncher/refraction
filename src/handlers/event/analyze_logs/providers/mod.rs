use color_eyre::eyre::Result;
use poise::serenity_prelude::Message;

#[path = "0x0.rs"]
mod _0x0;
mod attachment;
mod haste;
mod mclogs;
mod paste_gg;
mod pastebin;

pub type LogProvider = Result<Option<String>>;

pub async fn find_log(message: &Message) -> LogProvider {
    macro_rules! provider_impl {
        ($provider:ident) => {
            if let Some(content) = $provider::find(&message.content).await? {
                return Ok(Some(content));
            }
        };
    }
    provider_impl!(_0x0);
    provider_impl!(mclogs);
    provider_impl!(haste);
    provider_impl!(paste_gg);
    provider_impl!(pastebin);

    if let Some(content) = attachment::find(message).await? {
        return Ok(Some(content));
    }

    Ok(None)
}
