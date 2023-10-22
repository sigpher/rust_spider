use std::error::Error;
use std::fs;

use serde::Deserialize;
use serde::Serialize;

pub fn get_config() -> Result<SettingsStruct, Box<dyn Error>> {
    let settings = toml::from_str(&fs::read_to_string("settings.toml")?)?;
    Ok(settings)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsStruct {
    pub base: String,
    pub pages: u32,
}
