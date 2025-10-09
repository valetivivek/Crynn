// this is a control layer over the firefox 


use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TargetId(pub String);

#[derive(Debug, Clone)]
pub struct Engine {
    pub ws_url: String,
}

impl Engine {
    pub async fn connect(ws_url: String) -> Result<Self> {
        println!("Connecting to BiDi at {ws_url}");
        // Later: WebSocket handshake
        Ok(Self { ws_url })
    }

    pub async fn new_tab(&self) -> Result<TargetId> {
        println!("Creating new tab");
        Ok(TargetId("dummy".into()))
    }

    pub async fn navigate(&self, target: &TargetId, url: &str) -> Result<()> {
        println!("Navigating {} to {}", target.0, url);
        Ok(())
    }
}
