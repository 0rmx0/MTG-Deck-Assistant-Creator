use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("carte introuvable: {0}")]
    CardNotFound(String),
    #[error("deck invalide: {0}")]
    InvalidDeck(String),
    #[error("decklist invalide: {0}")]
    Decklist(String),
    #[error("erreur de sérialisation: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;
