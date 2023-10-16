use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    // scripts for aquiring system information
    script_path_wifi: String,
    script_path_battery: String,
    script_path_time: String,
    script_path_volume: String,
    script_path_brightness: String,
    script_path_date: String,
    script_path_cpu: String,
    script_path_ram: String,

    // general settings
    close_on_any_key: bool,
    content_size_ratio: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            script_path_wifi: "".into(),
            script_path_battery: "".into(),
            script_path_time: "".into(),
            script_path_volume: "".into(),
            script_path_brightness: "".into(),
            script_path_date: "".into(),
            script_path_cpu: "".into(),
            script_path_ram: "".into(),
            close_on_any_key: true,
            content_size_ratio: 1.0,
        }
    }
}

impl Settings {
    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = std::fs::File::create("settings.json")?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        let file = std::fs::File::open("settings.json")?;
        let your_struct = serde_json::from_reader(file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(your_struct)
    }
}
