use std::collections::HashMap;

use hellfire_core::card::{Card, Color, Colors};
use hellfire_core::format::Format;
use hellfire_core::mana::ManaCost;
use serde::Deserialize;

use crate::{ApiError, ApiResult};

const BASE: &str = "https://api.scryfall.com";

/// Client Scryfall — cache local des cartes, recherche par nom.
#[derive(Clone)]
pub struct ScryfallClient {
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct SfCard {
    id: String,
    name: String,
    #[serde(default)]
    mana_cost: Option<String>,
    #[serde(default)]
    type_line: String,
    #[serde(default)]
    oracle_text: String,
    #[serde(default)]
    colors: Vec<String>,
    #[serde(default)]
    color_identity: Vec<String>,
    #[serde(default)]
    set: String,
    #[serde(default)]
    rarity: String,
    #[serde(default)]
    prices: SfPrices,
    #[serde(default)]
    legalities: HashMap<String, String>,
}

#[derive(Deserialize, Default)]
struct SfPrices {
    #[serde(default)]
    usd: Option<String>,
}

fn parse_colors(v: &[String]) -> Colors {
    let mut set = std::collections::BTreeSet::new();
    for c in v {
        match c.as_str() {
            "W" => {
                set.insert(Color::White);
            }
            "U" => {
                set.insert(Color::Blue);
            }
            "B" => {
                set.insert(Color::Black);
            }
            "R" => {
                set.insert(Color::Red);
            }
            "G" => {
                set.insert(Color::Green);
            }
            _ => {}
        }
    }
    Colors(set)
}

fn format_key(f: Format) -> &'static str {
    match f {
        Format::Standard => "standard",
        Format::Modern => "modern",
        Format::Pioneer => "pioneer",
        Format::Legacy => "legacy",
        Format::Vintage => "vintage",
        Format::Commander => "commander",
        Format::Pauper => "pauper",
        Format::Brawl => "brawl",
    }
}

impl From<SfCard> for Card {
    fn from(s: SfCard) -> Card {
        let mut legalities = HashMap::new();
        for (k, v) in s.legalities {
            for f in [
                Format::Standard,
                Format::Modern,
                Format::Pioneer,
                Format::Legacy,
                Format::Vintage,
                Format::Commander,
                Format::Pauper,
                Format::Brawl,
            ] {
                if k == format_key(f) {
                    legalities.insert(f, v == "legal");
                }
            }
        }
        Card {
            scryfall_id: s.id,
            name: s.name,
            mana_cost: s.mana_cost.as_deref().map(ManaCost::parse),
            type_line: s.type_line,
            oracle_text: s.oracle_text,
            colors: parse_colors(&s.colors),
            color_identity: parse_colors(&s.color_identity),
            set_code: s.set,
            rarity: s.rarity,
            price_usd_cents: s
                .prices
                .usd
                .and_then(|p| p.parse::<f64>().ok())
                .map(|p| (p * 100.0) as u64)
                .unwrap_or(0),
            legalities,
        }
    }
}

impl ScryfallClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent("HellfireDeckbuilder/0.1")
                .build()
                .expect("client HTTP"),
        }
    }

    /// Recherche fuzzy par nom, ex. « lightning bolt ».
    pub async fn search_card(&self, name: &str) -> ApiResult<Card> {
        let url = format!("{BASE}/cards/named");
        let resp: SfCard = self
            .http
            .get(url)
            .query(&[("fuzzy", name)])
            .send()
            .await?
            .error_for_status()
            .map_err(|e| ApiError::BadResponse(format!("carte « {name} » introuvable : {e}")))?
            .json()
            .await?;
        Ok(resp.into())
    }

    /// Recherche paginée (retourne la première page).
    pub async fn search_query(&self, query: &str) -> ApiResult<Vec<Card>> {
        let url = format!("{BASE}/cards/search");
        let resp: serde_json::Value = self
            .http
            .get(url)
            .query(&[("q", query), ("unique", "cards")])
            .send()
            .await?
            .error_for_status()
            .map_err(|e| ApiError::BadResponse(format!("requête « {query} » rejetée : {e}")))?
            .json()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| ApiError::BadResponse("pas de champ data".into()))?;
        let mut cards = Vec::new();
        for v in data {
            if let Ok(c) = serde_json::from_value::<SfCard>(v.clone()) {
                cards.push(c.into());
            }
        }
        Ok(cards)
    }

    /// Récupère plusieurs cartes par identifiant Scryfall.
    pub async fn collection(&self, ids: &[String]) -> ApiResult<Vec<Card>> {
        let url = format!("{BASE}/cards/collection");
        let body: Vec<serde_json::Value> =
            ids.iter().map(|i| serde_json::json!({ "id": i })).collect();
        let resp: serde_json::Value = self
            .http
            .post(url)
            .json(&serde_json::json!({ "identifiers": body }))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| ApiError::BadResponse(format!("collection refusée : {e}")))?
            .json()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| ApiError::BadResponse("pas de champ data".into()))?;
        let mut cards = Vec::new();
        for v in data {
            if let Ok(c) = serde_json::from_value::<SfCard>(v.clone()) {
                cards.push(c.into());
            }
        }
        Ok(cards)
    }
}
