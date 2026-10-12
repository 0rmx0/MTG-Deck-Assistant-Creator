use eframe::egui;

use crate::settings::{load_api_key, remove_api_key, save_api_key, AiProvider, DbEngine, Settings};

pub struct SettingsWindow {
    pub open: bool,
    api_key_input: String,
    show_key: bool,
    status: Option<(bool, String)>,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        SettingsWindow {
            open: false,
            api_key_input: String::new(),
            show_key: false,
            status: None,
        }
    }
}

impl SettingsWindow {
    pub fn open(&mut self) {
        self.status = None;
        self.api_key_input.clear();
        self.open = true;
    }

    pub fn show(&mut self, ctx: &egui::Context, settings: &mut Settings) {
        let mut keep_open = self.open;
        let title = format!("Paramètres{}", if settings.api_key_stored { " 🔑" } else { "" });
        egui::Window::new(title)
            .open(&mut keep_open)
            .resizable(false)
            .min_width(480.0)
            .show(ctx, |ui| {
                self.contents(ui, settings);
            });
        self.open = keep_open;
    }

    fn contents(&mut self, ui: &mut egui::Ui, settings: &mut Settings) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Base de données");
            ui.horizontal(|ui| {
                ui.label("Moteur :");
                egui::ComboBox::from_id_salt("db_engine_combo")
                    .selected_text(settings.db_engine.label())
                    .width(160.0)
                    .show_ui(ui, |ui| {
                        for engine in DbEngine::ALL {
                            ui.selectable_value(
                                &mut settings.db_engine,
                                engine,
                                engine.label(),
                            );
                        }
                    });
            });

            match settings.db_engine {
                DbEngine::Sqlite => {
                    ui.horizontal(|ui| {
                        ui.label("Fichier :");
                        let mut path_text = settings
                            .sqlite_path
                            .clone()
                            .unwrap_or_else(|| "mtg_decks.db (dans le dossier de config)".into());
                        ui.text_edit_singleline(&mut path_text);
                        if path_text.trim().is_empty() {
                            settings.sqlite_path = None;
                        } else {
                            settings.sqlite_path = Some(path_text.trim().to_string());
                        }
                        if ui.button("Réinitialiser").clicked() {
                            settings.sqlite_path = None;
                        }
                    });
                    ui.small("SQLite : aucune configuration serveur nécessaire.");
                }
                DbEngine::Postgres => {
                    let pg = &mut settings.postgres;
                    ui.horizontal(|ui| {
                        ui.label("Hôte :");
                        ui.text_edit_singleline(&mut pg.host);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Port :");
                        ui.add(egui::DragValue::new(&mut pg.port).range(1..=65535));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Base :");
                        ui.text_edit_singleline(&mut pg.database);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Utilisateur :");
                        ui.text_edit_singleline(&mut pg.user);
                    });
                    ui.small("Le mot de passe PostgreSQL sera demandé à la connexion.");
                }
            }

            ui.separator();
            ui.heading("Fournisseur d'IA");
            ui.horizontal(|ui| {
                ui.label("Fournisseur :");
                egui::ComboBox::from_id_salt("ai_provider_combo")
                    .selected_text(settings.ai_provider.label())
                    .width(160.0)
                    .show_ui(ui, |ui| {
                        for provider in AiProvider::ALL {
                            ui.selectable_value(
                                &mut settings.ai_provider,
                                provider,
                                provider.label(),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Modèle :");
                ui.text_edit_singleline(&mut settings.ai_model);
                if ui
                    .small_button("Par défaut")
                    .on_hover_text("Modèle recommandé pour ce fournisseur")
                    .clicked()
                {
                    settings.ai_model = settings.ai_provider.default_model().into();
                }
            });

            let stored_key = load_api_key(settings);
            let has_stored_key = stored_key.is_some();
            ui.horizontal(|ui| {
                ui.label("Clé API :");
                let mut edit = egui::TextEdit::singleline(&mut self.api_key_input)
                    .desired_width(320.0)
                    .password(!self.show_key)
                    .hint_text(if has_stored_key {
                        "•••••••• (clé enregistrée)"
                    } else {
                        "Collez votre clé API ici"
                    });
                if stored_key.is_some() {
                    edit = edit.hint_text("•••••••• (clé enregistrée)");
                }
                ui.add(edit);
                if ui
                    .small_button(if self.show_key { "🙈" } else { "👁" })
                    .on_hover_text("Afficher / masquer")
                    .clicked()
                {
                    self.show_key = !self.show_key;
                }
            });

            let key_note = if has_stored_key {
                format!(
                    "Une clé est enregistrée pour {}.",
                    settings.ai_provider.label()
                )
            } else {
                "Aucune clé enregistrée pour ce fournisseur.".into()
            };
            ui.small(key_note);

            ui.horizontal(|ui| {
                if ui
                    .button("Enregistrer la clé")
                    .clicked()
                {
                    match save_api_key(settings, &self.api_key_input) {
                        Ok(()) => {
                            self.status = Some((true, "Clé API enregistrée dans le trousseau système.".into()));
                            self.api_key_input.clear();
                        }
                        Err(e) => self.status = Some((false, format!("Erreur : {e}"))),
                    }
                }
                if has_stored_key && ui.button("Supprimer la clé").clicked() {
                    match remove_api_key(settings) {
                        Ok(()) => {
                            self.status = Some((true, "Clé API supprimée.".into()));
                            self.api_key_input.clear();
                        }
                        Err(e) => self.status = Some((false, format!("Erreur : {e}"))),
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    match settings.save() {
                        Ok(()) => self.status = Some((true, "Paramètres enregistrés.".into())),
                        Err(e) => self.status = Some((false, format!("Erreur : {e}"))),
                    }
                }
                if ui.button("Rétablir").clicked() {
                    *settings = Settings::default();
                    self.status = Some((true, "Paramètres réinitialisés.".into()));
                }
                if let Some((ok, msg)) = &self.status {
                    ui.colored_label(
                        if *ok { egui::Color32::from_rgb(40, 160, 80) } else { egui::Color32::from_rgb(200, 60, 60) },
                        msg,
                    );
                }
            });

            ui.separator();
            ui.collapsing("Résumé de la connexion", |ui| {
                ui.monospace(settings.connection_summary());
            });
        });
    }
}

