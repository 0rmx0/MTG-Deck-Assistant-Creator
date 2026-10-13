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
            .with_inner_size([1100.0, 700.0])
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
    status: (bool, String),
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
            status: (true, "Prêt à enflammer le meta.".into()),
            _runtime: runtime,
        }
    }

    /// « Lever le poing » : valider le deck courant.
    fn raise_fist(&mut self) {
        match self.deck_state.deck.validate() {
            Ok(stats) => {
                self.status = (
                    true,
                    format!(
                        "Deck validé : {} cartes, CMC moyen {:.2}. Le poing est levé.",
                        stats.total_cards, stats.avg_cmc
                    ),
                );
            }
            Err(e) => self.status = (false, e.to_string()),
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
            self.status = (ok, msg);
            ctx.request_repaint();
        }

        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_SECONDARY)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                let (color, symbol) = if !self.status.0 {
                    (Theme::MAGENTA, "!!")
                } else if self.status.1.contains("validé") {
                    (Theme::TERMINAL, "✔")
                } else {
                    (Theme::CYAN, "»")
                };
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(symbol)
                            .color(color)
                            .strong()
                            .monospace(),
                    );
                    ui.label(
                        egui::RichText::new(&self.status.1)
                            .color(Theme::TEXT_PRIMARY)
                            .monospace(),
                    );
                });
            });

        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_SECONDARY)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::HALO))
                    .inner_margin(egui::Margin::same(12.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        Theme::logotype(ui);
                        ui.add_space(4.0);
                        Theme::meta(ui, "NO GODS NO MASTERS NO CORPORATIONS");
                        ui.add_space(2.0);
                        let stats = self.deck_state.deck.stats();
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Format : {}",
                                    self.deck_state.deck.format.label()
                                ))
                                .color(Theme::CYAN),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} cartes recrutées · CMC moyen {:.2}",
                                    stats.total_cards, stats.avg_cmc
                                ))
                                .color(Theme::TERMINAL)
                                .monospace(),
                            );
                        });
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("0\nrm\nX0")
                                    .color(Theme::MAGENTA)
                                    .monospace(),
                            )
                            .extend(),
                        );
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_PRIMARY)
                    .inner_margin(egui::Margin::same(10.0)),
            )
            .show(ctx, |ui| {
                egui::SidePanel::left("squad")
                    .resizable(false)
                    .default_width(ui.available_width() * 0.40)
                    .frame(
                        egui::Frame::group(ui.style())
                            .fill(Theme::BG_SECONDARY)
                            .stroke(egui::Stroke::new(1.0_f32, Theme::HALO))
                            .rounding(egui::Rounding::same(10.0))
                            .inner_margin(egui::Margin::same(10.0)),
                    )
                    .show_inside(ui, |ui| {
                        Theme::heading(ui, "TON ESCOUADE");
                        ui.add_space(6.0);
                        Theme::cutter_separator(ui);
                        ui.add_space(6.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if self.deck_state.deck.entries.is_empty() {
                                Theme::meta(ui, "Aucune carte recrutée — importe une decklist.");
                            }
                            for entry in &self.deck_state.deck.entries {
                                let side = if entry.sideboard { " [SB]" } else { "" };
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{:>2}x ", entry.quantity))
                                            .color(Theme::TERMINAL)
                                            .monospace()
                                            .strong(),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}{}",
                                            entry.card.name, side
                                        ))
                                        .color(Theme::TEXT_PRIMARY),
                                    );
                                    if let Some(mana) = &entry.card.mana_cost {
                                        ui.label(
                                            egui::RichText::new(mana.to_notation())
                                                .color(Theme::TEXT_SECONDARY)
                                                .small()
                                                .monospace(),
                                        );
                                    }
                                });
                            }
                        });
                    });

                egui::SidePanel::right("intel")
                    .resizable(false)
                    .default_width(ui.available_width() * 0.62)
                    .frame(egui::Frame::none())
                    .show_inside(ui, |ui| {
                        egui::TopBottomPanel::top("curve")
                            .frame(
                                egui::Frame::group(ui.style())
                                    .fill(Theme::BG_SECONDARY)
                                    .stroke(egui::Stroke::new(1.0_f32, Theme::HALO))
                                    .rounding(egui::Rounding::same(10.0))
                                    .inner_margin(egui::Margin::same(10.0)),
                            )
                            .show_inside(ui, |ui| {
                                Theme::heading(ui, "COURBE DE MANA");
                                ui.add_space(6.0);
                                Theme::cutter_separator(ui);
                                ui.add_space(6.0);
                                draw_curve(ui, &self.deck_state);
                            });

                        egui::CentralPanel::default()
                            .frame(egui::Frame::none())
                            .show_inside(ui, |ui| {
                                egui::Frame::group(ui.style())
                                    .fill(Theme::BG_SECONDARY)
                                    .stroke(egui::Stroke::new(1.0_f32, Theme::HALO))
                                    .rounding(egui::Rounding::same(10.0))
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        Theme::heading(ui, "ACTIONS");
                                        ui.add_space(6.0);
                                        Theme::cutter_separator(ui);
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            if Theme::primary_button(ui, "Recruter").clicked() {
                                                self.import_window.open();
                                            }
                                            if Theme::secondary_button(ui, "Sabotage").clicked() {
                                                self.status = (
                                                    false,
                                                    "Sabotage : sélectionne une carte dans l'escouade pour la retirer.".into(),
                                                );
                                            }
                                            if Theme::primary_button(ui, "Lever le poing")
                                                .clicked()
                                            {
                                                self.raise_fist();
                                            }
                                            if Theme::secondary_button(ui, "Se replier")
                                                .clicked()
                                            {
                                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                            }
                                        });
                                        ui.add_space(6.0);
                                        Theme::meta(
                                            ui,
                                            "Recruter une carte · Sabotage (retirer) · Lever le poing (valider) · Se replier",
                                        );
                                        ui.add_space(8.0);
                                        hidden_layer(ui);
                                    });
                            });
                    });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// Courbe de mana : colonnes corail 0..7+, proportions relatives au maximum.
