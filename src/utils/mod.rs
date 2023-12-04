use color_eyre::eyre::{eyre, Result};
use rand::seq::SliceRandom;

#[macro_use]
mod macros;
mod resolve_message;

pub use resolve_message::resolve as resolve_message;

/*
 * chooses a random element from an array
 */
pub fn random_choice<const N: usize>(arr: [&str; N]) -> Result<String> {
    let mut rng = rand::thread_rng();
    let resp = arr
        .choose(&mut rng)
        .ok_or_else(|| eyre!("Couldn't choose random object from array:\n{arr:#?}!"))?;

    Ok((*resp).to_string())
}
