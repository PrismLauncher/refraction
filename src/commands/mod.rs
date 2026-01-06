use crate::{Data, Error};

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

pub type Command = poise::Command<Data, Error>;

pub fn all() -> Vec<Command> {
	vec![
		general!(delete_interaction),
		general!(help),
		general!(joke),
		general!(members),
		general!(ping),
		general!(rory),
		general!(say),
		general!(stars),
		general!(tag),
		moderation!(set_welcome),
		moderation!(support_ban, support_ban),
		moderation!(support_ban, support_unban),
	]
}
