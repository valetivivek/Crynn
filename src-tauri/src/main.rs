// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bidi;
mod config;
mod downloads;

use bidi::{FirefoxInstance, TabInfo};
use config::{load_settings, save_settings, Settings};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tab {
    id: String,
    firefox_id: String,
    title: String,
    url: String,
    pinned: bool,
    muted: bool,
    audible: bool,
    group_id: Option<String>,
}

type FirefoxState = Arc<Mutex<Option<FirefoxInstance>>>;

#[tauri::command]
async fn launch_firefox(
    profile_path: String,
    state: State<'_, FirefoxState>,
) -> Result<(), String> {
    let mut firefox_state = state.lock().await;
    let profile = PathBuf::from(profile_path);
    let mut instance = FirefoxInstance::new(profile);
    instance
        .launch()
        .await
        .map_err(|e| e.to_string())?;
    *firefox_state = Some(instance);
    Ok(())
}

#[tauri::command]
async fn open_tab(
    url: String,
    state: State<'_, FirefoxState>,
) -> Result<String, String> {
    let firefox_state = state.lock().await;
    if let Some(ref instance) = *firefox_state {
        let tab_id = instance
            .open_tab(&url)
            .await
            .map_err(|e| e.to_string())?;
        Ok(tab_id)
    } else {
        Err("Firefox not running".to_string())
    }
}

#[tauri::command]
async fn close_tab(
    firefox_id: String,
    state: State<'_, FirefoxState>,
) -> Result<(), String> {
    let firefox_state = state.lock().await;
    if let Some(ref instance) = *firefox_state {
        instance
            .close_tab(&firefox_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Firefox not running".to_string())
    }
}

#[tauri::command]
async fn navigate_tab(
    firefox_id: String,
    url: String,
    state: State<'_, FirefoxState>,
) -> Result<(), String> {
    let firefox_state = state.lock().await;
    if let Some(ref instance) = *firefox_state {
        instance
            .navigate(&firefox_id, &url)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Firefox not running".to_string())
    }
}

#[tauri::command]
async fn list_tabs(state: State<'_, FirefoxState>) -> Result<Vec<TabInfo>, String> {
    let firefox_state = state.lock().await;
    if let Some(ref instance) = *firefox_state {
        instance
            .list_tabs()
            .await
            .map_err(|e| e.to_string())
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
fn get_settings() -> Result<Settings, String> {
    load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings_command(settings: Settings) -> Result<(), String> {
    save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
async fn is_firefox_running(state: State<'_, FirefoxState>) -> Result<bool, String> {
    let firefox_state = state.lock().await;
    if let Some(ref instance) = *firefox_state {
        Ok(instance.is_running().await)
    } else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(None::<FirefoxInstance>)))
        .invoke_handler(tauri::generate_handler![
            launch_firefox,
            open_tab,
            close_tab,
            navigate_tab,
            list_tabs,
            get_settings,
            save_settings_command,
            is_firefox_running
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                window.set_decorations(true)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
