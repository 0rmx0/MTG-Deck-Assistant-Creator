use hellfire_api::AiDeckProposal;
use hellfire_core::Card;
use hellfire_core::{Deck, Format};

use crate::storage_handle::StorageHandle;

/// État partagé du deck courant de la GUI.
pub struct DeckState {
    pub rt: tokio::runtime::Handle,
    pub deck: Deck,
    pub import_rx: Option<tokio::sync::mpsc::Receiver<(bool, String, Option<Deck>)>>,
    pub collection_rx: Option<tokio::sync::mpsc::Receiver<Result<(Card, u32), String>>>,
    pub ai_rx: Option<tokio::sync::mpsc::Receiver<Result<AiDeckProposal, String>>>,
    pub deck_from_ai: Option<tokio::sync::mpsc::Receiver<Result<Deck, String>>>,
    pub storage: StorageHandle,
}

impl DeckState {
    pub fn new(rt: tokio::runtime::Handle, settings: &crate::settings::Settings) -> Self {
        DeckState {
            rt,
            deck: Deck::new("Deck sans nom", Format::Commander),
            import_rx: None,
            collection_rx: None,
            ai_rx: None,
            deck_from_ai: None,
            storage: StorageHandle::new(settings),
        }
    }

    /// Consomme un résultat d'import prêt, persiste le deck et remonte le statut.
    pub fn poll_import(&mut self) -> Option<(bool, String)> {
        let rx = self.import_rx.as_mut()?;
        let (ok, msg, deck) = rx.try_recv().ok()?;
        self.import_rx = None;
        if let Some(mut deck) = deck {
            if let Err(e) = self.storage.save(&mut deck) {
                return Some((false, format!("deck importé mais non sauvegardé : {e}")));
            }
            self.deck = deck;
        }
        Some((ok, msg))
    }

    pub fn poll_collection(&mut self) -> Option<Result<(Card, u32), String>> {
        let rx = self.collection_rx.as_mut()?;
        let result = rx.try_recv().ok()?;
        self.collection_rx = None;
        if let Ok((card, qty)) = &result {
            let _ = self.storage.upsert_card(card, *qty);
        }
        Some(result)
    }

    pub fn poll_ai(&mut self) -> Option<Result<AiDeckProposal, String>> {
        let rx = self.ai_rx.as_mut()?;
        let result = rx.try_recv().ok()?;
        self.ai_rx = None;
        Some(result)
    }

    pub fn poll_deck_from_ai(&mut self) -> Option<Result<Deck, String>> {
        let rx = self.deck_from_ai.as_mut()?;
        let result = rx.try_recv().ok()?;
        self.deck_from_ai = None;
        if let Ok(deck) = &result {
            let mut deck = deck.clone();
            if let Err(e) = self.storage.save(&mut deck) {
                return Some(Err(format!("deck généré mais non sauvegardé : {e}")));
            }
            self.deck = deck;
        }
        Some(result)
    }
}

impl Default for DeckState {
    fn default() -> Self {
        unimplemented!()
    }
}
