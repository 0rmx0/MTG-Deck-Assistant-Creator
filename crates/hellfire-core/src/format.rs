use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Format {
    Standard,
    Modern,
    Pioneer,
    Legacy,
    Vintage,
    Commander,
    Pauper,
    Brawl,
}

impl Format {
    pub fn min_deck_size(self) -> usize {
        match self {
            Format::Commander => 100,
            Format::Brawl => 60,
            _ => 60,
        }
    }

    pub fn max_copies(self) -> u32 {
        match self {
            Format::Commander | Format::Brawl => 1,
            _ => 4,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Format::Standard => "STANDARD",
            Format::Modern => "MODERN",
            Format::Pioneer => "PIONEER",
            Format::Legacy => "LEGACY",
            Format::Vintage => "VINTAGE",
            Format::Commander => "COMMANDER",
            Format::Pauper => "PAUPER",
            Format::Brawl => "BRAWL",
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}
