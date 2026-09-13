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
    browser_rect: Option<egui::Rect>,
    last_load_time: std::time::Instant,
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
            browser_rect: None,
            last_load_time: std::time::Instant::now(),
        }
    }

    fn update_browser_frame(&mut self) -> bool {
        // Renderuj gdy: strona się ładuje LUB Servo potrzebuje LUB minęło < 30s od ostatniej nawigacji
        let should_render = self.engine.is_loading() 
            || self.engine.needs_repaint() 
            || self.last_load_time.elapsed().as_secs() < 30;
        
        if should_render {
            if let Some(pixels) = self.engine.render_frame() {
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [self.frame_width, self.frame_height],
                    &pixels,
                );
                if let Some(texture) = &mut self.gl_texture_handle {
                    texture.set(image, egui::TextureOptions::LINEAR);
                }
                self.engine.clear_repaint_flag();
                return true;
            }
        }
        false
    }
    
    fn is_mouse_over_browser(&self, pos: egui::Pos2) -> bool {
        if let Some(rect) = self.browser_rect {
            rect.contains(pos)
        } else {
            false
        }
    }
}

impl eframe::App for BrassBrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(new_url) = self.navbar.render(ctx) {
            self.engine.navigate(&new_url);
            self.last_load_time = std::time::Instant::now();
        }

        let raw_events = ctx.input(|i| i.raw.events.clone());
        let pointer_pos = ctx.input(|i| i.pointer.latest_pos());

        for event in raw_events {
            match event {
                egui::Event::PointerMoved(pos) => {
                    if self.is_mouse_over_browser(pos) {
                        if let Some(rect) = self.browser_rect {
                            self.engine.handle_mouse_move(pos.x - rect.min.x, pos.y - rect.min.y);
                        }
                    }
                }
                egui::Event::PointerButton { pos, button, pressed, .. } => {
                    if self.is_mouse_over_browser(pos) {
                        if let Some(rect) = self.browser_rect {
                            let btn = match button {
                                egui::PointerButton::Primary => 0,
                                egui::PointerButton::Secondary => 1,
                                egui::PointerButton::Middle => 2,
                                _ => 255,
                            };
                            self.engine.handle_mouse_button(btn, pressed, pos.x - rect.min.x, pos.y - rect.min.y);
                        }
                    }
                }
                egui::Event::Scroll(delta) => {
                    if let Some(pos) = pointer_pos {
                        if self.is_mouse_over_browser(pos) {
                            if let Some(rect) = self.browser_rect {
                                self.engine.handle_wheel(delta.x, delta.y, pos.x - rect.min.x, pos.y - rect.min.y);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        self.update_browser_frame();

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            
            // Dopasuj rozmiar renderingu Servo do rzeczywistego rozmiaru okna
            if available_size.x as u32 != self.frame_width as u32 || 
               available_size.y as u32 != self.frame_height as u32 {
                self.frame_width = available_size.x as usize;
                self.frame_height = available_size.y as usize;
                self.engine.resize(available_size.x as u32, available_size.y as u32);
            }
            
            let response = ui.add_sized(
                available_size,
                egui::Image::new(self.gl_texture_handle.as_ref().unwrap())
                    .fit_to_exact_size(available_size)
            );
            self.browser_rect = Some(response.rect);
        });

        // Renderuj gdy strona się ładuje lub są zmiany
        if self.engine.is_loading() || self.engine.needs_repaint() || self.last_load_time.elapsed().as_secs() < 30 {
            ctx.request_repaint();
        }
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