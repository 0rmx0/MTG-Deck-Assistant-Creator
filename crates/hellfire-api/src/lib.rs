pub mod ai_deck;
pub mod decklist;
pub mod edhrec;
pub mod scryfall;

pub use ai_deck::{AiDeckAssistant, AiDeckCard, AiDeckProposal, AiProviderInfo, DeckCriteria};
pub use decklist::DeckListImport;
pub use edhrec::EdhrecClient;
pub use scryfall::ScryfallClient;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("réseau en panne : {0}")]
    Http(#[from] reqwest::Error),
    #[error("réponse illisible : {0}")]
    BadResponse(String),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
