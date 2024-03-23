use crate::Data;

use eyre::Report;

mod general;

pub type Command = poise::Command<Data, Report>;

pub fn get() -> Vec<Command> {
	vec![
		general::joke(),
		general::members(),
		general::ping(),
		general::rory(),
		general::say(),
		general::stars(),
		general::tag(),
		general::help(),
	]
}
