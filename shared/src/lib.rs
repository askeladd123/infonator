#![allow(unused)]
use regex::Regex;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const DATA_DIR_NAME: &str = "infonator";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    // scripts for aquiring system information
    pub script_path_wifi_name: PathBuf,
    pub script_path_wifi_quality: PathBuf,
    pub script_path_battery_percentage: PathBuf,
    pub script_path_battery_time_left: PathBuf,
    pub script_path_time: PathBuf,
    pub script_path_volume: PathBuf,
    pub script_path_brightness: PathBuf,
    pub script_path_date: PathBuf,
    pub script_path_cpu_temperature: PathBuf,
    pub script_path_ram_usage: PathBuf,

    // general settings
    pub close_on_any_key: bool,
    pub content_size_ratio: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            close_on_any_key: true,
            content_size_ratio: 1.0,
            script_path_time: "".into(),
            script_path_volume: "".into(),
            script_path_brightness: "".into(),
            script_path_date: "".into(),
            script_path_cpu_temperature: "".into(),
            script_path_ram_usage: "".into(),
            script_path_wifi_name: "".into(),
            script_path_wifi_quality: "".into(),
            script_path_battery_percentage: "".into(),
            script_path_battery_time_left: "".into(),
        }
    }
}

impl Settings {
    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = std::fs::File::create(
            dirs::data_dir()
                .unwrap()
                .join(DATA_DIR_NAME)
                .join("settings.json"),
        )?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        let file = std::fs::File::open(
            dirs::data_dir()
                .unwrap()
                .join(DATA_DIR_NAME)
                .join("settings.json"),
        )?;
        let your_struct = serde_json::from_reader(file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(your_struct)
    }
}

pub enum Error {}

pub fn run_user_script(path: &Path) -> Result<std::process::Output, String> {
    match std::process::Command::new(path).output() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn extract_wifi_quality(from: &str) -> Option<f32> {
    let re = Regex::new(r"([-+]?\d*\.?\d+)").unwrap();
    if let Some(caps) = re.captures(from) {
        if let Ok(number) = caps[1].parse::<f32>() {
            if 0. <= number && number <= 100. {
                return Some(number);
            }
        }
    }
    None
}

pub fn extract_battery_percentage(from: &str) -> Option<f32> {
    let re = Regex::new(r"([-+]?\d*\.?\d+)").unwrap();
    if let Some(caps) = re.captures(from) {
        if let Ok(number) = caps[1].parse::<f32>() {
            return Some(number);
        }
    }
    None
}

pub mod time {
    pub struct TimeOfDay {
        hours: u32,
        minutes: u32,
    }

    impl TimeOfDay {
        pub fn hours(&self) -> u32 {
            self.hours
        }

        pub fn minutes(&self) -> u32 {
            self.minutes
        }

        pub fn new(hours: u32, minutes: u32) -> Option<Self> {
            if hours < 24 && minutes < 60 {
                Some(TimeOfDay { hours, minutes })
            } else {
                None
            }
        }
    }
}

pub fn extract_time(from: &str) -> Option<time::TimeOfDay> {
    let re = Regex::new(r"^(\d{1,2}):(\d{2})$").unwrap();
    if let Some(caps) = re.captures(from) {
        if let (Ok(hours), Ok(minutes)) = (caps[1].parse::<u32>(), caps[2].parse::<u32>()) {
            return time::TimeOfDay::new(hours, minutes);
        }
    }
    None
}
