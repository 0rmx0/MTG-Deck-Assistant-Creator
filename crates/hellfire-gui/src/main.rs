mod ai_builder;
mod collection_window;
mod deck_state;
mod import_window;
mod secret_store;
mod settings;
mod settings_window;
mod storage_handle;
mod theme;

use eframe::egui;
use tokio::runtime::Runtime;

use crate::ai_builder::AiBuilderWindow;
use crate::collection_window::CollectionWindow;
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
    collection_window: CollectionWindow,
    ai_builder: AiBuilderWindow,
    deck_state: DeckState,
    status: (bool, String),
    active_tab: Tab,
    _runtime: Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Deck,
    Collection,
    Decks,
}

impl MtgDeckAssistantApp {
    fn new() -> Self {
        let runtime = Runtime::new().expect("runtime tokio");
        let rt = runtime.handle().clone();
        let settings = Settings::load();
        let deck_state = DeckState::new(rt, &settings);
        MtgDeckAssistantApp {
            settings,
            settings_window: SettingsWindow::default(),
            import_window: ImportWindow::default(),
            collection_window: CollectionWindow::default(),
            ai_builder: AiBuilderWindow::default(),
            deck_state,
            active_tab: Tab::Deck,
            status: (true, "Prêt à enflammer le meta.".into()),
            _runtime: runtime,
        }
    }

    /// Valider le deck courant.
    fn raise_fist(&mut self) {
        match self.deck_state.deck.validate() {
            Ok(stats) => {
                self.status = (
                    true,
                    format!(
                        "Deck validé : {} cartes, CMC moyen {:.2}.",
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
                    if ui.button("Ma collection…").clicked() {
                        self.collection_window.open();
                        ui.close_menu();
                    }
                    if ui.button("Créer un deck avec l'IA…").clicked() {
                        self.ai_builder.open(self.deck_state.deck.format.label());
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
            self.status = (ok, msg);
            ctx.request_repaint();
        }
        if let Some(result) = self.deck_state.poll_collection() {
            self.collection_window.on_card_resolved(result);
            ctx.request_repaint();
        }
        if let Some(result) = self.deck_state.poll_ai() {
            self.ai_builder.on_proposal(result);
            ctx.request_repaint();
        }
        if let Some(result) = self.deck_state.poll_deck_from_ai() {
            match result {
                Ok(deck) => {
                    self.status = (
                        true,
                        format!(
                            "Deck « {} » généré et enregistré ({} cartes).",
                            deck.name,
                            deck.stats().total_cards
                        ),
                    );
                    self.ai_builder.open = false;
                }
                Err(e) => {
                    self.ai_builder.status = Some((false, e.clone()));
                    self.status = (false, e);
                }
            }
            ctx.request_repaint();
        }
        self.collection_window.show(ctx, &mut self.deck_state);
        self.ai_builder
            .show(ctx, &mut self.deck_state, &self.settings);

        let storage_warning = self.deck_state.storage.warning();
        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_SECONDARY)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                let (color, symbol) = if !self.status.0 {
                    (Theme::CORAL, "!!")
                } else if self.status.1.contains("validé") {
                    (Theme::EMBER, "✔")
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
                    if let Some(w) = &storage_warning {
                        ui.add_space(12.0);
                        ui.label(
                            egui::RichText::new(format!("⚠ {w}"))
                                .color(Theme::CORAL)
                                .monospace(),
                        );
                    }
                });
            });

        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_SECONDARY)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
                    .inner_margin(egui::Margin::same(12.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        Theme::logotype(ui);
                        ui.add_space(4.0);
                        Theme::meta(ui, "MASQUE COUSU · LE DECK EST UNE ARME");
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
                                .color(Theme::EMBER)
                                .monospace(),
                            );
                        });
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
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Theme::BG_PRIMARY)
                    .inner_margin(egui::Margin::same(10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let tabs = [
                        (Tab::Deck, "Mon deck"),
                        (Tab::Collection, "Ma collection"),
                        (Tab::Decks, "Mes decks"),
                    ];
                    for (tab, label) in tabs {
                        if ui.selectable_label(self.active_tab == tab, label).clicked() {
                            self.active_tab = tab;
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Theme::primary_button(ui, "Créer avec l'IA").clicked() {
                            self.ai_builder.open(self.deck_state.deck.format.label());
                        }
                    });
                });
                ui.add_space(6.0);
                match self.active_tab {
                    Tab::Deck => self.deck_tab(ui, ctx),
                    Tab::Collection => self.collection_tab(ui),
                    Tab::Decks => self.decks_tab(ui),
                }
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}

