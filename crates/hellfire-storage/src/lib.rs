pub mod postgres;
pub mod sqlite;

use hellfire_core::Deck;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite sabotage : {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("PostgreSQL sabotage : {0}")]
    Postgres(#[from] ::postgres::Error),
    #[error("deck illisible : {0}")]
    Serde(#[from] serde_json::Error),
}

pub type StorageResult<T> = std::result::Result<T, StorageError>;

pub trait DeckStore {
    fn save_deck(&mut self, deck: &Deck) -> StorageResult<i64>;
    fn load_deck(&mut self, id: i64) -> StorageResult<Option<Deck>>;
    fn list_decks(&mut self) -> StorageResult<Vec<(i64, String, String)>>;
    fn delete_deck(&mut self, id: i64) -> StorageResult<()>;
}
