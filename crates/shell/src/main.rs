use std::process::{Command, Stdio}; //launch the firefox browser
use std::time::Duration;
use std::thread::sleep;
use std::fs;
use std::path::PathBuf;

use crynn_engine_bridge::Engine; //connect the firefox via BiDi(Bidirectional)
use anyhow::Result;


// setting the path where the firefox esr is located to launch the engine
fn firefox_path() -> PathBuf {
    PathBuf::from(r"C:\Users\valeti.v\AppData\Local\Mozilla Firefox\firefox.exe")
}
// creating a profile for the crynn for launching the firefox using crynn preferences
fn firefox_profile_path() -> PathBuf {
    let dir = std::env::temp_dir().join("crynn_profile");
    fs::create_dir_all(&dir).ok();
    // copy crynn.prefs into prefs.js
    let prefs_src = PathBuf::from("build/firefox_prefs/crynn.prefs");
    let prefs_dst = dir.join("prefs.js");
    if let Ok(prefs) = fs::read_to_string(prefs_src) {
        fs::write(prefs_dst, prefs).ok();
    }
    dir
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let profile = firefox_profile_path();
    let firefox = firefox_path();

    // Start Firefox with BiDi debugging port
    let mut child = Command::new(firefox)
        .args([
            "--remote-debugging-port=9222",
            "--no-remote",
            "--marionette",
            "--headless", // Remove if you want visible window
            "--profile",
        ])
        .arg(profile)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Wait for Firefox to be ready
    sleep(Duration::from_secs(2));

    // Connect to BiDi WebSocket
    let engine = Engine::connect("ws://localhost:9222".to_string()).await?;

    // Try to create a new tab and navigate
    let tab = engine.new_tab().await?;
    engine.navigate(&tab, "https://example.org").await?;

    println!("Engine launched and navigated.");
    child.wait()?; // keep process alive
    Ok(())
}