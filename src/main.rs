use eframe::egui;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};
use std::sync::{Arc, Mutex};

struct BrowserApp {
    url_input: String,
    current_url: String,
    webview_ready: Arc<Mutex<bool>>,
}

impl BrowserApp {
    fn new() -> Self {
        Self {
            url_input: "https://trangorgeos.website".to_string(),
            current_url: "https://trangorgeos.website".to_string(),
            webview_ready: Arc::new(Mutex::new(false)),
        }
    }

    fn navigate(&mut self, url: &str) {
        self.current_url = url.to_string();
        self.url_input = url.to_string();
        println!("[UI] Nawigacja do: {}", url);
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Navbar
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("").clicked() {
                    println!("[UI] Wstecz");
                }
                if ui.button("►").clicked() {
                    println!("[UI] Dalej");
                }
                if ui.button("🔄").clicked() {
                    println!("[UI] Odśwież");
                }
                
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut self.url_input)
                        .hint_text("Wpisz URL..."),
                );
                
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let url = self.url_input.clone();
                    self.navigate(&url);
                }
            });
        });

        // Główna zawartość - tutaj powinien być WebView
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("WebView będzie tutaj...");
            ui.label(format!("Aktualny URL: {}", self.current_url));
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    println!("[Brass Browser] Uruchamianie...");
    
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Brass Browser"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Brass Browser",
        options,
        Box::new(|_cc| Box::new(BrowserApp::new())),
    )
}