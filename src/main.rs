mod secret_store;
mod settings;
mod settings_window;

use eframe::egui;

use crate::settings::Settings;
use crate::settings_window::SettingsWindow;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("MTG Deck Assistant Creator"),
        ..Default::default()
    };
    eframe::run_native(
        "MTG Deck Assistant Creator",
        options,
        Box::new(|_cc| Ok(Box::new(MtgDeckAssistantApp::new()))),
    )
}

struct MtgDeckAssistantApp {
    settings: Settings,
    settings_window: SettingsWindow,
}

impl MtgDeckAssistantApp {
    fn new() -> Self {
        MtgDeckAssistantApp {
            settings: Settings::load(),
            settings_window: SettingsWindow::default(),
        }
    }
}

impl eframe::App for MtgDeckAssistantApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Paramètres…").clicked() {
                        self.settings_window.open();
                        ui.close_menu();
                    }
                    if ui.button("Quitter").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Aide", |ui| {
                    if ui.button("À propos…").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        self.settings_window.show(ctx, &mut self.settings);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MTG Deck Assistant Creator");
            ui.small(format!("Répertoire de config : {}", Settings::config_dir().display()));
            ui.separator();
            ui.label("Assistant de construction de decks Magic: The Gathering");
            ui.label(format!("Base de données : {}", self.settings.db_engine.label()));
            ui.label(format!("Fournisseur IA : {}", self.settings.ai_provider.label()));
            if self.settings.api_key_stored {
                ui.label("Clé API : enregistrée 🔑");
            }
        });
    }
}
