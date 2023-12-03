use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod general;
mod moderation;

pub fn to_global_commands() -> Vec<Command<Data, Report>> {
    vec![
        general::joke(),
        general::members(),
        general::modrinth(),
        general::rory(),
        general::say(),
        general::stars(),
        moderation::ban_user(),
        moderation::kick_user(),
    ]
}
