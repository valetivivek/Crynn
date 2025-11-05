use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Child;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub audible: bool,
}

pub struct FirefoxInstance {
    pub process: Arc<Mutex<Option<Child>>>,
    pub profile_path: PathBuf,
    pub bidi_port: u16,
    pub session_id: Option<String>,
}

impl FirefoxInstance {
    pub fn new(profile_path: PathBuf) -> Self {
        FirefoxInstance {
            process: Arc::new(Mutex::new(None)),
            profile_path,
            bidi_port: 9222,
            session_id: None,
        }
    }

    pub async fn launch(&mut self) -> Result<()> {
        // Find Firefox binary
        let firefox_path = self.find_firefox()?;

        // Create profile directory if it doesn't exist
        std::fs::create_dir_all(&self.profile_path)?;

        // Launch Firefox with remote debugging
        let mut cmd = tokio::process::Command::new(&firefox_path);
        cmd.arg("--profile")
            .arg(&self.profile_path)
            .arg("--remote-debugging-port")
            .arg(self.bidi_port.to_string())
            .arg("--marionette")
            .arg("about:blank");

        let child = cmd
            .spawn()
            .context("Failed to spawn Firefox")?;

        {
            let mut process = self.process.lock().await;
            *process = Some(child);
        }

        // Wait a bit for Firefox to start
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

        // Connect to BiDi/CDP
        self.connect_bidi().await?;

        Ok(())
    }

    fn find_firefox(&self) -> Result<PathBuf> {
        if cfg!(target_os = "macos") {
            // Try common macOS locations
            let paths = vec![
                "/Applications/Firefox.app/Contents/MacOS/firefox",
                "/usr/local/bin/firefox",
            ];
            for path in paths {
                if PathBuf::from(path).exists() {
                    return Ok(PathBuf::from(path));
                }
            }
        } else if cfg!(target_os = "linux") {
            // Try common Linux locations
            if let Ok(output) = Command::new("which").arg("firefox").output() {
                if let Ok(path_str) = String::from_utf8(output.stdout) {
                    let path = PathBuf::from(path_str.trim());
                    if path.exists() {
                        return Ok(path);
                    }
                }
            }
        } else if cfg!(target_os = "windows") {
            // Try common Windows locations
            let paths = vec![
                r"C:\Program Files\Mozilla Firefox\firefox.exe",
                r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
            ];
            for path in paths {
                if PathBuf::from(path).exists() {
                    return Ok(PathBuf::from(path));
                }
            }
        }

        anyhow::bail!("Firefox not found. Please install Firefox.");
    }

    async fn connect_bidi(&mut self) -> Result<()> {
        // Connect to WebDriver BiDi endpoint
        let url = format!("http://localhost:{}/json/version", self.bidi_port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let version: Value = response.json().await?;
            self.session_id = version
                .get("sessionId")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // Create new BiDi session
            let session_url = format!("http://localhost:{}/session", self.bidi_port);
            let session_response = client
                .post(&session_url)
                .json(&json!({
                    "capabilities": {
                        "alwaysMatch": {
                            "browserName": "firefox",
                            "acceptInsecureCerts": true
                        }
                    }
                }))
                .send()
                .await?;

            if session_response.status().is_success() {
                let session_data: Value = session_response.json().await?;
                self.session_id = session_data
                    .get("value")
                    .and_then(|v| v.get("sessionId"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                return Ok(());
            }
        }

        // Fallback: Use CDP (Chrome DevTools Protocol) if BiDi fails
        self.connect_cdp().await
    }

    async fn connect_cdp(&mut self) -> Result<()> {
        // CDP fallback implementation
        let url = format!("http://localhost:{}/json/version", self.bidi_port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let _version: Value = response.json().await?;
            // For CDP, we'll use the existing session
            return Ok(());
        }

        anyhow::bail!("Failed to connect to Firefox debugging protocol");
    }

    pub async fn open_tab(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::new();
        // Use CDP endpoint for Firefox
        let tabs_url = format!("http://localhost:{}/json/new?{}", self.bidi_port, url);
        let response = client.get(&tabs_url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            let tab_id = data
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .context("Failed to get tab ID")?;
            Ok(tab_id)
        } else {
            anyhow::bail!("Failed to open tab: {}", response.status())
        }
    }

    pub async fn close_tab(&self, tab_id: &str) -> Result<()> {
        let client = reqwest::Client::new();
        // Use CDP close endpoint
        let close_url = format!("http://localhost:{}/json/close/{}", self.bidi_port, tab_id);
        let response = client.post(&close_url).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!("Failed to close tab: {}", response.status())
        }
    }

    pub async fn navigate(&self, tab_id: &str, url: &str) -> Result<()> {
        let client = reqwest::Client::new();
        // Use CDP navigate endpoint
        let navigate_url = format!("http://localhost:{}/json/runtime/evaluate", self.bidi_port);
        let script = format!("window.location.href = '{}'", url);
        let response = client
            .post(&navigate_url)
            .json(&json!({
                "id": tab_id,
                "expression": script
            }))
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            // Fallback: use about:blank and then navigate
            let new_tab_url = format!("http://localhost:{}/json/new?{}", self.bidi_port, url);
            client.get(&new_tab_url).send().await?;
            Ok(())
        }
    }

    pub async fn list_tabs(&self) -> Result<Vec<TabInfo>> {
        let client = reqwest::Client::new();
        let tabs_url = format!("http://localhost:{}/json/list", self.bidi_port);
        let response = client.get(&tabs_url).send().await?;

        if response.status().is_success() {
            let tabs: Vec<Value> = response.json().await?;
            let mut result = Vec::new();

            for tab in tabs {
                result.push(TabInfo {
                    id: tab
                        .get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default(),
                    title: tab
                        .get("title")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default(),
                    url: tab
                        .get("url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default(),
                    audible: tab
                        .get("audible")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                });
            }

            Ok(result)
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_tab_info(&self, tab_id: &str) -> Result<TabInfo> {
        let tabs = self.list_tabs().await?;
        tabs.into_iter()
            .find(|t| t.id == tab_id)
            .context("Tab not found")
    }

    pub async fn is_running(&self) -> bool {
        let mut process = self.process.lock().await;
        if let Some(ref mut child) = *process {
            match child.try_wait() {
                Ok(Some(_)) => false, // Process has exited
                Ok(None) => true,      // Process is still running
                Err(_) => false,      // Error checking status
            }
        } else {
            false
        }
    }

    pub async fn terminate(&mut self) {
        let mut process = self.process.lock().await;
        if let Some(mut child) = process.take() {
            let _ = child.kill().await;
        }
    }
}

