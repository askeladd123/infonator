#![allow(unused)]
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const SETTINGS_DIR: &str = "../";

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
        let mut file = std::fs::File::create(PathBuf::from(SETTINGS_DIR).join("settings.json"))?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        let file = std::fs::File::open(PathBuf::from(SETTINGS_DIR).join("settings.json"))?;
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

pub fn extract_wifi_quality(from: &str) -> Result<f32, String> {
    todo!()
}

pub fn extract_battery_percentage(from: &str) -> Result<f32, String> {
    todo!()
}

pub struct TimeOfDay {
    hours: u32,
    minutes: u32,
}

pub fn extract_time(from: &str) -> Result<TimeOfDay, String> {
    todo!()
}
