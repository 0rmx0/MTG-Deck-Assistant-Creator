mod deck_state;
mod import_window;
mod secret_store;
mod settings;
mod settings_window;
mod storage_handle;
mod theme;

use eframe::egui;
use tokio::runtime::Runtime;

use crate::deck_state::DeckState;
use crate::import_window::ImportWindow;
use crate::settings::Settings;
use crate::settings_window::SettingsWindow;
use crate::theme::Theme;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("HELLFIRE DECKBUILDER"),
        ..Default::default()
    };
    eframe::run_native(
        "HELLFIRE DECKBUILDER",
        options,
        Box::new(|cc| {
            Theme::install(&cc.egui_ctx);
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
                    if ui.button("Se replier").clicked() {
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
            // Bannière : logotype gravé + devise, masque cousu en ASCII.
            egui::Frame::group(ui.style())
                .fill(Theme::BG_SECONDARY)
                .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
                .rounding(egui::Rounding::same(2.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        Theme::logotype(ui);
                        ui.add_space(4.0);
                        Theme::meta(ui, "MASQUE COUSU · LE DECK EST UNE ARME");
                        ui.add_space(2.0);
                        Theme::meta(ui, "Construction de deck comme acte d'insurrection");
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("X--X\n | |\n~~~~~")
                                    .color(Theme::CORAL)
                                    .monospace(),
                            )
                            .extend(),
                        );
                    });
                });

            ui.add_space(12.0);
            Theme::cutter_separator(ui);
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                Theme::heading(ui, "État de l'arsenal");
            });
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("Base de données :");
                Theme::stat(ui, self.settings.db_engine.label());
                ui.add_space(12.0);
                Theme::format_badge(ui, self.settings.db_engine.label());
            });
            ui.horizontal(|ui| {
                ui.label("Fournisseur IA :");
                Theme::stat(ui, self.settings.ai_provider.label());
                ui.add_space(12.0);
                Theme::meta(ui, self.settings.ai_model.clone());
            });            if self.settings.api_key_stored {
                Theme::meta(ui, "Clé API : enregistrée dans le coffre 🔒");
            } else {
                Theme::meta(ui, "Clé API : aucune — passe par Paramètres pour en ranger une.");
            }

            ui.add_space(8.0);
            Theme::meta(
                ui,
                format!("Coffre de config : {}", Settings::config_dir().display()),
            );
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                Theme::meta(ui, "VOX POPULI — tutoiement de rigueur, le meta sera enflammé.");
            if let Some((ok, msg)) = &self.status {
                ui.add_space(6.0);
                Theme::meta(
                    ui,
                    (if *ok { "✔ " } else { "✘ " }).to_string() + msg,
                );
            }

            ui.add_space(8.0);
            let stats = self.deck_state.deck.stats();
            Theme::meta(
                ui,
                format!(
                    "Deck courant : « {} » — {} cartes, CMC moyen {:.2}",
                    self.deck_state.deck.name, stats.total_cards, stats.avg_cmc
                ),
            );
            ui.add_space(4.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.deck_state.deck.entries {
                    let side = if entry.sideboard { " [SB]" } else { "" };
                    Theme::meta(
                        ui,
                        format!("{}x {}{}", entry.quantity, entry.card.name, side),
                    );
                }
            });

            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}
