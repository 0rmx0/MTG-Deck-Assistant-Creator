use hellfire_core::{Card, Deck};
use rusqlite::{params, Connection};

use crate::{CollectionStore, DeckStore, StorageResult};

/// Stockage SQLite — un seul fichier, zéro serveur.
pub struct SqliteStore {
    conn: Connection,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS decks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    format TEXT NOT NULL,
    data TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS collection (
    scryfall_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    data TEXT NOT NULL
);
"#;

impl SqliteStore {
    pub fn open(path: &str) -> StorageResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn in_memory() -> StorageResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }
}

impl DeckStore for SqliteStore {
    fn save_deck(&mut self, deck: &Deck) -> StorageResult<i64> {
        let data = serde_json::to_string(deck)?;
        match deck.id {
            Some(id) => {
                self.conn.execute(
                    "UPDATE decks SET name = ?1, format = ?2, data = ?3 WHERE id = ?4",
                    params![deck.name, deck.format.label(), data, id],
                )?;
                Ok(id)
            }
            None => {
                self.conn.execute(
                    "INSERT INTO decks (name, format, data) VALUES (?1, ?2, ?3)",
                    params![deck.name, deck.format.label(), data],
                )?;
                Ok(self.conn.last_insert_rowid())
            }
        }
    }

    fn load_deck(&mut self, id: i64) -> StorageResult<Option<Deck>> {
        let mut stmt = self.conn.prepare("SELECT data FROM decks WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        match rows.next()? {
            Some(row) => {
                let data: String = row.get(0)?;
                Ok(Some(serde_json::from_str(&data)?))
            }
            None => Ok(None),
        }
    }

    fn list_decks(&mut self) -> StorageResult<Vec<(i64, String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, format FROM decks ORDER BY id DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    fn delete_deck(&mut self, id: i64) -> StorageResult<()> {
        self.conn
            .execute("DELETE FROM decks WHERE id = ?1", params![id])?;
        Ok(())
    }
}

impl CollectionStore for SqliteStore {
    fn upsert_card(&mut self, card: &Card, quantity: u32) -> StorageResult<()> {
        let data = serde_json::to_string(card)?;
        self.conn.execute(
            "INSERT INTO collection (scryfall_id, name, quantity, data) VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(scryfall_id) DO UPDATE SET quantity = quantity + ?3, data = ?4",
            params![card.scryfall_id, card.name, quantity, data],
        )?;
        Ok(())
    }

    fn set_quantity(&mut self, scryfall_id: &str, quantity: u32) -> StorageResult<()> {
        self.conn.execute(
            "UPDATE collection SET quantity = ?2 WHERE scryfall_id = ?1",
            params![scryfall_id, quantity],
        )?;
        Ok(())
    }

    fn remove_card(&mut self, scryfall_id: &str) -> StorageResult<()> {
        self.conn.execute(
            "DELETE FROM collection WHERE scryfall_id = ?1",
            params![scryfall_id],
        )?;
        Ok(())
    }

    fn load_collection(&mut self) -> StorageResult<Vec<(Card, u32)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data, quantity FROM collection ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
        })?;
        let mut out = Vec::new();
        for r in rows {
            let (data, qty) = r?;
            out.push((serde_json::from_str(&data)?, qty));
        }
        Ok(out)
    }

    fn clear_collection(&mut self) -> StorageResult<()> {
        self.conn.execute("DELETE FROM collection", [])?;
        Ok(())
    }
}
