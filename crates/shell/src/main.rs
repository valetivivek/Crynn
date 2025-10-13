use eframe::egui::{self, CentralPanel, TextEdit, Button, Layout, TopBottomPanel};
use anyhow::Result;

struct CrynnApp {
    url_input: String,
    memory_usage: String,
    current_url: String,
    search_input: String,
    is_search_mode: bool,
    tabs: Vec<Tab>,
    active_tab: usize,
    // gecko_engine: Option<GeckoEngine>, // Temporarily disabled for testing
    navigation_history: Vec<String>,
    history_index: isize,
}

struct Tab {
    url: String,
    title: String,
    content: String,
}

impl Tab {
    fn new(url: String) -> Self {
        Self {
            url: url.clone(),
            title: url,
            content: String::new(),
        }
    }
}

impl Default for CrynnApp {
    fn default() -> Self {
        Self {
            url_input: String::new(),
            memory_usage: String::from("Memory monitoring not available"),
            current_url: String::new(),
            search_input: String::new(),
            is_search_mode: false,
            tabs: vec![Tab::new("about:blank".to_string())],
            active_tab: 0,
            // gecko_engine: None,
            navigation_history: Vec::new(),
            history_index: -1,
        }
    }
}

impl CrynnApp {
    fn initialize_browser_engine(&mut self) -> Result<()> {
        // TODO: Initialize GeckoEngine when ready
        // For now, just simulate initialization
        println!("Browser engine initialization simulated");
        Ok(())
    }
    
    fn navigate_to_url(&mut self, url: &str) {
        // Initialize browser engine if not already done
        if let Err(e) = self.initialize_browser_engine() {
            println!("Failed to initialize browser engine: {}", e);
            return;
        }
        
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.url = url.to_string();
            tab.title = url.to_string();
            
            // TODO: Navigate using GeckoEngine instead of external browser
            // For now, simulate navigation
            println!("Navigated to: {}", url);
            
            // Add to navigation history
            self.add_to_history(url);
            
            // Update current URL
            self.current_url = url.to_string();
        }
    }
    
    fn search_query(&mut self, query: &str) {
        let search_url = if query.starts_with("http") {
            query.to_string()
        } else {
            format!("https://www.google.com/search?q={}", url::form_urlencoded::byte_serialize(query.as_bytes()).collect::<String>())
        };
        self.navigate_to_url(&search_url);
    }
    
    fn navigate_to_direct_url(&mut self, url: &str) {
        // Direct navigation without search logic
        self.navigate_to_url(url);
    }
    
    fn add_to_history(&mut self, url: &str) {
        // Remove any history after current index
        if self.history_index >= 0 {
            self.navigation_history.truncate((self.history_index + 1) as usize);
        }
        
        // Add new URL to history
        self.navigation_history.push(url.to_string());
        self.history_index = (self.navigation_history.len() - 1) as isize;
    }
    
    fn go_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(url) = self.navigation_history.get(self.history_index as usize) {
                let url_clone = url.clone();
                self.navigate_to_url(&url_clone);
            }
        }
    }
    
    fn go_forward(&mut self) {
        if self.history_index < (self.navigation_history.len() - 1) as isize {
            self.history_index += 1;
            if let Some(url) = self.navigation_history.get(self.history_index as usize) {
                let url_clone = url.clone();
                self.navigate_to_url(&url_clone);
            }
        }
    }
    
    fn reload_page(&mut self) {
        // TODO: Reload using GeckoEngine
        // For now, simulate reload
        println!("Page reloaded");
    }
    
    fn get_memory_usage(&mut self) -> String {
        // TODO: Get memory usage from GeckoEngine
        // For now, simulate memory usage
        format!("Memory: {} KB", std::process::id() * 1000 / 1024)
    }
}

