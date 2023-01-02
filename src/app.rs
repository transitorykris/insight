use std::path::Path;

use logger::Logger;

const LOG_FILE: &str = "/tmp/openlaps_dashboard_testing.db";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct InsightApp {
    // state of windows
    about_visible: bool,

    #[serde(skip)]
    logger: Logger,

    #[serde(skip)]
    sessions: Vec<u64>,
}

impl Default for InsightApp {
    fn default() -> Self {
        Self {
            about_visible: false,
            logger: Logger::new(Path::new(LOG_FILE)),
            sessions: Vec::new(),
        }
    }
}

impl InsightApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // `persistence` feature must be enabled for this to work.
        // Not worried about persistence right now
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let mut app = InsightApp::default();
        // TODO handle errors
        app.sessions = app.logger.get_sessions().unwrap();
        app
    }
}

impl eframe::App for InsightApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            about_visible: _,
            logger,
            sessions,
        } = self;

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

        egui::SidePanel::left("sessions_panel").show(ctx, |ui| {
            ui.heading("Sessions");
            for session in sessions {
                ui.label(format!("{}", session));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lap");
            ui.label(format!("{:?}", logger.get_last()));

            use egui::plot::{Line, Plot, PlotPoints};
            let data = [
                [1.0, 1.0],
                [1.1, 2.0],
                [1.2, 3.0],
                [1.3, 3.0],
                [2.0, 3.0],
                [2.0, 3.0],
                [2.0, 2.0],
                [3.1, 2.0],
                [2.0, 1.0],
                [1.0, 1.0],
            ];
            let sin: PlotPoints = (0..10).map(|i| data[i]).collect();
            let line = Line::new(sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });

        if self.about_visible {
            egui::Window::new("About")
                .resizable(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.heading("Openlaps Insight");
                    ui.label("Copyright 2022 Kris Foster <kris.foster@gmail.com>");
                    if ui.button("Okay").clicked() {
                        self.about_visible = false;
                    }
                });
        }
    }
}
