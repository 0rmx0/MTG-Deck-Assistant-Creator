use serde::Deserialize;

use crate::{ApiError, ApiResult};

const BASE: &str = "https://json.edhrec.com";

/// Client EDHREC — recommandations de commandants et de cartes pour le format Commander.
#[derive(Clone)]
pub struct EdhrecClient {
    http: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EdhrecTheme {
    pub tag: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EdhrecCommander {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub themes: Vec<EdhrecTheme>,
}

impl EdhrecClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent("HellfireDeckbuilder/0.1")
                .build()
                .expect("client HTTP"),
        }
    }

    /// Commandants les plus joués pour une identité de couleur (ex. "br").
    pub async fn top_commanders(&self, color_identity: &str) -> ApiResult<Vec<EdhrecCommander>> {
        let url = format!("{BASE}/commanders/{color_identity}.json");
        let resp: serde_json::Value = self
            .http
            .get(&url)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| {
                ApiError::BadResponse(format!("identité « {color_identity} » vide : {e}"))
            })?
            .json()
            .await?;
        let list = resp
            .pointer("/panels/commanders")
            .or_else(|| resp.get("commanders"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        let mut out = Vec::new();
        for item in list {
            if let Ok(card) = serde_json::from_value::<serde_json::Value>(item.clone()) {
                let name = card
                    .get("card")
                    .or_else(|| card.get("name"))
                    .and_then(|c| match c.get("name") {
                        Some(n) => Some(n.clone()),
                        None => c.as_str().map(|s| serde_json::Value::String(s.to_string())),
                    })
                    .and_then(|n| match n {
                        serde_json::Value::String(s) => Some(s),
                        other => other.as_str().map(|s| s.to_string()),
                    })
                    .unwrap_or_default();
                let slug = card
                    .get("slug")
                    .or_else(|| card.pointer("/card/sanitize"))
                    .and_then(|s| s.as_str())
                    .unwrap_or_default()
                    .to_string();
                if !name.is_empty() {
                    out.push(EdhrecCommander {
                        name,
                        slug,
                        themes: Vec::new(),
                    });
                }
            }
        }
        Ok(out)
    }
}
