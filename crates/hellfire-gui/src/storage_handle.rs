use hellfire_core::Card;
use hellfire_core::Deck;
use hellfire_storage::sqlite::SqliteStore;
use hellfire_storage::DeckStore;
use hellfire_storage::{CollectionStore, StorageResult};

/// Accès au stockage (decks + collection). SQLite par défaut.
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
            .expect("stockage indisponible");
        StorageHandle { store }
    }

    pub fn save(&mut self, deck: &mut Deck) -> StorageResult<i64> {
        self.store.save_deck(deck)
    }

    pub fn load_deck(&mut self, id: i64) -> StorageResult<Option<Deck>> {
        self.store.load_deck(id)
    }

    pub fn list_decks(&mut self) -> StorageResult<Vec<(i64, String, String)>> {
        self.store.list_decks()
    }

    pub fn delete_deck(&mut self, id: i64) -> StorageResult<()> {
        self.store.delete_deck(id)
    }

    pub fn upsert_card(&mut self, card: &Card, quantity: u32) -> StorageResult<()> {
        self.store.upsert_card(card, quantity)
    }

    pub fn set_quantity(&mut self, scryfall_id: &str, quantity: u32) -> StorageResult<()> {
        self.store.set_quantity(scryfall_id, quantity)
    }

    pub fn remove_card(&mut self, scryfall_id: &str) -> StorageResult<()> {
        self.store.remove_card(scryfall_id)
    }

    pub fn load_collection(&mut self) -> StorageResult<Vec<(Card, u32)>> {
        self.store.load_collection()
    }

    pub fn clear_collection(&mut self) -> StorageResult<()> {
        self.store.clear_collection()
    }
}

impl Default for StorageHandle {
    fn default() -> Self {
        Self::new()
    }
}
