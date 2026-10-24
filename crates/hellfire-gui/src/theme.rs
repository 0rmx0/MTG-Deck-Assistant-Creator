use eframe::egui;

/// Charte graphique R_M_X — univers 0rmx0.
/// Noir profond lo-fi + néons rares (règle 80/15/5) :
/// ~80 % de noirs, ~15 % de blancs/gris, ~5 % d'accents cyan/magenta/vert.
/// Rayon unique 10 px, halo discret, aucune ombre portée dure.
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    pub const BG_PRIMARY: egui::Color32 = egui::Color32::from_rgb(0x11, 0x11, 0x11);
    pub const BG_SECONDARY: egui::Color32 = egui::Color32::from_rgb(0x22, 0x22, 0x22);
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(0xFF, 0xFF, 0xFF);
    pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(0xCC, 0xCC, 0xCC);
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(0x00, 0xFF, 0xFF);
    pub const MAGENTA: egui::Color32 = egui::Color32::from_rgb(0xFF, 0x00, 0xFF);
    pub const TERMINAL: egui::Color32 = egui::Color32::from_rgb(0x00, 0xFF, 0x7F);
    pub const MTG_WHITE: egui::Color32 = egui::Color32::from_rgb(0xED, 0xE5, 0xD0);
    pub const MTG_BLUE: egui::Color32 = egui::Color32::from_rgb(0x2E, 0x6F, 0xA3);
    pub const MTG_BLACK: egui::Color32 = egui::Color32::from_rgb(0x4B, 0x3B, 0x57);
    pub const MTG_RED: egui::Color32 = egui::Color32::from_rgb(0xC4, 0x3A, 0x2F);
    pub const MTG_GREEN: egui::Color32 = egui::Color32::from_rgb(0x3F, 0x7A, 0x4A);
    pub const MTG_ARTIFACT: egui::Color32 = egui::Color32::from_rgb(0x8C, 0x85, 0x77);
    /// Halo discret rgba(255,255,255,0.1) sur fond #111.
    pub const HALO: egui::Color32 = egui::Color32::from_rgb(0x1C, 0x1C, 0x1C);

    /// Applique le thème R_M_X au contexte egui : fond `#111`, surfaces `#222`,
    /// rayon unique 10 px, focus visible en cyan (l'accessibilité fait partie
    /// du style), une seule transition `transform 0.2s ease` / `scale(1.05)`.
    pub fn install(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.window_fill = Self::BG_SECONDARY;
        style.visuals.panel_fill = Self::BG_PRIMARY;
        style.visuals.extreme_bg_color = Self::BG_PRIMARY;
        style.visuals.faint_bg_color = Self::BG_SECONDARY;
        style.visuals.code_bg_color = Self::BG_SECONDARY;
        style.visuals.hyperlink_color = Self::CYAN;
        style.visuals.override_text_color = Some(Self::TEXT_PRIMARY);
        style.animation_time = 0.2;
        for corner in [
            &mut style.visuals.menu_rounding,
            &mut style.visuals.window_rounding,
            &mut style.visuals.widgets.noninteractive.rounding,
            &mut style.visuals.widgets.inactive.rounding,
            &mut style.visuals.widgets.hovered.rounding,
            &mut style.visuals.widgets.active.rounding,
            &mut style.visuals.widgets.open.rounding,
        ] {
            *corner = egui::Rounding::same(10.0_f32);
        }

        // Halo discret rgba(255,255,255,0.1) ≈ #1B1B1B sur #111, pas d'ombre dure.
        style.visuals.window_stroke = egui::Stroke::new(1.0_f32, Self::HALO);
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, Self::HALO);
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, Self::HALO);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, Self::HALO);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, Self::CYAN);
        style.visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0_f32, Self::HALO);
        style.visuals.window_shadow = egui::Shadow::NONE;
        style.visuals.popup_shadow = egui::Shadow::NONE;
        style.visuals.widgets.noninteractive.bg_fill = Self::BG_SECONDARY;
        style.visuals.widgets.inactive.bg_fill = Self::BG_SECONDARY;
        style.visuals.widgets.hovered.bg_fill = Self::BG_SECONDARY;
        style.visuals.widgets.active.bg_fill = Self::BG_SECONDARY;
        style.visuals.widgets.open.bg_fill = Self::BG_SECONDARY;
        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0_f32, Self::TEXT_SECONDARY);
        style.visuals.widgets.inactive.fg_stroke =
            egui::Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        style.visuals.widgets.hovered.fg_stroke =
            egui::Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, Self::CYAN);
        style.visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        style.visuals.selection.bg_fill = Self::BG_SECONDARY;
        style.visuals.selection.stroke = egui::Stroke::new(1.0_f32, Self::CYAN);
        ctx.set_style(style);
    }

    /// Titres de section : cyan (néon réservé aux points d'interaction).
    pub fn heading(ui: &mut egui::Ui, text: &str) {
        ui.add(
            egui::Label::new(
                egui::RichText::new(text.to_uppercase())
                    .color(Self::CYAN)
                    .strong()
                    .heading(),
            )
            .extend(),
        );
    }

    /// Logo principal : blanc, majuscules, centré.
    pub fn logotype(ui: &mut egui::Ui) {
        ui.add(
            egui::Label::new(
                egui::RichText::new("R_m_X DECK ASSISTANT")
                    .color(Self::TEXT_PRIMARY)
                    .strong()
                    .heading(),
            )
            .extend(),
        );
    }

    /// Métadonnée : gris, petit corps.
    pub fn meta(ui: &mut egui::Ui, text: impl Into<String>) {
        ui.add(
            egui::Label::new(
                egui::RichText::new(text.into())
                    .color(Self::TEXT_SECONDARY)
                    .small(),
            )
            .extend(),
        );
    }

    /// Donnée technique : monospace, vert terminal sur noir.
    pub fn stat(ui: &mut egui::Ui, text: impl Into<String>) {
        ui.add(
            egui::Label::new(
                egui::RichText::new(text.into())
                    .color(Self::TERMINAL)
                    .monospace(),
            )
            .extend(),
        );
    }

    /// Bouton primaire : bordure cyan, texte cyan, fond surface.
    pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
        let button = egui::Button::new(
            egui::RichText::new(label.to_uppercase())
                .color(Self::CYAN)
                .strong(),
        )
        .fill(Self::BG_SECONDARY)
        .stroke(egui::Stroke::new(1.0_f32, Self::CYAN));
        ui.add(button)
    }

    /// Bouton secondaire : bordure et texte gris.
    pub fn secondary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
        let button = egui::Button::new(
            egui::RichText::new(label.to_uppercase())
                .color(Self::TEXT_SECONDARY)
                .strong(),
        )
        .fill(Self::BG_SECONDARY)
        .stroke(egui::Stroke::new(1.0_f32, Self::TEXT_SECONDARY));
        ui.add(button)
    }

    /// Toast / notification : bandeau surface, bordure gauche 4 px.
    /// Magenta = erreur, cyan = info, vert terminal = succès.
    pub fn toast(ui: &mut egui::Ui, ok: bool, msg: &str) {
        let (bar, text_color) = if ok {
            (Self::TERMINAL, Self::TEXT_PRIMARY)
        } else {
            (Self::MAGENTA, Self::TEXT_PRIMARY)
        };
        let frame = egui::Frame::group(&egui::Style::default())
            .fill(Self::BG_SECONDARY)
            .stroke(egui::Stroke::NONE)
            .rounding(egui::Rounding::same(10.0_f32))
            .inner_margin(egui::Margin::symmetric(10.0, 6.0));
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(4.0, 16.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, egui::Rounding::same(0.0_f32), bar);
                ui.add(egui::Label::new(egui::RichText::new(msg).color(text_color)));
            });
        });
    }

    /// Séparateur : trait halo discret.
    pub fn cutter_separator(ui: &mut egui::Ui) {
        let sep = egui::Separator::default().spacing(12.0).horizontal();
        let resp = ui.add(sep);
        ui.painter().line_segment(
            [resp.rect.left_center(), resp.rect.right_center()],
            egui::Stroke::new(1.0_f32, Self::HALO),
        );
    }

    /// Badge format : surface `#222`, texte cyan, rayon 10 px.
    pub fn format_badge(ui: &mut egui::Ui, label: &str) {
        let frame = egui::Frame::group(&egui::Style::default())
            .fill(Self::BG_SECONDARY)
            .stroke(egui::Stroke::new(1.0_f32, Self::HALO))
            .rounding(egui::Rounding::same(10.0_f32))
            .inner_margin(egui::Margin::symmetric(8.0, 2.0));
        frame.show(ui, |ui| {
            ui.add(egui::Label::new(egui::RichText::new(label).color(Self::CYAN).small()).extend());
        });
    }
}
