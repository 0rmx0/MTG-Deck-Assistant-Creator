use eframe::egui;
use hellfire_core::decklist;

use crate::deck_state::DeckState;

pub struct ImportWindow {
    pub open: bool,
    text: String,
    deck_name: String,
    pub busy: bool,
    pub status: Option<(bool, String)>,
}

impl Default for ImportWindow {
    fn default() -> Self {
        ImportWindow {
            open: false,
            text: String::new(),
            deck_name: String::new(),
            busy: false,
            status: None,
        }
    }
}

impl ImportWindow {
    pub fn open(&mut self) {
        self.status = None;
        self.text.clear();
        self.deck_name.clear();
        self.open = true;
    }

    pub fn show(&mut self, ctx: &egui::Context, deck_state: &mut DeckState) {
        let mut keep_open = self.open;
        egui::Window::new("Importer une decklist")
            .open(&mut keep_open)
            .resizable(false)
            .default_width(520.0)
            .show(ctx, |ui| {
                self.contents(ctx, ui, deck_state);
            });
        self.open = keep_open;
    }

    fn contents(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, deck_state: &mut DeckState) {
        ui.label("Collez votre liste : « 1 Cavalerie ninja mécanisée (TMT) 156 ». Seuls la quantité et le nom sont obligatoires.");
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Nom du deck :");
            ui.text_edit_singleline(&mut self.deck_name);
        });
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .max_height(260.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .font(egui::TextStyle::Monospace),
                );
            });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.busy, egui::Button::new("Importer"))
                .clicked()
            {
                self.start_import(ctx, deck_state);
            }
            if ui.button("Effacer").clicked() {
                self.text.clear();
                self.status = None;
            }
        });

        if self.busy {
            ui.add_space(4.0);
            ui.spinner();
            ui.label("Résolution des cartes via Scryfall…");
        }

        if let Some((ok, msg)) = &self.status {
            ui.add_space(4.0);
            ui.colored_label(
                if *ok {
                    egui::Color32::from_rgb(140, 200, 120)
                } else {
                    egui::Color32::from_rgb(235, 100, 90)
                },
                msg,
            );
        }
    }

    fn start_import(&mut self, ctx: &egui::Context, deck_state: &mut DeckState) {
        let entries = match decklist::parse_decklist(&self.text) {
            Ok(e) if e.is_empty() => {
                self.status = Some((false, "Liste vide.".into()));
                return;
            }
            Ok(e) => e,
            Err(err) => {
                self.status = Some((false, err.to_string()));
                return;
            }
        };

        self.busy = true;
        self.status = None;
        let client = hellfire_api::ScryfallClient::new();
        let name = if self.deck_name.trim().is_empty() {
            "Deck importé".to_string()
        } else {
            self.deck_name.trim().to_string()
        };
        let format = deck_state.deck.format;
        let (tx, rx) = tokio::sync::mpsc::channel::<(bool, String, Option<hellfire_core::Deck>)>(1);
        deck_state.import_rx = Some(rx);
        let ctx = ctx.clone();

        deck_state.rt.spawn(async move {
            let result = client.import_decklist(&entries).await;
            let (ok, msg, resolved_deck) = match result {
                Ok(import) => {
                    let mut deck = hellfire_core::Deck::new(name, format);
                    let mut errors = Vec::new();
                    for (entry, card) in &import.resolved {
                        if let Err(e) = deck.recruit(card.clone(), entry.quantity, entry.sideboard)
                        {
                            errors.push(format!("{} ({e})", entry.name));
                        }
                    }
                    let mut msg = format!(
                        "{} cartes importées sur {} lignes.",
                        import.total_cards(),
                        entries.len()
                    );
                    if !import.failed.is_empty() {
                        msg.push_str(&format!(
                            " {} carte(s) introuvable(s) : {}",
                            import.failed.len(),
                            import
                                .failed
                                .iter()
                                .map(|(e, _)| e.name.clone())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                    }
                    if !errors.is_empty() {
                        msg.push_str(&format!(" Refus : {}", errors.join(", ")));
                    }
                    (errors.is_empty(), msg, Some(deck))
                }
                Err(e) => (false, e.to_string(), None),
            };
            let _ = tx.send((ok, msg, resolved_deck)).await;
            ctx.request_repaint();
        });
    }
}
