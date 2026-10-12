use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::card::Colors;

/// Coût de mana générique : nombre de symboles par couleur + générique.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManaCost {
    #[serde(default)]
    pub generic: u32,
    #[serde(default)]
    pub white: u32,
    #[serde(default)]
    pub blue: u32,
    #[serde(default)]
    pub black: u32,
    #[serde(default)]
    pub red: u32,
    #[serde(default)]
    pub green: u32,
}

impl ManaCost {
    pub fn cmc(&self) -> u32 {
        self.generic + self.white + self.blue + self.black + self.red + self.green
    }

    pub fn colors(&self) -> Colors {
        let mut set = std::collections::BTreeSet::new();
        if self.white > 0 {
            set.insert(crate::card::Color::White);
        }
        if self.blue > 0 {
            set.insert(crate::card::Color::Blue);
        }
        if self.black > 0 {
            set.insert(crate::card::Color::Black);
        }
        if self.red > 0 {
            set.insert(crate::card::Color::Red);
        }
        if self.green > 0 {
            set.insert(crate::card::Color::Green);
        }
        Colors(set)
    }

    /// Parse une notation ManaCrypt/Scryfall du type "{2}{R}{R}".
    pub fn parse(s: &str) -> ManaCost {
        let mut cost = ManaCost::default();
        for sym in s.split_inclusive('}') {
            let inner = sym.trim_matches(|c| c == '{' || c == '}');
            match inner {
                "W" => cost.white += 1,
                "U" => cost.blue += 1,
                "B" => cost.black += 1,
                "R" => cost.red += 1,
                "G" => cost.green += 1,
                digits if digits.chars().all(|c| c.is_ascii_digit()) && !digits.is_empty() => {
                    if let Ok(n) = digits.parse::<u32>() {
                        cost.generic += n;
                    }
                }
                _ => {}
            }
        }
        cost
    }

    pub fn to_notation(&self) -> String {
        let mut s = String::new();
        if self.generic > 0 {
            s.push_str(&format!("{{{}}}", self.generic));
        }
        for (n, c) in [(self.white, "W"), (self.blue, "U"), (self.black, "B"), (self.red, "R"), (self.green, "G")] {
            for _ in 0..n {
                s.push_str(&format!("{{{}}}", c));
            }
        }
        s
    }
}

/// Courbe de mana agrégée par CMC 0..7+.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManaCurve(pub BTreeMap<u32, u32>);

impl ManaCurve {
    pub fn add(&mut self, cmc: u32, count: u32) {
        *self.0.entry(cmc.min(7)).or_insert(0) += count;
    }

    pub fn get(&self, cmc: u32) -> u32 {
        self.0.get(&cmc.min(7)).copied().unwrap_or(0)
    }

    pub fn max(&self) -> u32 {
        self.0.values().copied().max().unwrap_or(1)
    }
}
