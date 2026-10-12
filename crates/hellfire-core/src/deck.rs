use serde::{Deserialize, Serialize};

use crate::card::{Card, CardKind, Colors};
use crate::error::{CoreError, Result};
use crate::format::Format;
use crate::mana::ManaCurve;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckEntry {
    pub card: Card,
    pub quantity: u32,
    pub sideboard: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeckStats {
    pub total_cards: u32,
    pub creature_count: u32,
    pub spell_count: u32,
    pub land_count: u32,
    pub avg_cmc: f32,
    pub curve: ManaCurve,
    pub price_cents: u64,
    pub colors: Colors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: Option<i64>,
    pub name: String,
    pub format: Format,
    pub entries: Vec<DeckEntry>,
}

impl Deck {
    pub fn new(name: impl Into<String>, format: Format) -> Self {
        Self {
            id: None,
            name: name.into(),
            format,
            entries: Vec::new(),
        }
    }

    /// « Recruter » une carte dans le deck.
    pub fn recruit(&mut self, card: Card, quantity: u32, sideboard: bool) -> Result<()> {
        let max = self.format.max_copies();
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.card.scryfall_id == card.scryfall_id && e.sideboard == sideboard)
        {
            if entry.quantity + quantity > max {
                return Err(CoreError::InvalidDeck(format!(
                    "{} exemplaires max de {} en {}",
                    max, card.name, self.format
                )));
            }
            entry.quantity += quantity;
        } else {
            if quantity > max {
                return Err(CoreError::InvalidDeck(format!(
                    "{} exemplaires max de {} en {}",
                    max, card.name, self.format
                )));
            }
            self.entries.push(DeckEntry {
                card,
                quantity,
                sideboard,
            });
        }
        Ok(())
    }

    /// « Sabotage » : retirer des exemplaires d'une carte.
    pub fn sabotage(&mut self, scryfall_id: &str, quantity: u32) -> Result<()> {
        let pos = self
            .entries
            .iter()
            .position(|e| e.card.scryfall_id == scryfall_id)
            .ok_or_else(|| CoreError::CardNotFound(scryfall_id.to_string()))?;
        if self.entries[pos].quantity <= quantity {
            self.entries.remove(pos);
        } else {
            self.entries[pos].quantity -= quantity;
        }
        Ok(())
    }

    pub fn stats(&self) -> DeckStats {
        let mut s = DeckStats::default();
        let mut cmc_total: f32 = 0.0;
        let mut cmc_cards: u32 = 0;
        let mut identity = Colors::default();
        for e in &self.entries {
            if e.sideboard {
                continue;
            }
            s.total_cards += e.quantity;
            match e.card.kind() {
                CardKind::Creature => s.creature_count += e.quantity,
                CardKind::Spell => s.spell_count += e.quantity,
                CardKind::Land => s.land_count += e.quantity,
            }
            s.curve.add(e.card.cmc(), e.quantity);
            s.price_cents += e.card.price_usd_cents * e.quantity as u64;
            if e.card.kind() != CardKind::Land {
                cmc_total += e.card.cmc() as f32 * e.quantity as f32;
                cmc_cards += e.quantity;
            }
            identity = identity.union(&e.card.color_identity);
        }
        s.avg_cmc = if cmc_cards > 0 { cmc_total / cmc_cards as f32 } else { 0.0 };
        s.colors = identity;
        s
    }

    /// « Lever le poing » : validation du deck pour le format choisi.
    pub fn validate(&self) -> Result<DeckStats> {
        let stats = self.stats();
        let min = self.format.min_deck_size();
        if stats.total_cards < min as u32 {
            return Err(CoreError::InvalidDeck(format!(
                "Ce deck rassemble {} manifestants ? Non : {} cartes, il en faut {} en {}",
                min, stats.total_cards, min, self.format
            )));
        }
        for e in &self.entries {
            if e.quantity > self.format.max_copies() {
                return Err(CoreError::InvalidDeck(format!(
                    "{} exemplaires de {} — la loi c'est {} max",
                    e.quantity, e.card.name, self.format.max_copies()
                )));
            }
        }
        Ok(stats)
    }
}
