use crate::Data;

use eyre::Report;
use poise::Command;

mod general;

pub fn get() -> Vec<Command<Data, Report>> {
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
