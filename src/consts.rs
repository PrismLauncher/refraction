#![allow(clippy::unreadable_literal)]
use std::str::FromStr;

use poise::serenity_prelude::Colour;

#[derive(Clone, Copy, Debug, Default)]
pub struct Colors(i32);

impl Colors {
	pub const RED: i32 = 0xEF4444;
	pub const GREEN: i32 = 0x22C55E;
	pub const BLUE: i32 = 0x60A5FA;
	pub const YELLOW: i32 = 0xFDE047;
	pub const ORANGE: i32 = 0xFB923C;

	pub fn as_i32(self) -> i32 {
		self.0
	}
}

impl FromStr for Colors {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"red" => Ok(Colors(Self::RED)),
			"green" => Ok(Colors(Self::GREEN)),
			"blue" => Ok(Colors(Self::BLUE)),
			"yellow" => Ok(Colors(Self::YELLOW)),
			"orange" => Ok(Colors(Self::ORANGE)),
			_ => Err(()),
		}
	}
}

impl From<Colors> for Colour {
	fn from(value: Colors) -> Self {
		Self::from(value.as_i32())
	}
}
