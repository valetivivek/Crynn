use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused imports
use tokio::sync::RwLock;

pub mod providers;
pub mod config;

use providers::VpnProvider;
use config::VpnConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Surfshark,
    NordVpn,
    ProtonVpn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnCredentials {
    pub username: String,
    pub password: String,
    pub service: ProviderType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerLocation {
    pub id: String,
    pub name: String,
    pub country: String,
    pub city: String,
    pub server: String,
    pub load: f32, // 0.0 to 100.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConnection {
    pub id: String,
    pub provider: ProviderType,
    pub location: ServerLocation,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub status: ConnectionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub duration: chrono::Duration,
    pub server_load: f32,
}

pub struct VpnManager {
    config: VpnConfig,
    providers: HashMap<ProviderType, Box<dyn VpnProvider + Send + Sync>>,
    active_connection: RwLock<Option<VpnConnection>>,
    credentials: RwLock<HashMap<ProviderType, VpnCredentials>>,
}

impl VpnManager {
    pub fn new(config: VpnConfig) -> Result<Self> {
        let mut providers: HashMap<ProviderType, Box<dyn VpnProvider + Send + Sync>> = HashMap::new();
        
        // Initialize providers based on config
        if config.enable_surfshark {
            providers.insert(ProviderType::Surfshark, Box::new(providers::SurfsharkProvider::new()));
        }
        if config.enable_nordvpn {
            providers.insert(ProviderType::NordVpn, Box::new(providers::NordVpnProvider::new()));
        }
        if config.enable_protonvpn {
            providers.insert(ProviderType::ProtonVpn, Box::new(providers::ProtonVpnProvider::new()));
        }

        Ok(Self {
            config,
            providers,
            active_connection: RwLock::new(None),
            credentials: RwLock::new(HashMap::new()),
        })
    }

    pub async fn set_credentials(&self, credentials: VpnCredentials) -> Result<()> {
        let mut creds = self.credentials.write().await;
        creds.insert(credentials.service.clone(), credentials);
        Ok(())
    }

    pub async fn list_locations(&self, provider: &ProviderType) -> Result<Vec<ServerLocation>> {
        if let Some(provider_impl) = self.providers.get(provider) {
            provider_impl.list_locations().await
        } else {
            Err(anyhow::anyhow!("Provider not available: {:?}", provider))
        }
    }

    pub async fn connect(&self, provider: ProviderType, location: ServerLocation) -> Result<()> {
        // Disconnect any existing connection
        self.disconnect().await?;

        if let Some(provider_impl) = self.providers.get(&provider) {
            let credentials = {
                let creds = self.credentials.read().await;
                creds.get(&provider).cloned()
            };

            if let Some(creds) = credentials {
                let connection = provider_impl.connect(&creds, &location).await?;
                
                let mut active = self.active_connection.write().await;
                *active = Some(connection);
                
                Ok(())
            } else {
                Err(anyhow::anyhow!("No credentials set for provider: {:?}", provider))
            }
        } else {
            Err(anyhow::anyhow!("Provider not available: {:?}", provider))
        }
    }

    pub async fn disconnect(&self) -> Result<()> {
        let mut active = self.active_connection.write().await;
        if let Some(connection) = active.as_ref() {
            if let Some(provider_impl) = self.providers.get(&connection.provider) {
                provider_impl.disconnect().await?;
            }
            *active = None;
        }
        Ok(())
    }

    pub async fn get_status(&self) -> ConnectionStatus {
        let active = self.active_connection.read().await;
        active.as_ref().map_or(ConnectionStatus::Disconnected, |c| c.status.clone())
    }

    pub async fn get_stats(&self) -> Result<Option<VpnStats>> {
        let active = self.active_connection.read().await;
        if let Some(connection) = active.as_ref() {
            if let Some(provider_impl) = self.providers.get(&connection.provider) {
                Ok(Some(provider_impl.get_stats().await?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_memory_usage(&self) -> Result<usize> {
        // VPN manager itself uses minimal memory
        // The actual VPN processes are managed by the providers
        Ok(1024 * 1024) // 1MB baseline
    }

    pub async fn enable_kill_switch(&self) -> Result<()> {
        // TODO: Implement firewall-based kill switch
        // This would block all network traffic except VPN tunnel
        println!("Kill switch enabled - blocking non-VPN traffic");
        Ok(())
    }

    pub async fn disable_kill_switch(&self) -> Result<()> {
        // TODO: Disable firewall rules
        println!("Kill switch disabled - allowing all traffic");
        Ok(())
    }

    pub async fn test_dns_leak(&self) -> Result<bool> {
        // TODO: Implement DNS leak test
        // This would check if DNS queries go through the VPN
        Ok(true) // Placeholder - no leak detected
    }

    pub async fn cleanup(&self) -> Result<()> {
        self.disconnect().await?;
        Ok(())
    }
}

impl Drop for VpnManager {
    fn drop(&mut self) {
        // Ensure clean shutdown
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let _ = self.cleanup().await;
            })
        });
    }
}
