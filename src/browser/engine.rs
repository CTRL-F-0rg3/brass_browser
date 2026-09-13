use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};

pub struct BrowserEngine {
    pub current_url: String,
    pub width: u32,
    pub height: u32,
}

impl BrowserEngine {
    pub fn new(initial_url: &str) -> Self {
        Self {
            current_url: initial_url.to_string(),
            width: 1280,
            height: 720,
        }
    }

    pub fn run(&mut self) {
        println!("[Engine] Uruchamianie z wry...");
        
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Brass Browser")
            .with_inner_size(tao::dpi::LogicalSize::new(self.width as f64, self.height as f64))
            .build(&event_loop)
            .unwrap();

        let url = if self.current_url.starts_with("http") {
            self.current_url.clone()
        } else {
            format!("https://{}", self.current_url)
        };

        println!("[Engine] Ładowanie: {}", url);

        let webview = WebViewBuilder::new(&window)
            .unwrap()
            .with_url(&url)
            .unwrap()
            .build()
            .unwrap();

        println!("[Engine] ✓ WebView utworzony!");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let _ = webview.resize();
                }
                _ => {}
            }
        });
    }

    pub fn navigate(&mut self, url: &str) {
        self.current_url = url.to_string();
        println!("[Engine] Nawigacja do: {}", url);
    }
}