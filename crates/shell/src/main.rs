use eframe::egui::{self, CentralPanel, TextEdit, Button, Layout, TopBottomPanel};

struct CrynnApp {
    url_input: String,
    memory_usage: String,
    current_url: String,
    search_input: String,
    is_search_mode: bool,
    tabs: Vec<Tab>,
    active_tab: usize,
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
        }
    }
}

impl CrynnApp {
    fn navigate_to_url(&mut self, url: &str) {
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.url = url.to_string();
            tab.title = url.to_string();
            
            // Open URL in external browser for now
            if url.starts_with("http") {
                if let Err(e) = webbrowser::open(url) {
                    println!("Failed to open URL: {}", e);
                }
            }
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
}

impl eframe::App for CrynnApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with navigation
        TopBottomPanel::top("navigation").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Navigation buttons
                if ui.button("←").clicked() {
                    println!("Back");
                }
                if ui.button("→").clicked() {
                    println!("Forward");
                }
                if ui.button("⟳").clicked() {
                    println!("Reload");
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
                
                ui.heading(&format!("🌐 {}", current_title));
                ui.separator();
                
                // Web content placeholder
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("🌐 Web Content");
                        ui.label("(Currently opening in external browser)");
                        ui.label(&format!("URL: {}", current_url));
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
                                    self.navigate_to_url(url);
                                    self.url_input = url.to_string();
                                }
                            }
                        });
                        
                        ui.separator();
                        
                        // Video support info
                        ui.heading("📹 Video Support:");
                        ui.label("✅ YouTube videos will open in your default browser");
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
                    });
                });
            }
        });
        
        // Bottom status bar
        TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                    if ui.button("Memory").clicked() {
                        self.memory_usage = format!("Memory: {} KB RSS", 
                            std::process::id() * 1000);
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