use std::fs;
use chrono::{Datelike, Timelike, Utc};
use  toml::Table;

pub fn get_time() -> String {
    // get the current time in a stting format i like.
    let now = Utc::now();
    let (is_pm, hour) = now.hour12(); {
        let time = format!("{:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" });

        time
    }
}

pub fn get_version() -> String {
    "0.1.0".to_string()
}