use hellfire_core::Deck;
use hellfire_storage::{sqlite::SqliteStore, DeckStore, StorageResult};

/// Accès au stockage des decks. SQLite par défaut (fichier dans le
/// répertoire de config) ; l'implémentation est volontairement simple
/// tant que le choix du moteur n'est pas câblé dans la GUI.
pub struct StorageHandle {
    store: SqliteStore,
}

impl StorageHandle {
    pub fn new() -> Self {
        let dir = crate::settings::Settings::config_dir();
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("decks.db");
        let store = SqliteStore::open(path.to_string_lossy().as_ref())
            .or_else(|_| SqliteStore::in_memory())
            .expect("stockage decks indisponible");
        StorageHandle { store }
    }

    pub fn save(&mut self, deck: &mut Deck) -> StorageResult<i64> {
        self.store.save_deck(deck)
    }
}

impl Default for StorageHandle {
    fn default() -> Self {
        Self::new()
    }
}
