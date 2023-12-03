use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static COLORS: Lazy<HashMap<&str, (u8, u8, u8)>> = Lazy::new(|| {
    HashMap::from([
        ("red", (239, 68, 68)),
        ("green", (34, 197, 94)),
        ("blue", (96, 165, 250)),
        ("yellow", (253, 224, 71)),
        ("orange", (251, 146, 60)),
    ])
});

pub const ETA_MESSAGES: [&str; 16] = [
    "Sometime",
    "Some day",
    "Not far",
    "The future",
    "Never",
    "Perhaps tomorrow?",
    "There are no ETAs",
    "No",
    "Nah",
    "Yes",
    "Yas",
    "Next month",
    "Next year",
    "Next week",
    "In Prism Launcher 2.0.0",
    "At the appropriate juncture, in due course, in the fullness of time",
];
