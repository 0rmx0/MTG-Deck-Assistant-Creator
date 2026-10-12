use eframe::egui;
use hellfire_api::{AiDeckCard, AiDeckProposal, AiProviderInfo, DeckCriteria};

use crate::deck_state::DeckState;
use crate::settings::{AiProvider, Settings};

/// Questions guidées posées une à une avant la génération.
const QUESTIONS: &[(&str, &[&str])] = &[
    (
        "Quel style de jeu recherches-tu ?",
        &["Aggro", "Midrange", "Contrôle", "Combo", "Peu importe"],
    ),
    (
        "Quelles couleurs privilégier ?",
        &[
            "Blanc (W)",
            "Bleu (U)",
            "Noir (B)",
            "Rouge (R)",
            "Vert (V)",
            "Libre",
        ],
    ),
    (
        "Quel budget maximal (USD) ?",
        &["50", "100", "200", "Illimité"],
    ),
    (
        "Priorité : puissance ou fun ?",
        &["Puissance maximale", "Équilibré", "Thème / fun"],
    ),
];

fn provider_base_url(provider: AiProvider) -> &'static str {
    match provider {
        AiProvider::Mistral => "https://api.mistral.com/v1",
        AiProvider::OpenAi => "https://api.openai.com/v1",
        AiProvider::Anthropic => "https://api.anthropic.com/v1",
        AiProvider::Google => "https://generativelanguage.googleapis.com/v1beta/openai",
    }
}

/// Fenêtre de création de deck assistée par IA à partir de la collection.
pub struct AiBuilderWindow {
    pub open: bool,
    deck_name: String,
    format_label: String,
    step: usize,
    answers: Vec<String>,
    notes: String,
    busy: bool,
    pub status: Option<(bool, String)>,
    proposal: Option<AiDeckProposal>,
}

impl Default for AiBuilderWindow {
    fn default() -> Self {
        AiBuilderWindow {
            open: false,
            deck_name: String::new(),
            format_label: String::new(),
            step: 0,
            answers: Vec::new(),
            notes: String::new(),
            busy: false,
            status: None,
            proposal: None,
        }
    }
}

impl AiBuilderWindow {
    pub fn open(&mut self, format_label: &str) {
        self.status = None;
        self.proposal = None;
        self.step = 0;
        self.answers.clear();
        self.notes.clear();
        self.deck_name.clear();
        self.format_label = format_label.to_string();
        self.open = true;
    }

    pub fn on_proposal(&mut self, result: Result<AiDeckProposal, String>) {
        self.busy = false;
        match result {
            Ok(p) => {
                self.status = Some((true, "Deck proposé — vérifie puis accepte.".into()));
                self.proposal = Some(p);
            }
            Err(e) => self.status = Some((false, e)),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, deck_state: &mut DeckState, settings: &Settings) {
        let mut keep_open = self.open;
        egui::Window::new("Créer un deck avec l'IA")
            .open(&mut keep_open)
            .resizable(false)
            .default_width(560.0)
            .show(ctx, |ui| {
                self.contents(ui, deck_state, settings);
            });
        self.open = keep_open;
    }

    fn contents(&mut self, ui: &mut egui::Ui, deck_state: &mut DeckState, settings: &Settings) {
        crate::theme::Theme::heading(ui, "CRITÈRES");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Nom du deck :");
            ui.add(
                egui::TextEdit::singleline(&mut self.deck_name)
                    .hint_text("ex. Rogue BR")
                    .desired_width(220.0),
            );
        });
        ui.label(format!("Format : {}", self.format_label));
        ui.add_space(6.0);

        crate::theme::Theme::cutter_separator(ui);
        ui.add_space(6.0);
        crate::theme::Theme::heading(ui, "QUESTIONS");
        ui.add_space(4.0);

