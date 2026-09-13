use servo::{
    EventLoopWaker, RenderingContext, Servo, ServoBuilder, ServoUrl, 
    SoftwareRenderingContext, WebView, WebViewBuilder, WebViewDelegate,
};
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// 1. Implementacja EventLoopWaker
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

// 2. Implementacja WebViewDelegate
struct BrowserWebViewDelegate {
    needs_repaint: Arc<AtomicBool>,
}

impl WebViewDelegate for BrowserWebViewDelegate {
    fn notify_new_frame_ready(&self, _webview: WebView) {
        self.needs_repaint.store(true, Ordering::SeqCst);
    }
}

pub struct BrowserEngine {
    current_url: String,
    pub servo: Option<Servo>,
    pub webview: Option<WebView>,
    // Zmieniamy typ na Rc<SoftwareRenderingContext>
    rendering_context: Option<Rc<SoftwareRenderingContext>>,
    needs_repaint: Arc<AtomicBool>,
    width: u32,
    height: u32,
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

        let waker = EguiEventLoopWaker {
            needs_repaint: Arc::clone(&self.needs_repaint),
        };

        let mut builder = ServoBuilder::default();
        // Jeśli builder ma metodę z wakerem, użyj jej. W przeciwnym razie standardowy build.
        let servo_inst = builder.build();

        println!("[Engine] ✓ Servo zbudowane!");

        // 3. Tworzymy Software Rendering Context
        let size = dpi::PhysicalSize::new(self.width, self.height);
        let rendering_context = SoftwareRenderingContext::new(size)
            .expect("Nie udało się utworzyć SoftwareRenderingContext");

        // 4. Opakowujemy w Rc (Reference Counted)
        let rc_context: Rc<SoftwareRenderingContext> = Rc::new(rendering_context);

        println!("[Engine] ✓ Rendering context utworzony!");

        let delegate = BrowserWebViewDelegate {
            needs_repaint: Arc::clone(&self.needs_repaint),
        };

        // 5. Klonujemy Rc (to tylko zwiększa licznik, nie kopiuje kontekstu!)
        // i rzutujemy na Rc<dyn RenderingContext> dla WebViewBuilder
        let rc_trait: Rc<dyn RenderingContext> = rc_context.clone();

        // 6. Tworzymy WebViewBuilder (pamiętaj o kolejności: servo, context)
        let webview_builder = WebViewBuilder::new(&servo_inst, rc_trait)
            .delegate(Rc::new(delegate));

        let webview = webview_builder.build();

        println!("[Engine] ✓ WebView utworzony!");

        if let Ok(url) = ServoUrl::parse(&self.current_url) {
            println!("[Engine] URL do załadowania: {}", url);
        }

        self.servo = Some(servo_inst);
        self.webview = Some(webview);
        self.rendering_context = Some(rc_context);
    }

    pub fn navigate(&mut self, url: &str) {
        let formatted_url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };
        
        println!("[Engine] Nawigacja do: {}", formatted_url);
        self.current_url = formatted_url;
        
        if let Some(_webview) = &mut self.webview {
            if let Ok(_parsed_url) = ServoUrl::parse(&self.current_url) {
                println!("[Engine] URL sparsowany - czeka na implementację load()");
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

    /// Renderuje klatkę i zwraca piksele (RGBA)
    pub fn render_frame(&mut self) -> Option<Vec<u8>> {
        if let Some(webview) = &mut self.webview {
            // Maluj klatkę w WebView
            webview.paint();
            
            // TUTAJ BĘDZIE KLUCZOWY KROK:
            // Gdy to się skompiluje, będziemy musieli wyciągnąć piksele z self.rendering_context
            // np. self.rendering_context.as_ref().unwrap().read_pixels()
            
            // TYMCZASOWO: zwracamy szary bufor, żeby udowodnić że pętla działa
            let width = self.width as usize;
            let height = self.height as usize;
            return Some(vec![100u8; width * height * 4]);
        }
        None
    }
}