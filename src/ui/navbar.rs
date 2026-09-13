pub struct NavBar {
    pub url_input: String,
}

impl NavBar {
    pub fn new(default_url: &str) -> Self {
        Self {
            url_input: default_url.to_string(),
        }
    }

    /// Rysuje pasek adresu na górze ekranu za pomocą egui
    pub fn render(&mut self, ctx: &egui::Context) -> Option<String> {
        let mut action = None;

        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("◄").clicked() {
                    println!("[UI] Wstecz");
                }
                if ui.button("►").clicked() {
                    println!("[UI] Dalej");
                }
                if ui.button("🔄").clicked() {
                    println!("[UI] Odśwież");
                }

                // Pasek wpisywania adresu URL
                let response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut self.url_input)
                        .hint_text("Wpisz URL lub szukaj..."),
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    action = Some(self.url_input.clone());
                }
            });
        });

        action
    }
}