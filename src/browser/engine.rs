use servo::{
    DeviceIntPoint, DeviceIntRect, DevicePoint, EventLoopWaker, InputEvent, MouseButton,
    MouseButtonAction, RenderingContext, Servo, ServoBuilder,
    SoftwareRenderingContext, WebView, WebViewBuilder, WebViewDelegate, WebViewPoint,
    WheelDelta, WheelMode,
};
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

struct EguiEventLoopWaker {
    needs_repaint: Arc<AtomicBool>,
}

impl EventLoopWaker for EguiEventLoopWaker {
    fn wake(&self) {
        self.needs_repaint.store(true, Ordering::SeqCst);
    }

    fn clone_box(&self) -> Box<dyn EventLoopWaker> {
        Box::new(Self {
            needs_repaint: Arc::clone(&self.needs_repaint),
        })
    }
}

struct BrowserWebViewDelegate {
    needs_repaint: Arc<AtomicBool>,
}

impl WebViewDelegate for BrowserWebViewDelegate {
    fn notify_new_frame_ready(&self, webview: WebView) {
        self.needs_repaint.store(true, Ordering::SeqCst);
        webview.paint();
    }
}

pub struct BrowserEngine {
    current_url: String,
    servo: Option<Servo>,
    webview: Option<WebView>,
    rendering_context: Option<Rc<SoftwareRenderingContext>>,
    needs_repaint: Arc<AtomicBool>,
    pub width: u32,
    pub height: u32,
}

impl BrowserEngine {
    pub fn new(initial_url: &str) -> Self {
        Self {
            current_url: initial_url.to_string(),
            servo: None,
            webview: None,
            rendering_context: None,
            needs_repaint: Arc::new(AtomicBool::new(false)),
            width: 1280,
            height: 720,
        }
    }

    pub fn init(&mut self) {
        println!("[Engine] Inicjalizacja Servo...");

        let needs_repaint = Arc::clone(&self.needs_repaint);
        let _waker = EguiEventLoopWaker {
            needs_repaint: needs_repaint.clone(),
        };

        let builder = ServoBuilder::default();
        let servo_inst = builder.build();
        println!("[Engine] ✓ Servo zbudowane!");

        let size = dpi::PhysicalSize::new(self.width, self.height);
        let rendering_context = SoftwareRenderingContext::new(size)
            .expect("Nie udało się utworzyć SoftwareRenderingContext");
        let rc_context: Rc<SoftwareRenderingContext> = Rc::new(rendering_context);
        println!("[Engine] ✓ RenderingContext utworzony!");

        let delegate = BrowserWebViewDelegate {
            needs_repaint: needs_repaint.clone(),
        };

        let rc_trait: Rc<dyn RenderingContext> = rc_context.clone();
        let webview_builder = WebViewBuilder::new(&servo_inst, rc_trait)
            .delegate(Rc::new(delegate));

        let webview = webview_builder.build();
        println!("[Engine] ✓ WebView utworzony!");

        let initial_url = self.current_url.clone();
        self.servo = Some(servo_inst);
        self.webview = Some(webview);
        self.rendering_context = Some(rc_context);

        self.navigate(&initial_url);
    }

    pub fn navigate(&mut self, url: &str) {
        let formatted_url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        println!("[Engine] Nawigacja do: {}", formatted_url);
        self.current_url = formatted_url.clone();

        if let (Some(webview), Some(servo)) = (&self.webview, &self.servo) {
            if let Ok(parsed_url) = url::Url::parse(&formatted_url) {
                println!("[Engine] URL sparsowany: {}", parsed_url);
                webview.load(parsed_url);
                servo.spin_event_loop();
            } else {
                eprintln!("[Engine] Błąd parsowania URL: {}", formatted_url);
            }
        }
    }

    pub fn current_url(&self) -> &str {
        &self.current_url
    }

    pub fn needs_repaint(&self) -> bool {
        self.needs_repaint.load(Ordering::SeqCst)
    }

    pub fn clear_repaint_flag(&self) {
        self.needs_repaint.store(false, Ordering::SeqCst);
    }

    /// Obsługa kliknięć myszy
    pub fn handle_mouse_button(&mut self, button: u8, pressed: bool, x: f32, y: f32) {
        if let (Some(webview), Some(servo)) = (&self.webview, &self.servo) {
            let servo_button = match button {
                0 => MouseButton::Primary,
                1 => MouseButton::Secondary,
                2 => MouseButton::Auxiliary,
                _ => return,
            };

            let action = if pressed {
                MouseButtonAction::Down
            } else {
                MouseButtonAction::Up
            };

            // NAPRAWA: Używamy DevicePoint (f32) zamiast DeviceIntPoint (i32)
            let point = WebViewPoint::Device(DevicePoint::new(x, y));

            let event = InputEvent::MouseButton(servo::MouseButtonEvent {
                action,
                button: servo_button,
                point,
            });

            webview.notify_input_event(event);
            servo.spin_event_loop();
        }
    }

    /// Obsługa ruchu myszy
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if let (Some(webview), Some(servo)) = (&self.webview, &self.servo) {
            // NAPRAWA: Używamy DevicePoint (f32)
            let point = WebViewPoint::Device(DevicePoint::new(x, y));
            
            // NAPRAWA: Dodano brakujące pole is_compatibility_event_for_touch
            let event = InputEvent::MouseMove(servo::MouseMoveEvent {
                point,
                is_compatibility_event_for_touch: false,
            });

            webview.notify_input_event(event);
            servo.spin_event_loop();
        }
    }

    /// Obsługa scrollowania
    pub fn handle_wheel(&mut self, delta_x: f32, delta_y: f32, x: f32, y: f32) {
        if let (Some(webview), Some(servo)) = (&self.webview, &self.servo) {
            // NAPRAWA: Używamy DevicePoint (f32)
            let point = WebViewPoint::Device(DevicePoint::new(x, y));
            
            let event = InputEvent::Wheel(servo::WheelEvent {
                delta: WheelDelta {
                    x: delta_x as f64,
                    y: delta_y as f64,
                    z: 0.0,
                    mode: WheelMode::DeltaLine,
                },
                point,
            });

            webview.notify_input_event(event);
            servo.spin_event_loop();
        }
    }

    /// Zmiana rozmiaru okna
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        if let Some(context) = &self.rendering_context {
            let new_size = dpi::PhysicalSize::new(width, height);
            context.resize(new_size);
            
            if let Some(webview) = &self.webview {
                webview.paint();
            }
            if let Some(servo) = &self.servo {
                servo.spin_event_loop();
            }
        }
    }

    pub fn render_frame(&mut self) -> Option<Vec<u8>> {
        if let (Some(webview), Some(servo), Some(context)) = (
            &self.webview,
            &self.servo,
            &self.rendering_context,
        ) {
            webview.paint();
            servo.spin_event_loop();
            context.present();

            let min_point = DeviceIntPoint::new(0, 0);
            let max_point = DeviceIntPoint::new(self.width as i32, self.height as i32);
            let rect = DeviceIntRect::new(min_point, max_point);

            if let Some(rgba_image) = context.read_to_image(rect) {
                return Some(rgba_image.into_raw());
            }
        }
        None
    }
}