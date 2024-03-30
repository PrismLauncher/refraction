use std::{collections::HashMap, sync::OnceLock};

use poise::serenity_prelude::Color;

pub fn colors() -> &'static HashMap<&'static str, Color> {
	static COLORS: OnceLock<HashMap<&str, Color>> = OnceLock::new();
	COLORS.get_or_init(|| {
		HashMap::from([
			("red", Color::from((239, 68, 68))),
			("green", Color::from((34, 197, 94))),
			("blue", Color::from((96, 165, 250))),
			("yellow", Color::from((253, 224, 71))),
			("orange", Color::from((251, 146, 60))),
			// TODO purple & pink :D
		])
	})
}
