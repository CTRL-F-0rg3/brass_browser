mod browser;
mod ui;

use browser::engine::BrowserEngine;
use ui::navbar::NavBar;

use eframe::egui;

struct BrassBrowserApp {
    engine: BrowserEngine,
    navbar: NavBar,
    gl_texture_handle: Option<egui::TextureHandle>,
    frame_width: usize,
    frame_height: usize,
}

impl BrassBrowserApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut engine = BrowserEngine::new("https://trangorgeos.website");
        engine.init();

        let navbar = NavBar::new(engine.current_url());
        
        let width = 1280;
        let height = 720;

        let pixels = vec![255u8; width * height * 4];
        let image = egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels);

        let handle = cc.egui_ctx.load_texture("browser_canvas", image, egui::TextureOptions::LINEAR);

        Self {
            engine,
            navbar,
            gl_texture_handle: Some(handle),
            frame_width: width,
            frame_height: height,
        }
    }

    fn update_browser_frame(&mut self) {
        if let Some(servo) = &mut self.engine.servo {
            // Przetwarzanie zdarzeń układu i potoku WebRendera Servo
        }
    }
}

impl eframe::App for BrassBrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(new_url) = self.navbar.render(ctx) {
            self.engine.navigate(&new_url);
        }

        self.update_browser_frame();

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            if let Some(texture) = &self.gl_texture_handle {
                ui.add(
                    egui::Image::new(texture)
                        .fit_to_exact_size(available_size)
                );
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Brass Browser"),
        ..Default::default()
    };

    println!("[Brass Browser] Uruchamianie...");

    eframe::run_native(
        "Brass Browser",
        options,
        Box::new(|cc| Box::new(BrassBrowserApp::new(cc))),
    )
}