use hellfire_core::Deck;
use postgres::{Client, NoTls};

use crate::{DeckStore, StorageResult};

/// Stockage PostgreSQL — pour les cellules importantes.
pub struct PostgresStore {
    client: Client,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS decks (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    format TEXT NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
"#;

impl PostgresStore {
    pub fn connect(url: &str) -> StorageResult<Self> {
        let mut client = Client::connect(url, NoTls)?;
        client.batch_execute(SCHEMA)?;
        Ok(Self { client })
    }
}

impl DeckStore for PostgresStore {
    fn save_deck(&mut self, deck: &Deck) -> StorageResult<i64> {
        let data = serde_json::to_string(deck)?;
        match deck.id {
            Some(id) => {
                self.client.execute(
                    "UPDATE decks SET name = $1, format = $2, data = $3 WHERE id = $4",
                    &[&deck.name, &deck.format.label(), &data, &id],
                )?;
                Ok(id)
            }
            None => {
                let row = self.client.query_one(
                    "INSERT INTO decks (name, format, data) VALUES ($1, $2, $3) RETURNING id",
                    &[&deck.name, &deck.format.label(), &data],
                )?;
                Ok(row.get(0))
            }
        }
    }

    fn load_deck(&mut self, id: i64) -> StorageResult<Option<Deck>> {
        let rows = self.client.query("SELECT data FROM decks WHERE id = $1", &[&id])?;
        match rows.first() {
            Some(row) => {
                let data: String = row.get(0);
                Ok(Some(serde_json::from_str(&data)?))
            }
            None => Ok(None),
        }
    }

    fn list_decks(&mut self) -> StorageResult<Vec<(i64, String, String)>> {
        let rows = self.client.query("SELECT id, name, format FROM decks ORDER BY id DESC", &[])?;
        Ok(rows
            .iter()
            .map(|r| (r.get(0), r.get(1), r.get(2)))
            .collect())
    }

    fn delete_deck(&mut self, id: i64) -> StorageResult<()> {
        self.client.execute("DELETE FROM decks WHERE id = $1", &[&id])?;
        Ok(())
    }
}

