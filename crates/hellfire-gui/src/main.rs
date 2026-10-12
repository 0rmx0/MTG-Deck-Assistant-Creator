mod deck_state;
mod import_window;
mod secret_store;
mod settings;
mod settings_window;
mod storage_handle;

use eframe::egui;
use tokio::runtime::Runtime;

use crate::deck_state::DeckState;
use crate::import_window::ImportWindow;
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
        Box::new(|_cc| {
            Ok(Box::new(MtgDeckAssistantApp::new()))
        }),
    )
}

struct MtgDeckAssistantApp {
    settings: Settings,
    settings_window: SettingsWindow,
    import_window: ImportWindow,
    deck_state: DeckState,
    status: Option<(bool, String)>,
    _runtime: Runtime,
}

impl MtgDeckAssistantApp {
    fn new() -> Self {
        let runtime = Runtime::new().expect("runtime tokio");
        let _ = runtime;
        MtgDeckAssistantApp {
            settings: Settings::load(),
            settings_window: SettingsWindow::default(),
            import_window: ImportWindow::default(),
            deck_state: DeckState::new(),
            status: None,
            _runtime: runtime,
        }
    }
}

impl eframe::App for MtgDeckAssistantApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Importer une decklist…").clicked() {
                        self.import_window.open();
                        ui.close_menu();
                    }
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
        self.import_window.show(ctx, &mut self.deck_state);

        if let Some((ok, msg)) = self.deck_state.poll_import() {
            self.import_window.busy = false;
            self.import_window.status = Some((ok, msg.clone()));
            self.status = Some((ok, msg));
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MTG Deck Assistant Creator");
            ui.small(format!(
                "Répertoire de config : {}",
                Settings::config_dir().display()
            ));
            ui.separator();
            ui.label("Assistant de construction de decks Magic: The Gathering");
            ui.label(format!("Base de données : {}", self.settings.db_engine.label()));
            ui.label(format!("Fournisseur IA : {}", self.settings.ai_provider.label()));
            if self.settings.api_key_stored {
                ui.label("Clé API : enregistrée 🔑");
            }

            if let Some((ok, msg)) = &self.status {
                ui.add_space(6.0);
                ui.colored_label(
                    if *ok {
                        egui::Color32::from_rgb(140, 200, 120)
                    } else {
                        egui::Color32::from_rgb(235, 100, 90)
                    },
                    msg,
                );
            }

            ui.add_space(8.0);
            let stats = self.deck_state.deck.stats();
            ui.label(format!(
                "Deck courant : « {} » — {} cartes, CMC moyen {:.2}",
                self.deck_state.deck.name,
                stats.total_cards,
                stats.avg_cmc
            ));
            ui.add_space(4.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.deck_state.deck.entries {
                    let side = if entry.sideboard { " [SB]" } else { "" };
                    ui.label(format!(
                        "{}x {}{}",
                        entry.quantity, entry.card.name, side
                    ));
                }
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}
