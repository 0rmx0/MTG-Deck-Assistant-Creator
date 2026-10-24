use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResult};

/// Critères prédéfinis pour la génération d'un deck.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeckCriteria {
    /// Format cible (label, ex. "Commander").
    pub format: String,
    /// Style de jeu souhaité (aggro, contrôle, midrange, combo...).
    pub style: Option<String>,
    /// Identité de couleur souhaitée (ex. "BR"), vide = libre.
    pub colors: Option<String>,
    /// Commandant imposé (Commander), sinon l'IA choisit.
    pub commander: Option<String>,
    /// Budget maximum en USD (0 = illimité).
    pub budget_usd: Option<f64>,
    /// Notes libres transmises à l'IA.
    pub notes: Option<String>,
}

/// Réponse de l'IA : liste de cartes avec quantités.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiDeckProposal {
    pub deck_name: String,
    pub rationale: String,
    pub cards: Vec<AiDeckCard>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiDeckCard {
    pub name: String,
    pub quantity: u32,
    #[serde(default)]
    pub sideboard: bool,
}

/// Client générique d'assistant IA pour construire un deck.
#[derive(Clone)]
pub struct AiDeckAssistant {
    http: reqwest::Client,
    provider: crate::AiProviderInfo,
}

#[derive(Clone)]
pub struct AiProviderInfo {
    pub name: String,
    pub model: String,
    pub api_key: String,
    pub base_url: String,
}

impl AiDeckAssistant {
    pub fn new(provider: AiProviderInfo) -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent("HellfireDeckbuilder/0.1")
                .build()
                .expect("client HTTP"),
            provider,
        }
    }

    fn system_prompt(&self) -> String {
        "Tu es un expert Magic: The Gathering et un constructeur de decks de haut niveau. \
         Tu reçois la collection physique de cartes de l'utilisateur (nom + quantité + type + coût) \
         et des critères. Tu construis le meilleur deck possible UNIQUEMENT avec des cartes de la \
         collection, en respectant le format (taille minimale, nombre d'exemplaires, légalité). \
         Réponds STRICTEMENT en JSON valide, sans texte autour, au format : \
         {\"deck_name\": string, \"rationale\": string, \"cards\": [{\"name\": string, \"quantity\": int, \"sideboard\": bool}]}."
            .into()
    }

    fn user_prompt(
        &self,
        criteria: &DeckCriteria,
        collection: &[(String, u32, String, u32, bool)],
    ) -> String {
        let mut cards = String::new();
        for (name, qty, type_line, cmc, legal) in collection {
            cards.push_str(&format!(
                "- {name} x{qty} | {type_line} | CMC {cmc} | {}\n",
                if *legal { "légal" } else { "illégal" }
            ));
        }
        let mut crit = format!("Format : {}", criteria.format);
        if let Some(s) = &criteria.style {
            crit.push_str(&format!("\nStyle : {s}"));
        }
        if let Some(c) = &criteria.colors {
            crit.push_str(&format!("\nIdentité de couleur : {c}"));
        }
        if let Some(c) = &criteria.commander {
            crit.push_str(&format!("\nCommandant imposé : {c}"));
        }
        if let Some(b) = criteria.budget_usd {
            crit.push_str(&format!("\nBudget maximum : {b:.0} USD"));
        }
        if let Some(n) = &criteria.notes {
            crit.push_str(&format!("\nNotes : {n}"));
        }
        format!(
            "CRITÈRES :\n{crit}\n\nCOLLECTION DISPONIBLE (ne propose que ces cartes) :\n{cards}"
        )
    }

    /// Demande à l'IA de construire un deck depuis la collection.
    pub async fn build_deck(
        &self,
        criteria: &DeckCriteria,
        collection: &[(String, u32, String, u32, bool)],
    ) -> ApiResult<AiDeckProposal> {
        if collection.is_empty() {
            return Err(ApiError::BadResponse(
                "collection vide : importe d'abord des cartes".into(),
            ));
        }
        let body = serde_json::json!({
            "model": self.provider.model,
            "messages": [
                {"role": "system", "content": self.system_prompt()},
                {"role": "user", "content": self.user_prompt(criteria, collection)}
            ],
            "temperature": 0.4,
            "response_format": {"type": "json_object"}
        });
        let url = format!(
            "{}/chat/completions",
            self.provider.base_url.trim_end_matches('/')
        );
        let resp: serde_json::Value = self
            .http
            .post(url)
            .bearer_auth(&self.provider.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| ApiError::BadResponse(format!("IA injoignable : {e}")))?
            .json()
            .await?;
        let content = resp
            .pointer("/choices/0/message/content")
            .and_then(|c| c.as_str())
            .ok_or_else(|| ApiError::BadResponse("réponse IA sans contenu".into()))?;
        let proposal: AiDeckProposal = serde_json::from_str(content.trim())
            .map_err(|e| ApiError::BadResponse(format!("réponse IA non décodable : {e}")))?;
        if proposal.cards.is_empty() {
            return Err(ApiError::BadResponse(
                "l'IA n'a proposé aucune carte".into(),
            ));
        }
        Ok(proposal)
    }
}
