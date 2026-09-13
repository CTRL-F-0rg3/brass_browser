use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();
    println!("[Brass Browser] Uruchamianie z wry + tao...");

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("Brass Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .expect("Nie udało się utworzyć okna");

    let window = Arc::new(window);
    let webview: Arc<Mutex<Option<wry::WebView>>> = Arc::new(Mutex::new(None));
    
    let webview_clone = webview.clone();
    let window_clone = window.clone();

    let html = r#"<!DOCTYPE html>
<html style="height: 100%; margin: 0; padding: 0; overflow: hidden;">
<head>
    <meta charset="UTF-8">
    <style>
        body {
            margin: 0;
            padding: 0;
            height: 100%;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1e1e1e;
            overflow: hidden;
        }
        #navbar {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 40px;
            background: #2d2d2d;
            display: flex;
            align-items: center;
            padding: 0 8px;
            gap: 8px;
            border-bottom: 1px solid #404040;
            z-index: 1000;
        }
        #navbar button {
            background: #404040;
            border: none;
            color: #fff;
            padding: 6px 12px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            height: 28px;
        }
        #navbar button:hover { background: #505050; }
        #urlbar {
            flex: 1;
            padding: 6px 12px;
            border: 1px solid #404040;
            border-radius: 4px;
            background: #1e1e1e;
            color: #fff;
            font-size: 14px;
            height: 28px;
        }
        #urlbar:focus { outline: none; border-color: #0066cc; }
        #content {
            position: absolute;
            top: 40px;
            left: 0;
            right: 0;
            bottom: 0;
            background: #fff;
        }
        #browser {
            width: 100%;
            height: 100%;
            border: none;
        }
    </style>
</head>
<body>
    <div id="navbar">
        <button id="backBtn" title="Wstecz">◄</button>
        <button id="forwardBtn" title="Dalej">►</button>
        <button id="refreshBtn" title="Odśwież">🔄</button>
        <input type="text" id="urlbar" value="https://trangorgeos.website" placeholder="Wpisz URL...">
        <button id="goBtn">Idź</button>
    </div>
    <div id="content">
        <iframe id="browser" src="https://trangorgeos.website"></iframe>
    </div>
    <script>
        const iframe = document.getElementById('browser');
        const urlbar = document.getElementById('urlbar');
        
        function navigate() {
            let url = urlbar.value.trim();
            if (!url) return;
            if (!url.startsWith('http://') && !url.startsWith('https://')) {
                url = 'https://' + url;
            }
            iframe.src = url;
            urlbar.value = url;
        }
        
        document.getElementById('goBtn').addEventListener('click', navigate);
        urlbar.addEventListener('keypress', (e) => { 
            if (e.key === 'Enter') navigate(); 
        });
        document.getElementById('refreshBtn').addEventListener('click', () => { 
            try { iframe.contentWindow.location.reload(); } catch(e) {} 
        });
    </script>
</body>
</html>"#;

    let html_escaped = html
        .replace('%', "%25")
        .replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('"', "%22")
        .replace('\n', "%0A")
        .replace('\r', "%0D");
    let data_url = format!("data:text/html;charset=utf-8,{}", html_escaped);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                if webview_clone.lock().unwrap().is_none() {
                    println!("[Engine] Tworzenie WebView...");
                    
                    match WebViewBuilder::new(&window_clone)
                        .with_url(&data_url)
                        .with_devtools(true)
                        .build()
                    {
                        Ok(wv) => {
                            *webview_clone.lock().unwrap() = Some(wv);
                            println!("[Engine] ✓ WebView utworzony!");
                        }
                        Err(e) => {
                            eprintln!("[Engine] Błąd: {}", e);
                        }
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            _ => {}
        }
    });
}