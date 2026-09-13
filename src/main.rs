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
        let mut engine = BrowserEngine::new("https://example.com");
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
        if let Some(pixels) = self.engine.render_frame() {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [self.frame_width, self.frame_height],
                &pixels,
            );
            
            if let Some(texture) = &mut self.gl_texture_handle {
                texture.set(image, egui::TextureOptions::LINEAR);
            }
        }
    }
}

impl eframe::App for BrassBrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(new_url) = self.navbar.render(ctx) {
            self.engine.navigate(&new_url);
        }

        // Pobieramy zdarzenia i pozycję kursora z egui w bezpieczny sposób
        let events = ctx.input(|i| i.raw.events.clone());
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos());

        for event in events {
            match event {
                egui::Event::PointerMoved(pos) => {
                    let y_in_browser = pos.y - 40.0; // Odliczamy wysokość navbara
                    if y_in_browser > 0.0 {
                        self.engine.handle_mouse_move(pos.x, y_in_browser);
                    }
                }
                egui::Event::PointerButton { pos, button, pressed, .. } => {
                    let y_in_browser = pos.y - 40.0;
                    if y_in_browser > 0.0 {
                        let btn = match button {
                            egui::PointerButton::Primary => 0,
                            egui::PointerButton::Secondary => 1,
                            egui::PointerButton::Middle => 2,
                            _ => 255,
                        };
                        self.engine.handle_mouse_button(btn, pressed, pos.x, y_in_browser);
                    }
                }
                egui::Event::Scroll(delta) => {
                    if let Some(pos) = pointer_pos {
                        let y_in_browser = pos.y - 40.0;
                        if y_in_browser > 0.0 {
                            self.engine.handle_wheel(delta.x, delta.y, pos.x, y_in_browser);
                        }
                    }
                }
                _ => {}
            }
        }

        self.update_browser_frame();

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            
            // Obsługa zmiany rozmiaru okna
            if available_size.x as u32 != self.frame_width as u32 || 
               available_size.y as u32 != self.frame_height as u32 {
                self.frame_width = available_size.x as usize;
                self.frame_height = available_size.y as usize;
                self.engine.resize(available_size.x as u32, available_size.y as u32);
            }
            
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