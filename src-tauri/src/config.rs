use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinding {
    pub action: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub accent: Option<String>,
    pub keybindings: HashMap<String, Keybinding>,
    pub locale: String,
    pub default_search: String,
    pub dnt: bool,
    pub profile_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert(
            "newTab".to_string(),
            Keybinding {
                action: "newTab".to_string(),
                keys: vec!["Control".to_string(), "KeyT".to_string()],
            },
        );
        keybindings.insert(
            "closeTab".to_string(),
            Keybinding {
                action: "closeTab".to_string(),
                keys: vec!["Control".to_string(), "KeyW".to_string()],
            },
        );
        keybindings.insert(
            "nextTab".to_string(),
            Keybinding {
                action: "nextTab".to_string(),
                keys: vec!["Control".to_string(), "Tab".to_string()],
            },
        );
        keybindings.insert(
            "prevTab".to_string(),
            Keybinding {
                action: "prevTab".to_string(),
                keys: vec!["Control".to_string(), "Shift".to_string(), "Tab".to_string()],
            },
        );
        keybindings.insert(
            "reopenClosed".to_string(),
            Keybinding {
                action: "reopenClosed".to_string(),
                keys: vec!["Control".to_string(), "Shift".to_string(), "KeyT".to_string()],
            },
        );
        keybindings.insert(
            "focusAddress".to_string(),
            Keybinding {
                action: "focusAddress".to_string(),
                keys: vec!["Control".to_string(), "KeyL".to_string()],
            },
        );
        keybindings.insert(
            "newWindow".to_string(),
            Keybinding {
                action: "newWindow".to_string(),
                keys: vec!["Control".to_string(), "KeyN".to_string()],
            },
        );
        keybindings.insert(
            "tabSearch".to_string(),
            Keybinding {
                action: "tabSearch".to_string(),
                keys: vec!["Control".to_string(), "KeyK".to_string()],
            },
        );

        Settings {
            theme: "auto".to_string(),
            accent: None,
            keybindings,
            locale: "en".to_string(),
            default_search: "https://www.google.com/search?q=".to_string(),
            dnt: false,
            profile_path: String::new(),
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("crynn");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

pub fn get_settings_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("settings.json"))
}

pub fn load_settings() -> Result<Settings> {
    let path = get_settings_path()?;
    if !path.exists() {
        let default = Settings::default();
        save_settings(&default)?;
        return Ok(default);
    }

    let content = fs::read_to_string(&path)?;
    let settings: Settings = serde_json::from_str(&content)
        .context("Failed to parse settings.json")?;
    Ok(settings)
}

pub fn save_settings(settings: &Settings) -> Result<()> {
    let path = get_settings_path()?;
    let content = serde_json::to_string_pretty(settings)?;
    fs::write(&path, content)?;
    Ok(())
}

