use ratatui::style::Color;

/// Charte « Hellfire Deckbuilder » — clair-obscur, gravure, accents néon.
/// Règle 70/20/10 : noirs/bruns dominants, blancs/gris pour le texte,
/// un seul accent néon par zone.
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    pub const BG_PRIMARY: Color = Color::Rgb(0x0A, 0x0A, 0x0A);
    pub const BG_SECONDARY: Color = Color::Rgb(0x1A, 0x17, 0x14);
    pub const BG_ELEVATED: Color = Color::Rgb(0x3C, 0x28, 0x18);
    pub const TEXT_PRIMARY: Color = Color::Rgb(0xF2, 0xEF, 0xE9);
    pub const TEXT_SECONDARY: Color = Color::Rgb(0x9A, 0x93, 0x8A);
    pub const CORAL: Color = Color::Rgb(0xFF, 0x4E, 0x3A);
    pub const CYAN: Color = Color::Rgb(0x00, 0xE5, 0xE5);
    pub const EMBER: Color = Color::Rgb(0xFF, 0x8C, 0x00);
    pub const MTG_WHITE: Color = Color::Rgb(0xED, 0xE5, 0xD0);
    pub const MTG_BLUE: Color = Color::Rgb(0x2E, 0x6F, 0xA3);
    pub const MTG_BLACK: Color = Color::Rgb(0x4B, 0x3B, 0x57);
    pub const MTG_RED: Color = Color::Rgb(0xC4, 0x3A, 0x2F);
    pub const MTG_GREEN: Color = Color::Rgb(0x3F, 0x7A, 0x4A);
    pub const MTG_ARTIFACT: Color = Color::Rgb(0x8C, 0x85, 0x77);

    pub const BANNER: &[&str] = &[
        " ██░ ██ ▓█████  ██▓▄▄▄█████▓ ▄▄▄       ██▀███  ██▓",
        "▓██░ ██▓▓█   ▀ ▓██▒▓  ██▒ ▓▒▒████▄    ▓██ ▒ ██▒▓██▒",
        "▒██░ █▒ ░▒███   ▒██▒▒ ▓██░ ▒░▒██▀▀█▄  ▓██ ░▄█ ▒▒██▒",
    ];
}
