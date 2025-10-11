use anyhow::Result;
use crynn_vpn::{VpnManager as CrynnVpnManager, VpnConfig, ProviderType, VpnCredentials, ServerLocation};

pub struct VpnManager {
    manager: Option<CrynnVpnManager>,
}

impl VpnManager {
    pub async fn new() -> Result<Self> {
        let config = VpnConfig::default();
        let manager = CrynnVpnManager::new(config)?;
        Ok(Self {
            manager: Some(manager),
        })
    }

    pub async fn connect_surfshark(&self) -> Result<()> {
        println!("Connecting to Surfshark VPN");
        
        // TODO: Show server selection dialog
        // TODO: Authenticate with Surfshark
        // TODO: Connect to selected server
        
        Ok(())
    }

    pub async fn connect_nordvpn(&self) -> Result<()> {
        println!("Connecting to NordVPN");
        
        // TODO: Show server selection dialog
        // TODO: Authenticate with NordVPN
        // TODO: Connect to selected server
        
        Ok(())
    }

    pub async fn connect_protonvpn(&self) -> Result<()> {
        println!("Connecting to ProtonVPN");
        
        // TODO: Show server selection dialog
        // TODO: Authenticate with ProtonVPN
        // TODO: Connect to selected server
        
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        println!("Disconnecting VPN");
        
        if let Some(manager) = &self.manager {
            manager.disconnect().await?;
        }
        
        Ok(())
    }

    pub async fn get_status(&self) -> Result<crynn_vpn::ConnectionStatus> {
        if let Some(manager) = &self.manager {
            Ok(manager.get_status().await)
        } else {
            Ok(crynn_vpn::ConnectionStatus::Disconnected)
        }
    }

    pub async fn get_memory_usage(&self) -> Result<usize> {
        if let Some(manager) = &self.manager {
            manager.get_memory_usage().await
        } else {
            Ok(0)
        }
    }

    pub async fn test_dns_leak(&self) -> Result<bool> {
        if let Some(manager) = &self.manager {
            Ok(manager.test_dns_leak().await?)
        } else {
            Ok(true) // No VPN, no leak
        }
    }

    pub async fn enable_kill_switch(&self) -> Result<()> {
        if let Some(manager) = &self.manager {
            manager.enable_kill_switch().await?;
        }
        Ok(())
    }

    pub async fn disable_kill_switch(&self) -> Result<()> {
        if let Some(manager) = &self.manager {
            manager.disable_kill_switch().await?;
        }
        Ok(())
    }
}
