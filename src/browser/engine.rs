use servo::ServoUrl;

pub struct BrowserEngine {
    current_url: String,
    pub servo: Option<servo::Servo>,
}

impl BrowserEngine {
    pub fn new(initial_url: &str) -> Self {
        Self {
            current_url: initial_url.to_string(),
            servo: None,
        }
    }

    pub fn init(&mut self) {
        let builder = servo::ServoBuilder::default();
        let servo_inst = builder.build();

        if let Ok(url) = ServoUrl::parse(&self.current_url) {
            println!("[Engine] Załadowano URL początkowy: {}", url);
        }

        self.servo = Some(servo_inst);
    }

    pub fn navigate(&mut self, url: &str) {
        let formatted_url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        println!("[Engine] Nawigacja do: {}", formatted_url);
        self.current_url = formatted_url;

        if let Some(_servo) = &mut self.servo {
            if let Ok(parsed_url) = ServoUrl::parse(&self.current_url) {
                println!("[Engine] Przekazano URL: {}", parsed_url);
            }
        }
    }

    pub fn current_url(&self) -> &str {
        &self.current_url
    }
}