        let (question, options) = QUESTIONS[self.step.min(QUESTIONS.len() - 1)];
        ui.label(format!(
            "{} ({}/{})",
            question,
            self.step + 1,
            QUESTIONS.len()
        ));
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            for opt in options {
                let picked = self
                    .answers
                    .get(self.step)
                    .map(|a| a == *opt)
                    .unwrap_or(false);
                if ui.selectable_label(picked, *opt).clicked() {
                    if self.answers.len() > self.step {
                        self.answers[self.step] = (*opt).to_string();
                    } else {
                        self.answers.push((*opt).to_string());
                    }
                }
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.step > 0, egui::Button::new("Précédent"))
                .clicked()
            {
                self.step -= 1;
            }
            let last = self.step >= QUESTIONS.len() - 1;
            let label = if last {
                "Terminer les questions"
            } else {
                "Suivant"
            };
            if ui.button(label).clicked() && self.step < QUESTIONS.len() - 1 {
                self.step += 1;
            }
        });
        ui.add_space(4.0);
        ui.label("Notes libres (optionnel) :");
        ui.add(
            egui::TextEdit::multiline(&mut self.notes)
                .hint_text("ex. j'aime les dragons, éviter les contresorts…")
                .desired_width(f32::INFINITY)
                .desired_rows(2),
        );
        ui.add_space(6.0);

        crate::theme::Theme::cutter_separator(ui);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.busy, egui::Button::new("Générer le deck"))
                .clicked()
            {
                self.start_generation(deck_state, settings);
            }
            if self.proposal.is_some() && ui.button("Accepter et enregistrer").clicked() {
                self.accept(deck_state);
            }
        });
        if self.busy {
            ui.add_space(4.0);
            ui.spinner();
            ui.label("L'IA compose avec ta collection…");
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
        if let Some(p) = &self.proposal {
            ui.add_space(6.0);
            crate::theme::Theme::cutter_separator(ui);
            ui.add_space(6.0);
            crate::theme::Theme::heading(ui, "DECK PROPOSÉ");
            ui.add_space(4.0);
            ui.label(format!("« {} »", p.deck_name));
            ui.label(egui::RichText::new(&p.rationale).small().weak());
            ui.add_space(4.0);
            egui::ScrollArea::vertical()
                .max_height(220.0)
                .show(ui, |ui| {
                    for AiDeckCard {
                        name,
                        quantity,
                        sideboard,
                    } in &p.cards
                    {
                        let side = if *sideboard { " [SB]" } else { "" };
                        ui.label(format!("{quantity:>2}x {name}{side}"));
                    }
                });
        }
    }

    fn criteria(&self) -> DeckCriteria {
        let get = |i: usize| self.answers.get(i).cloned();
        let style = get(0).filter(|s| s != "Peu importe");
        let colors = get(1).filter(|s| s != "Libre");
        let budget = get(2).and_then(|s| s.parse::<f64>().ok());
        let priority = get(3);
        let mut notes = String::new();
        if let Some(p) = priority {
            notes.push_str(&format!("Priorité : {p}. "));
        }
        if !self.notes.trim().is_empty() {
            notes.push_str(self.notes.trim());
        }
        DeckCriteria {
            format: self.format_label.clone(),
            style,
            colors,
            commander: None,
            budget_usd: budget,
            notes: if notes.is_empty() { None } else { Some(notes) },
        }
    }

    fn start_generation(&mut self, deck_state: &mut DeckState, settings: &Settings) {
        let key = crate::secret_store::load_secret(settings.ai_provider);
        let Some(api_key) = key else {
            self.status = Some((
                false,
                "Aucune clé API enregistrée — configure-la dans Paramètres.".into(),
            ));
            return;
        };
        let collection = deck_state.storage.load_collection().unwrap_or_default();
        if collection.is_empty() {
            self.status = Some((false, "Collection vide — ajoute d'abord des cartes.".into()));
            return;
        }
        self.busy = true;
        self.status = None;
        self.proposal = None;
        let criteria = self.criteria();
        let format = deck_state.deck.format;
        let coll: Vec<(String, u32, String, u32, bool)> = collection
            .iter()
            .map(|(c, q)| {
                (
                    c.name.clone(),
                    *q,
                    c.type_line.clone(),
                    c.cmc(),
                    c.is_legal_in(format),
                )
            })
            .collect();
        let assistant = hellfire_api::AiDeckAssistant::new(AiProviderInfo {
            name: settings.ai_provider.label().to_string(),
            model: settings.ai_model.clone(),
            api_key,
            base_url: provider_base_url(settings.ai_provider).to_string(),
        });
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<AiDeckProposal, String>>(1);
        deck_state.ai_rx = Some(rx);
        deck_state.rt.spawn(async move {
            let result = assistant.build_deck(&criteria, &coll).await;
            let _ = tx.send(result.map_err(|e| e.to_string())).await;
        });
    }

    fn accept(&mut self, deck_state: &mut DeckState) {
        let Some(proposal) = self.proposal.take() else {
            return;
        };
        let client = hellfire_api::ScryfallClient::new();
        let format = deck_state.deck.format;
        let name = if self.deck_name.trim().is_empty() {
            proposal.deck_name.clone()
        } else {
            self.deck_name.trim().to_string()
        };
        let entries: Vec<(String, u32, bool)> = proposal
            .cards
            .iter()
            .map(|c| (c.name.clone(), c.quantity, c.sideboard))
            .collect();
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<hellfire_core::Deck, String>>(1);
        deck_state.deck_from_ai = Some(rx);
        deck_state.rt.spawn(async move {
            let mut deck = hellfire_core::Deck::new(name, format);
            let mut errors = Vec::new();
            for (card_name, qty, sideboard) in entries {
                match client.search_card(&card_name).await {
                    Ok(card) => {
                        if let Err(e) = deck.recruit(card, qty, sideboard) {
                            errors.push(format!("{card_name} ({e})"));
                        }
                    }
                    Err(e) => errors.push(format!("{card_name} ({e})")),
                }
            }
            if !errors.is_empty() {
                let _ = tx
                    .send(Err(format!(
                        "certaines cartes ont échoué : {}",
                        errors.join(", ")
                    )))
                    .await;
            } else {
                let _ = tx.send(Ok(deck)).await;
            }
        });
    }
}
