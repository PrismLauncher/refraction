#![allow(clippy::unreadable_literal)]
use std::str::FromStr;

use poise::serenity_prelude::Colour;

const BLUE: u32 = 0x60A5FA;
const GREEN: u32 = 0x22C55E;
const ORANGE: u32 = 0xFB923C;
const RED: u32 = 0xEF4444;
const YELLOW: u32 = 0xFDE047;

#[derive(Clone, Copy, Debug, Default)]
pub enum Colors {
	Blue,
	#[default]
	Green,
	Orange,
	Red,
	Yellow,
}

impl From<Colors> for Colour {
	fn from(value: Colors) -> Self {
		Self::from(match &value {
			Colors::Blue => BLUE,
			Colors::Green => GREEN,
			Colors::Orange => ORANGE,
			Colors::Red => RED,
			Colors::Yellow => YELLOW,
		})
	}
}

impl FromStr for Colors {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"blue" => Ok(Self::Blue),
			"green" => Ok(Self::Green),
			"orange" => Ok(Self::Orange),
			"red" => Ok(Self::Red),
			"yellow" => Ok(Self::Yellow),
			_ => Err(()),
		}
	}
}
