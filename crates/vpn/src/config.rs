use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    pub enable_surfshark: bool,
    pub enable_nordvpn: bool,
    pub enable_protonvpn: bool,
    pub kill_switch_enabled: bool,
    pub auto_connect: bool,
    pub preferred_provider: Option<String>,
    pub config_path: PathBuf,
}

impl Default for VpnConfig {
    fn default() -> Self {
        Self {
            enable_surfshark: true,
            enable_nordvpn: true,
            enable_protonvpn: true,
            kill_switch_enabled: false,
            auto_connect: false,
            preferred_provider: None,
            config_path: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("crynn")
                .join("vpn.json"),
        }
    }
}

impl VpnConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::default().config_path;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: VpnConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn with_surfshark(mut self, enabled: bool) -> Self {
        self.enable_surfshark = enabled;
        self
    }

    pub fn with_nordvpn(mut self, enabled: bool) -> Self {
        self.enable_nordvpn = enabled;
        self
    }

    pub fn with_protonvpn(mut self, enabled: bool) -> Self {
        self.enable_protonvpn = enabled;
        self
    }

    pub fn with_kill_switch(mut self, enabled: bool) -> Self {
        self.kill_switch_enabled = enabled;
        self
    }
}
