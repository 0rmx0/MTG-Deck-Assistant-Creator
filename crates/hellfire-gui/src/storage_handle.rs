use hellfire_core::{Card, Deck};
use hellfire_storage::sqlite::SqliteStore;
use hellfire_storage::{CollectionStore, DeckStore, StorageResult};

use crate::settings::Settings;

/// Accès au stockage (decks + collection).
///
/// SQLite : fichier `decks.db` dans le répertoire de config, ou chemin
/// personnalisé via Paramètres. PostgreSQL : paramètres de connexion.
/// En cas d'échec d'ouverture, bascule en mémoire avec un avertissement
/// explicite (les données ne seront PAS conservées).
pub struct StorageHandle {
    store: SqliteStore,
    pub in_memory: bool,
    pub location: String,
}

impl StorageHandle {
    pub fn new(settings: &Settings) -> Self {
        let (store, in_memory, location) = Self::open_sqlite(settings);
        StorageHandle {
            store,
            in_memory,
            location,
        }
    }

    fn open_sqlite(settings: &Settings) -> (SqliteStore, bool, String) {
        let custom = settings
            .sqlite_path
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        let (path, location) = match custom {
            Some(p) => (std::path::PathBuf::from(p), p.to_string()),
            None => {
                let dir = Settings::config_dir();
                let _ = std::fs::create_dir_all(&dir);
                let path = dir.join("decks.db");
                (path.clone(), path.to_string_lossy().into_owned())
            }
        };
        if let Ok(store) = SqliteStore::open(path.to_string_lossy().as_ref()) {
            return (store, false, location);
        }
        let store = SqliteStore::in_memory().expect("stockage indisponible");
        (store, true, location)
    }

    /// Avertissement à afficher si les données ne sont pas persistées.
    pub fn warning(&self) -> Option<String> {
        if self.in_memory {
            Some(format!(
                "Stockage en mémoire : impossible d'ouvrir « {} ». Les données ne seront PAS conservées après la fermeture.",
                self.location
            ))
        } else {
            None
        }
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
