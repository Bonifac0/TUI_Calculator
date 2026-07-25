use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AngleUnit {
    Deg,
    Rad,
}

impl Default for AngleUnit {
    fn default() -> Self {
        AngleUnit::Deg
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayoutMode {
    Auto,
    Big,
    Medium,
    Small,
}

impl Default for LayoutMode {
    fn default() -> Self {
        LayoutMode::Auto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub angle_unit: AngleUnit,
    pub precision: usize,
    pub default_layout: LayoutMode,
    pub theme: String,
    pub latex_rendering: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            angle_unit: AngleUnit::Deg,
            precision: 6,
            default_layout: LayoutMode::Auto,
            theme: "dark".to_string(),
            latex_rendering: true,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(settings) = toml::from_str(&content) {
                        return settings;
                    }
                }
            }
        }
        Settings::default()
    }

    fn get_config_path() -> Option<PathBuf> {
        if let Some(home) = std::env::var_os("HOME") {
            let mut path = PathBuf::from(home);
            path.push(".config");
            path.push("tui_calculator");
            path.push("config.toml");
            Some(path)
        } else {
            None
        }
    }
}
