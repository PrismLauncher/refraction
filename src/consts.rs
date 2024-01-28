use std::collections::HashMap;

use once_cell::sync::Lazy;
use poise::serenity_prelude::Color;

pub static COLORS: Lazy<HashMap<&str, Color>> = Lazy::new(|| {
	HashMap::from([
		("red", Color::from((239, 68, 68))),
		("green", Color::from((34, 197, 94))),
		("blue", Color::from((96, 165, 250))),
		("yellow", Color::from((253, 224, 71))),
		("orange", Color::from((251, 146, 60))),
		// TODO purple & pink :D
	])
});