impl MtgDeckAssistantApp {
    fn deck_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        {
            egui::SidePanel::left("squad")
                .resizable(false)
                .default_width(ui.available_width() * 0.40)
                .frame(
                    egui::Frame::group(ui.style())
                        .fill(Theme::BG_SECONDARY)
                        .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
                        .rounding(egui::Rounding::same(2.0))
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
                                        .color(Theme::EMBER)
                                        .monospace()
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{}{}", entry.card.name, side))
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
                                    .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
                                    .rounding(egui::Rounding::same(2.0))
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
                                    .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
                                    .rounding(egui::Rounding::same(2.0))
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        Theme::heading(ui, "ACTIONS");
                                        ui.add_space(6.0);
                                        Theme::cutter_separator(ui);
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            if Theme::primary_button(ui, "Ajouter une carte").clicked() {
                                                self.import_window.open();
                                            }
                                            if Theme::secondary_button(ui, "Retirer").clicked() {
                                                self.status = (
                                                    false,
                                                    "Retirer : sélectionne une carte dans le deck pour la retirer.".into(),
                                                );
                                            }
                                            if Theme::primary_button(ui, "Valider le deck").clicked()
                                            {
                                                self.raise_fist();
                                            }
                                            if Theme::secondary_button(ui, "Deck vers collection").clicked()
                                            {
                                                let mut count = 0u32;
                                                for entry in &self.deck_state.deck.entries {
                                                    if self
                                                        .deck_state
                                                        .storage
                                                        .upsert_card(&entry.card, entry.quantity)
                                                        .is_ok()
                                                    {
                                                        count += entry.quantity;
                                                    }
                                                }
                                                self.status = (
                                                    true,
                                                    format!("{count} cartes ajoutées à la collection."),
                                                );
                                            }
                                            if Theme::secondary_button(ui, "Quitter").clicked()
                                            {
                                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                            }
                                        });
                                        ui.add_space(6.0);
                                        Theme::meta(
                                            ui,
                                            "Ajouter une carte · Retirer · Valider le deck · Quitter",
                                        );
                                    });
                            });
                    });
        }
    }

    fn collection_tab(&mut self, ui: &mut egui::Ui) {
        let collection = self
            .deck_state
            .storage
            .load_collection()
            .unwrap_or_default();
        let total: u32 = collection.iter().map(|(_, q)| q).sum();
        egui::Frame::group(ui.style())
            .fill(Theme::BG_SECONDARY)
            .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
            .rounding(egui::Rounding::same(2.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                Theme::heading(ui, "MA COLLECTION");
                ui.add_space(6.0);
                Theme::cutter_separator(ui);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    Theme::meta(
                        ui,
                        &format!("{} cartes distinctes · {} cartes au total", collection.len(), total),
                    );
                    if ui.button("Gérer la collection…").clicked() {
                        self.collection_window.open();
                    }
                });
                ui.add_space(6.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if collection.is_empty() {
                        Theme::meta(ui, "Aucune carte — ouvre « Gérer la collection » ou importe une decklist dans ta collection.");
                    }
                    for (card, qty) in &collection {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("{qty:>3}x "))
                                    .color(Theme::EMBER)
                                    .monospace()
                                    .strong(),
                            );
                            ui.label(&card.name);
                            ui.label(
                                egui::RichText::new(card.type_line.clone())
                                    .color(Theme::TEXT_SECONDARY)
                                    .small(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("Ajouter au deck").clicked() {
                                        let _ = self.deck_state.deck.recruit(card.clone(), 1, false);
                                        self.status = (
                                            true,
                                            format!("{} ajoutée au deck.", card.name),
                                        );
                                    }
                                },
                            );
                        });
                    }
                });
            });
    }

    fn decks_tab(&mut self, ui: &mut egui::Ui) {
        let decks = self.deck_state.storage.list_decks().unwrap_or_default();
        egui::Frame::group(ui.style())
            .fill(Theme::BG_SECONDARY)
            .stroke(egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED))
            .rounding(egui::Rounding::same(2.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                Theme::heading(ui, "MES DECKS");
                ui.add_space(6.0);
                Theme::cutter_separator(ui);
                ui.add_space(6.0);
                Theme::meta(ui, &format!("{} deck(s) enregistré(s)", decks.len()));
                ui.add_space(6.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if decks.is_empty() {
                        Theme::meta(
                            ui,
                            "Aucun deck — importe une decklist ou génère-en un avec l'IA.",
                        );
                    }
                    for (id, name, format) in decks {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("#{id:<4}"))
                                    .color(Theme::CYAN)
                                    .monospace(),
                            );
                            ui.label(&name);
                            ui.label(
                                egui::RichText::new(format)
                                    .color(Theme::TEXT_SECONDARY)
                                    .small(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("Ouvrir").clicked() {
                                        match self.deck_state.storage.load_deck(id) {
                                            Ok(Some(deck)) => {
                                                self.deck_state.deck = deck;
                                                self.active_tab = Tab::Deck;
                                                self.status =
                                                    (true, format!("Deck « {name} » chargé."));
                                            }
                                            Ok(None) => {
                                                self.status = (false, "Deck introuvable.".into());
                                            }
                                            Err(e) => self.status = (false, e.to_string()),
                                        }
                                    }
                                    if ui.button("Supprimer").clicked() {
                                        if let Err(e) = self.deck_state.storage.delete_deck(id) {
                                            self.status = (false, e.to_string());
                                        } else {
                                            self.status =
                                                (true, format!("Deck « {name} » supprimé."));
                                        }
                                    }
                                },
                            );
                        });
                    }
                });
            });
    }
}

