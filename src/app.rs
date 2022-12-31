/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct InsightApp {
    // state of windows
    about_visible: bool,
}

impl Default for InsightApp {
    fn default() -> Self {
        Self {
            about_visible: false,
        }
    }
}

impl InsightApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // `persistence` feature must be enabled for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for InsightApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { about_visible: _ } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about_visible = true;
                    }
                });
            });
        });

        egui::SidePanel::left("laps_panel").show(ctx, |ui| {
            ui.heading("Laps");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lap");
        });

        if self.about_visible {
            egui::Window::new("About").show(ctx, |ui| {
                ui.heading("Openlaps Insight");
                ui.label("Copyright 2022 Kris Foster <kris.foster@gmail.com>");
                if ui.button("Okay").clicked() {
                    self.about_visible = false;
                }
            });
        }
    }
}
