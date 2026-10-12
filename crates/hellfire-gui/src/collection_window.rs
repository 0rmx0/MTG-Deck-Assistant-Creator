use eframe::egui;
use hellfire_core::Card;

use crate::deck_state::DeckState;

/// Fenêtre de gestion de la collection : ajout manuel (via Scryfall) et import de decklist.
pub struct CollectionWindow {
    pub open: bool,
    card_name: String,
    quantity: u32,
    busy: bool,
    status: Option<(bool, String)>,
    filter: String,
}

impl Default for CollectionWindow {
    fn default() -> Self {
        CollectionWindow {
            open: false,
            card_name: String::new(),
            quantity: 1,
            busy: false,
            filter: String::new(),
            status: None,
        }
    }
}

impl CollectionWindow {
    pub fn open(&mut self) {
        self.status = None;
        self.card_name.clear();
        self.quantity = 1;
        self.open = true;
    }

    /// Réponse asynchrone d'un ajout de carte.
    pub fn on_card_resolved(&mut self, result: Result<(Card, u32), String>) {
        self.busy = false;
        match result {
            Ok((card, qty)) => {
                self.status = Some((
                    true,
                    format!("{qty}x {} ajoutée à la collection.", card.name),
                ));
            }
            Err(e) => self.status = Some((false, e)),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, deck_state: &mut DeckState) {
        let mut keep_open = self.open;
        egui::Window::new("Ma collection")
            .open(&mut keep_open)
            .resizable(false)
            .default_width(560.0)
            .show(ctx, |ui| {
                self.contents(ui, deck_state);
            });
        self.open = keep_open;
    }

    fn contents(&mut self, ui: &mut egui::Ui, deck_state: &mut DeckState) {
        crate::theme::Theme::heading(ui, "AJOUTER UNE CARTE");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Nom :");
            let edit = egui::TextEdit::singleline(&mut self.card_name)
                .hint_text("ex. Lightning Bolt")
                .desired_width(240.0);
            ui.add(edit);
            ui.label("Quantité :");
            ui.add(egui::DragValue::new(&mut self.quantity).range(1..=999));
            if ui
                .add_enabled(
                    !self.busy && !self.card_name.trim().is_empty(),
                    egui::Button::new("Ajouter à la collection"),
                )
                .clicked()
            {
                self.start_add(deck_state);
            }
        });
        ui.add_space(4.0);
        crate::theme::Theme::meta(
            ui,
            "La carte est résolue via Scryfall puis enregistrée dans la base.",
        );
        if self.busy {
            ui.add_space(4.0);
            ui.spinner();
            ui.label("Recherche sur Scryfall…");
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
        ui.add_space(6.0);
        crate::theme::Theme::cutter_separator(ui);
        ui.add_space(6.0);
        crate::theme::Theme::heading(ui, "COLLECTION ENREGISTRÉE");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("Vider la collection").clicked() {
                let _ = deck_state.storage.clear_collection();
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Filtrer :");
            ui.add(
                egui::TextEdit::singleline(&mut self.filter)
                    .hint_text("nom de carte…")
                    .desired_width(200.0),
            );
        });
        ui.add_space(4.0);
        let collection = deck_state.storage.load_collection().unwrap_or_default();
        let filter = self.filter.to_lowercase();
        let filtered: Vec<&(Card, u32)> = collection
            .iter()
            .filter(|(c, _)| c.name.to_lowercase().contains(&filter))
            .collect();
        egui::ScrollArea::vertical()
            .max_height(260.0)
            .show(ui, |ui| {
                if filtered.is_empty() {
                    crate::theme::Theme::meta(
                        ui,
                        "Aucune carte — ajoute-en ou importe une decklist.",
                    );
                }
                for (card, qty) in filtered {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("{qty:>3}x "))
                                .strong()
                                .monospace(),
                        );
                        ui.label(&card.name);
                        ui.label(egui::RichText::new(card.type_line.clone()).small().weak());
                        let id = card.scryfall_id.clone();
                        if ui.small_button("−").clicked() {
                            if *qty > 1 {
                                let _ = deck_state.storage.set_quantity(&id, qty - 1);
                            } else {
                                let _ = deck_state.storage.remove_card(&id);
                            }
                        }
                        if ui.small_button("+").clicked() {
                            let _ = deck_state.storage.set_quantity(&id, qty + 1);
                        }
                        if ui.small_button("Supprimer").clicked() {
                            let _ = deck_state.storage.remove_card(&id);
                        }
                    });
                }
            });
    }

    fn start_add(&mut self, deck_state: &mut DeckState) {
        let name = self.card_name.trim().to_string();
        let qty = self.quantity;
        self.busy = true;
        self.status = None;
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<(Card, u32), String>>(1);
        deck_state.collection_rx = Some(rx);
        let ctx = deck_state.rt.clone();
        let client = hellfire_api::ScryfallClient::new();
        deck_state.rt.spawn(async move {
            let result = match client.search_card(&name).await {
                Ok(card) => Ok((card, qty)),
                Err(e) => Err(e.to_string()),
            };
            let _ = tx.send(result).await;
            let _ = ctx;
        });
    }
}
