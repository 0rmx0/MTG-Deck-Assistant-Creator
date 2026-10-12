use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::secret_store::{delete_secret, load_secret, store_secret};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbEngine {
    Sqlite,
    Postgres,
}

impl DbEngine {
    pub const ALL: [DbEngine; 2] = [DbEngine::Sqlite, DbEngine::Postgres];

    pub fn label(&self) -> &'static str {
        match self {
            DbEngine::Sqlite => "SQLite",
            DbEngine::Postgres => "PostgreSQL",
        }
    }
}

impl Default for DbEngine {
    fn default() -> Self {
        DbEngine::Sqlite
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiProvider {
    Mistral,
    OpenAi,
    Anthropic,
    Google,
}

impl AiProvider {
    pub const ALL: [AiProvider; 4] = [
        AiProvider::Mistral,
        AiProvider::OpenAi,
        AiProvider::Anthropic,
        AiProvider::Google,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AiProvider::Mistral => "Mistral AI",
            AiProvider::OpenAi => "OpenAI",
            AiProvider::Anthropic => "Anthropic",
            AiProvider::Google => "Google AI",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            AiProvider::Mistral => "mistral-large-latest",
            AiProvider::OpenAi => "gpt-4o",
            AiProvider::Anthropic => "claude-sonnet-4-5",
            AiProvider::Google => "gemini-2.0-flash",
        }
    }
}

impl Default for AiProvider {
    fn default() -> Self {
        AiProvider::Mistral
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig {
            host: "localhost".into(),
            port: 5432,
            database: "mtg_deck_assistant".into(),
            user: "postgres".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub db_engine: DbEngine,
    pub sqlite_path: Option<String>,
    pub postgres: PostgresConfig,
    pub ai_provider: AiProvider,
    pub ai_model: String,
    #[serde(default)]
    pub api_key_stored: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            db_engine: DbEngine::default(),
            sqlite_path: None,
            postgres: PostgresConfig::default(),
            ai_provider: AiProvider::default(),
            ai_model: AiProvider::default().default_model().into(),
            api_key_stored: false,
        }
    }
}

impl Settings {
    pub fn config_dir() -> PathBuf {
        let base = directories::ProjectDirs::from("com", "mtgdeckassistant", "MTG Deck Assistant")
            .expect("no home directory found");
        let dir = base.config_dir().to_path_buf();
        fs::create_dir_all(&dir).ok();
        dir
    }

    fn config_file() -> PathBuf {
        Self::config_dir().join("settings.json")
    }

    pub fn load() -> Settings {
        match fs::read_to_string(Self::config_file()) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Settings::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_file();
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn connection_summary(&self) -> String {
        match self.db_engine {
            DbEngine::Sqlite => self
                .sqlite_path
                .clone()
                .unwrap_or_else(|| "(emplacement à définir)".into()),
            DbEngine::Postgres => format!(
                "postgresql://{}@{}:{}/{}",
                self.postgres.user, self.postgres.host, self.postgres.port, self.postgres.database
            ),
        }
    }
}

pub fn save_api_key(settings: &mut Settings, key: &str) -> Result<(), String> {
    let key = key.trim();
    if key.is_empty() {
        return Err("La clé API ne peut pas être vide.".into());
    }
    store_secret(settings.ai_provider, key)?;
    settings.api_key_stored = true;
    settings.save()
}

pub fn remove_api_key(settings: &mut Settings) -> Result<(), String> {
    delete_secret(settings.ai_provider)?;
    settings.api_key_stored = false;
    settings.save()
}

pub fn load_api_key(settings: &Settings) -> Option<String> {
    load_secret(settings.ai_provider)
}
