use hellfire_core::{Deck, Format};

use crate::storage_handle::StorageHandle;

/// État partagé du deck courant de la GUI.
pub struct DeckState {
    pub deck: Deck,
    pub import_rx:
        Option<tokio::sync::mpsc::Receiver<(bool, String, Option<Deck>)>>,
    pub storage: StorageHandle,
}

impl DeckState {
    pub fn new() -> Self {
        DeckState {
            deck: Deck::new("Deck sans nom", Format::Commander),
            import_rx: None,
            storage: StorageHandle::new(),
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
}

impl Default for DeckState {
    fn default() -> Self {
        Self::new()
    }
}