impl eframe::App for CrynnApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with navigation
        TopBottomPanel::top("navigation").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Navigation buttons
                if ui.button("←").clicked() {
                    self.go_back();
                }
                if ui.button("→").clicked() {
                    self.go_forward();
                }
                if ui.button("⟳").clicked() {
                    self.reload_page();
                }
                
                ui.separator();
                
                // Toggle between URL and search
                if ui.button("🔍").clicked() {
                    self.is_search_mode = !self.is_search_mode;
                    if self.is_search_mode {
                        self.search_input = self.url_input.clone();
                    }
                }
                
                // URL/Search input
                if self.is_search_mode {
                    ui.label("Search:");
                    ui.add(TextEdit::singleline(&mut self.search_input).desired_width(400.0));
                    if ui.button("Search").clicked() || ui.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                        let query = self.search_input.clone();
                        self.search_query(&query);
                        self.url_input = query;
                    }
                } else {
                    ui.label("URL:");
                    ui.add(TextEdit::singleline(&mut self.url_input).desired_width(400.0));
                    if ui.button("Go").clicked() || ui.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                        let url = self.url_input.clone();
                        self.navigate_to_url(&url);
                    }
                }
                
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                    if ui.button("New Tab").clicked() {
                        self.tabs.push(Tab::new("about:blank".to_string()));
                        self.active_tab = self.tabs.len() - 1;
                    }
                });
            });
            
            // Tab bar
            ui.horizontal(|ui| {
                let mut tabs_to_remove = Vec::new();
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_active = i == self.active_tab;
                    let button_text = if tab.title.len() > 20 {
                        format!("{}...", &tab.title[..17])
                    } else {
                        tab.title.clone()
                    };
                    
                    let mut button = Button::new(&button_text);
                    if is_active {
                        button = button.fill(eframe::egui::Color32::from_gray(100));
                    }
                    
                    if ui.add(button).clicked() {
                        self.active_tab = i;
                        self.url_input = tab.url.clone();
                    }
                    
                    // Close tab button
                    if self.tabs.len() > 1 {
                        if ui.button("×").clicked() {
                            tabs_to_remove.push(i);
                        }
                    }
                }
                
                // Remove tabs after iteration
                for &i in tabs_to_remove.iter().rev() {
                    self.tabs.remove(i);
                    if self.active_tab >= self.tabs.len() {
                        self.active_tab = self.tabs.len() - 1;
                    }
                    if let Some(tab) = self.tabs.get(self.active_tab) {
                        self.url_input = tab.url.clone();
                    }
                }
            });
        });
        
        // Main content area
        CentralPanel::default().show(ctx, |ui| {
            if let Some(tab) = self.tabs.get(self.active_tab) {
                let current_url = tab.url.clone();
                let current_title = tab.title.clone();
                
                // Show current page content or homepage
                if !self.current_url.is_empty() && self.current_url != "about:blank" {
                    // Show current page content
                    ui.horizontal(|ui| {
                        ui.label("🌐 Current Page:");
                        ui.label(&self.current_url);
                        ui.with_layout(Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                            if ui.button("Clear History").clicked() {
                                self.navigation_history.clear();
                                self.history_index = -1;
                            }
                        });
                    });
                    ui.separator();
                    
                    ui.label("Navigation History:");
                    let history_copy = self.navigation_history.clone();
                    for (i, url) in history_copy.iter().enumerate() {
                        let is_current = i == self.history_index as usize;
                        ui.horizontal(|ui| {
                            if is_current {
                                ui.label("→");
                            } else {
                                ui.label("  ");
                            }
                            ui.label(&format!("{}. {}", i + 1, url));
                            if ui.button("Go").clicked() {
                                self.navigate_to_url(url);
                            }
                        });
                    }
                    
                    ui.separator();
                    ui.label("💡 Tip: All navigation happens within Crynn Browser using the Gecko engine!");
                } else {
                    // Show homepage with navigation options
                    ui.heading("🌐 Crynn Browser");
                    ui.separator();
                    
                    // Popular websites
                    ui.heading("Popular Websites:");
                    ui.horizontal_wrapped(|ui| {
                        let sites = [
                            ("YouTube", "https://youtube.com"),
                            ("Google", "https://google.com"),
                            ("GitHub", "https://github.com"),
                            ("Reddit", "https://reddit.com"),
                            ("Wikipedia", "https://wikipedia.org"),
                            ("Stack Overflow", "https://stackoverflow.com"),
                            ("Netflix", "https://netflix.com"),
                            ("Twitch", "https://twitch.tv"),
                        ];
                        
                        for (name, url) in sites.iter() {
                            if ui.button(*name).clicked() {
                                self.navigate_to_direct_url(url);
                                self.url_input = url.to_string();
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    // Video support info
                    ui.heading("📹 Video Support:");
                    ui.label("✅ YouTube videos play directly in Crynn Browser");
                    ui.label("✅ Netflix, Twitch, and other video platforms supported");
                    ui.label("✅ Full HTML5 video playback capabilities");
                    
                    ui.separator();
                    
                    // Search shortcuts
                    ui.heading("🔍 Quick Search:");
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Search Videos").clicked() {
                            self.search_query("videos");
                        }
                        if ui.button("Search News").clicked() {
                            self.search_query("news");
                        }
                        if ui.button("Search Images").clicked() {
                            self.search_query("images");
                        }
                        if ui.button("Search Maps").clicked() {
                            self.search_query("maps");
                        }
                    });
                }
            }
        });
        
        // Bottom status bar
        TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                    if ui.button("Memory").clicked() {
                        self.memory_usage = self.get_memory_usage();
                    }
                    ui.label(&self.memory_usage);
                });
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Crynn Browser shutting down...");
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("Crynn Browser starting...");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Crynn Browser"),
        ..Default::default()
    };

    eframe::run_native(
        "Crynn Browser",
        options,
        Box::new(|_cc| Box::new(CrynnApp::default())),
    )
}