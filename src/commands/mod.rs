use crate::Data;

use eyre::Report;

mod general;
mod moderation;

macro_rules! command {
	($module: ident, $name: ident) => {
		$module::$name::$name()
	};

	($module: ident, $name: ident, $func: ident) => {
		$module::$name::$func()
	};
}

macro_rules! module_macro {
	($module: ident) => {
		macro_rules! $module {
			($name: ident) => {
				command!($module, $name)
			};

			($name: ident, $func: ident) => {
				command!($module, $name, $func)
			};
		}
	};
}

module_macro!(general);
module_macro!(moderation);

pub type Command = poise::Command<Data, Report>;

pub fn get() -> Vec<Command> {
	vec![
		general!(help),
		general!(joke),
		general!(members),
		general!(ping),
		general!(rory),
		general!(say),
		general!(stars),
		general!(tag),
		moderation!(set_welcome),
	]
}