/// Courbe de mana : colonnes corail 0..7+, proportions relatives au maximum.
fn draw_curve(ui: &mut egui::Ui, deck_state: &DeckState) {
    let stats = deck_state.deck.stats();
    let max = stats.curve.max().max(1) as f32;
    let bars: Vec<(String, u32)> = (0..=7u32)
        .map(|cmc| {
            let label = if cmc == 7 {
                "7+".into()
            } else {
                cmc.to_string()
            };
            (label, stats.curve.get(cmc))
        })
        .collect();
    let height = 110.0_f32;
    let bar_width = 34.0_f32;
    let gap = 10.0_f32;
    let total_width = bars.len() as f32 * (bar_width + gap);
    let (_, plot_rect) =
        ui.allocate_exact_size(egui::vec2(total_width, height + 18.0), egui::Sense::hover());
    let origin = egui::pos2(plot_rect.rect.min.x, plot_rect.rect.max.y - 16.0);
    ui.painter().line_segment(
        [
            egui::pos2(origin.x, origin.y),
            egui::pos2(origin.x + total_width, origin.y),
        ],
        egui::Stroke::new(1.0_f32, Theme::BG_ELEVATED),
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
                .rect_filled(rect, egui::Rounding::same(0.0_f32), Theme::CORAL);
        } else {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(x, origin.y - 2.0),
                    egui::pos2(x + bar_width, origin.y),
                ),
                egui::Rounding::same(0.0_f32),
                Theme::BG_ELEVATED,
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
                Theme::EMBER,
            );
        }
    }
}