fn draw_curve(ui: &mut egui::Ui, deck_state: &DeckState) {
    let stats = deck_state.deck.stats();
    let max = stats.curve.max().max(1) as f32;
    let bars: Vec<(String, u32)> = (0..=7u32)
        .map(|cmc| {
            let label = if cmc == 7 { "7+".into() } else { cmc.to_string() };
            (label, stats.curve.get(cmc))
        })
        .collect();
    let height = 110.0_f32;
    let bar_width = 34.0_f32;
    let gap = 10.0_f32;
    let total_width = bars.len() as f32 * (bar_width + gap);
    let (_, plot_rect) = ui.allocate_exact_size(
        egui::vec2(total_width, height + 18.0),
        egui::Sense::hover(),
    );
    let origin = egui::pos2(plot_rect.rect.min.x, plot_rect.rect.max.y - 16.0);
    ui.painter().line_segment(
        [
            egui::pos2(origin.x, origin.y),
            egui::pos2(origin.x + total_width, origin.y),
        ],
        egui::Stroke::new(1.0_f32, Theme::HALO),
    );
    for (i, (label, value)) in bars.iter().enumerate() {
        let x = origin.x + i as f32 * (bar_width + gap);
        let filled = (*value as f32 / max) * height;
        let rect = egui::Rect::from_min_max(
            egui::pos2(x, origin.y - filled),
            egui::pos2(x + bar_width, origin.y),
        );
        if filled > 0.0 {
            ui.painter()
                .rect_filled(rect, egui::Rounding::same(2.0_f32), Theme::CYAN);
        } else {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(x, origin.y - 2.0),
                    egui::pos2(x + bar_width, origin.y),
                ),
                egui::Rounding::same(0.0_f32),
                Theme::BG_SECONDARY,
            );
        }
        ui.painter().text(
            egui::pos2(x + bar_width / 2.0, origin.y + 8.0),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::monospace(11.0),
            Theme::TEXT_SECONDARY,
        );
        if *value > 0 {
            ui.painter().text(
                egui::pos2(x + bar_width / 2.0, origin.y - filled - 8.0),
                egui::Align2::CENTER_CENTER,
                value.to_string(),
                egui::FontId::monospace(10.0),
                Theme::TERMINAL,
            );
        }
    }
}

/// Couche cachée : message base64 rendu tel quel, discret en bas de page.
fn hidden_layer(ui: &mut egui::Ui) {
    ui.add(
        egui::Label::new(
            egui::RichText::new("UkVHQVJERSBBIEwgSU5URVJJRVVSIERVIFNPTg==")
                .color(Theme::BG_SECONDARY)
                .monospace()
                .small(),
        )
        .extend(),
    );
}
