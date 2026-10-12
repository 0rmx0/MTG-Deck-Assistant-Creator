use hellfire_core::{Deck, Format};
use hellfire_storage::sqlite::SqliteStore;
use hellfire_storage::DeckStore;

#[test]
fn roundtrip() {
    let mut store = SqliteStore::in_memory().unwrap();
    let d = Deck::new("Insurrection", Format::Modern);
    let id = store.save_deck(&d).unwrap();
    let loaded = store.load_deck(id).unwrap().unwrap();
    assert_eq!(loaded.name, "Insurrection");
    assert_eq!(loaded.format, Format::Modern);
    assert_eq!(store.list_decks().unwrap().len(), 1);
    store.delete_deck(id).unwrap();
    assert!(store.load_deck(id).unwrap().is_none());
}
