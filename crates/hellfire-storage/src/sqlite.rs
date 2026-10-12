use hellfire_core::Deck;
use rusqlite::{params, Connection};

use crate::{DeckStore, StorageResult};

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
