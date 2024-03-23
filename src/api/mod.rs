use once_cell::sync::Lazy;

pub mod dadjoke;
pub mod paste_gg;
pub mod pluralkit;
pub mod prism_meta;
pub mod rory;

pub static USER_AGENT: Lazy<String> = Lazy::new(|| {
	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("development");

	format!("refraction/{version}")
});

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	reqwest::Client::builder()
		.user_agent(USER_AGENT.to_string())
		.build()
		.unwrap_or_default()
});
