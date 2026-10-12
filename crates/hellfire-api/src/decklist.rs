use std::collections::HashMap;

use hellfire_core::decklist::DeckListEntry;

use crate::scryfall::ScryfallClient;
use crate::{ApiError, ApiResult};

/// Résultat d'un import de decklist.
#[derive(Debug, Default)]
pub struct DeckListImport {
    pub resolved: Vec<(DeckListEntry, hellfire_core::card::Card)>,
    pub failed: Vec<(DeckListEntry, String)>,
}

impl DeckListImport {
    pub fn total_cards(&self) -> u32 {
        self.resolved.iter().map(|(e, _)| e.quantity).sum()
    }
}

impl ScryfallClient {
    /// Importe une decklist déjà parsée : résout chaque nom via Scryfall
    /// (recherche fuzzy). Les cartes introuvables sont listées dans
    /// `failed` sans interrompre l'import.
    pub async fn import_decklist(&self, entries: &[DeckListEntry]) -> ApiResult<DeckListImport> {
        let mut cache: HashMap<String, hellfire_core::card::Card> = HashMap::new();
        let mut import = DeckListImport::default();
        for entry in entries {
            let card = if let Some(c) = cache.get(&entry.name) {
                c.clone()
            } else {
                match self.search_card(&entry.name).await {
                    Ok(c) => {
                        cache.insert(entry.name.clone(), c.clone());
                        c
                    }
                    Err(e) => {
                        import.failed.push((entry.clone(), e.to_string()));
                        continue;
                    }
                }
            };
            import.resolved.push((entry.clone(), card));
        }
        if import.resolved.is_empty() && !import.failed.is_empty() {
            let msg = import
                .failed
                .first()
                .map(|(_, e)| e.clone())
                .unwrap_or_else(|| "aucune carte résolue".into());
            return Err(ApiError::BadResponse(msg));
        }
        Ok(import)
    }
}
