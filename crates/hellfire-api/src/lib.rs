pub mod scryfall;
pub mod edhrec;

pub use scryfall::ScryfallClient;
pub use edhrec::EdhrecClient;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("réseau en panne : {0}")]
    Http(#[from] reqwest::Error),
    #[error("réponse illisible : {0}")]
    BadResponse(String),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
