use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::format::Format;
use crate::mana::ManaCost;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Colors(pub BTreeSet<Color>);

impl Colors {
    pub fn single(c: Color) -> Self {
        Self(BTreeSet::from([c]))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_mono(&self) -> bool {
        self.0.len() == 1
    }

    /// Identité de couleur d'une carte (contenu des règles de mana).
    pub fn union(&self, other: &Colors) -> Colors {
        let mut set = self.0.clone();
        set.extend(other.0.iter().copied());
        Colors(set)
    }

    pub fn letters(&self) -> String {
        let mut s = String::new();
        for c in &self.0 {
            s.push(match c {
                Color::White => 'W',
                Color::Blue => 'U',
                Color::Black => 'B',
                Color::Red => 'R',
                Color::Green => 'G',
            });
        }
        s
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFace {
    pub name: String,
    pub mana_cost: Option<ManaCost>,
    #[serde(rename = "type_line")]
    pub type_line: String,
    #[serde(default)]
    pub oracle_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardKind {
    Creature,
    Spell,
    Land,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub scryfall_id: String,
    pub name: String,
    #[serde(default)]
    pub mana_cost: Option<ManaCost>,
    #[serde(rename = "type_line")]
    pub type_line: String,
    #[serde(default)]
    pub oracle_text: String,
    #[serde(default)]
    pub colors: Colors,
    #[serde(default)]
    pub color_identity: Colors,
    #[serde(default)]
    pub set_code: String,
    #[serde(default)]
    pub rarity: String,
    #[serde(default = "default_cents")]
    pub price_usd_cents: u64,
    #[serde(default)]
    pub legalities: std::collections::HashMap<Format, bool>,
}

fn default_cents() -> u64 {
    0
}

impl Card {
    pub fn kind(&self) -> CardKind {
        if self.type_line.contains("Land") {
            CardKind::Land
        } else if self.type_line.contains("Creature") {
            CardKind::Creature
        } else {
            CardKind::Spell
        }
    }

    pub fn cmc(&self) -> u32 {
        self.mana_cost.as_ref().map(|m| m.cmc()).unwrap_or(0)
    }

    pub fn is_legal_in(&self, format: Format) -> bool {
        self.legalities.get(&format).copied().unwrap_or(false)
    }
}
