use hellfire_core::Card;

use crate::StorageResult;

/// Stockage de la collection personnelle de cartes.
pub trait CollectionStore {
    fn upsert_card(&mut self, card: &Card, quantity: u32) -> StorageResult<()>;
    fn set_quantity(&mut self, scryfall_id: &str, quantity: u32) -> StorageResult<()>;
    fn remove_card(&mut self, scryfall_id: &str) -> StorageResult<()>;
    fn load_collection(&mut self) -> StorageResult<Vec<(Card, u32)>>;
    fn clear_collection(&mut self) -> StorageResult<()>;
}
