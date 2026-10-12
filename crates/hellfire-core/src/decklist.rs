use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// Ligne d'une decklist importée : seule la quantité et le nom sont obligatoires.
///
/// Formats acceptés (un par ligne) :
/// - `4 Lightning Bolt`
/// - `4x Lightning Bolt`
/// - `1 Cavalerie ninja mécanisée (TMT) 156`
/// - `SB: 2 Red Elemental Blast`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckListEntry {
    pub quantity: u32,
    pub name: String,
    #[serde(default)]
    pub set_code: Option<String>,
    #[serde(default)]
    pub collector_number: Option<String>,
    #[serde(default)]
    pub sideboard: bool,
}

/// Parse une decklist texte en entrées. Les lignes vides et les
/// commentaires (`//`, `#`) sont ignorés.
pub fn parse_decklist(text: &str) -> Result<Vec<DeckListEntry>> {
    let mut entries = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        let entry = parse_entry(line).ok_or_else(|| {
            CoreError::Decklist(format!("ligne {} illisible : « {line} »", idx + 1))
        })?;
        entries.push(entry);
    }
    Ok(entries)
}

fn parse_entry(line: &str) -> Option<DeckListEntry> {
    let mut sideboard = false;
    let mut trimmed = line.trim_start();
    if trimmed
        .get(..3)
        .is_some_and(|p| p.eq_ignore_ascii_case("SB:"))
    {
        sideboard = true;
        trimmed = trimmed[3..].trim_start();
    }
    let digit_len = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
    if digit_len == 0 {
        return None;
    }
    let quantity: u32 = trimmed[..digit_len].parse().ok()?;
    let mut rest = trimmed[digit_len..].trim_start();
    if let Some(after) = rest.strip_prefix(['x', 'X']) {
        rest = after.trim_start();
    }

    let mut tokens: Vec<&str> = rest.split_whitespace().collect();

    let mut collector_number = None;
    if tokens.len() >= 2 {
        if let Some(last) = tokens.last() {
            if !last.is_empty() && last.chars().all(|c| c.is_ascii_digit()) {
                collector_number = Some((*last).to_string());
                tokens.pop();
            }
        }
    }

    let mut set_code = None;
    if tokens.len() >= 2 {
        if let Some(last) = tokens.last().copied() {
            if let Some(inner) = last.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
                let valid = (3..=8).contains(&inner.len())
                    && inner.chars().all(|c| c.is_ascii_alphanumeric())
                    && inner.chars().any(|c| c.is_ascii_alphabetic());
                if valid {
                    set_code = Some(inner.to_string());
                    tokens.pop();
                }
            }
        }
    }

    let name = tokens.join(" ");
    if name.is_empty() {
        return None;
    }

    Some(DeckListEntry {
        quantity,
        name,
        set_code,
        collector_number,
        sideboard,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_tmnt_list() {
        let text = "\
1 Cavalerie ninja mécanisée (TMT) 156
1 Chat en crème glacée (TMT) 150
1 Old Hob, blues du chat de gouttière (TMT) 99
";
        let entries = parse_decklist(text).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].quantity, 1);
        assert_eq!(entries[0].name, "Cavalerie ninja mécanisée");
        assert_eq!(entries[0].set_code.as_deref(), Some("TMT"));
        assert_eq!(entries[0].collector_number.as_deref(), Some("156"));
        assert_eq!(entries[2].name, "Old Hob, blues du chat de gouttière");
    }

    #[test]
    fn parses_minimal_and_variants() {
        let text = "\
4 Lightning Bolt
4x Shock
SB: 2 Red Elemental Blast
// commentaire
# autre commentaire

2 Force of Savagery (M13)
";
        let entries = parse_decklist(text).unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries[0],
            DeckListEntry {
                quantity: 4,
                name: "Lightning Bolt".into(),
                set_code: None,
                collector_number: None,
                sideboard: false,
            }
        );
        assert_eq!(entries[1].quantity, 4);
        assert_eq!(entries[1].name, "Shock");
        assert!(entries[2].sideboard);
        assert_eq!(entries[2].quantity, 2);
        assert_eq!(entries[3].set_code.as_deref(), Some("M13"));
        assert_eq!(entries[3].collector_number, None);
    }

    #[test]
    fn rejects_garbage_lines() {
        let err = parse_decklist("Cavalerie ninja mécanisée")
            .unwrap_err()
            .to_string();
        assert!(err.contains("ligne 1 illisible"), "{err}");
    }

    #[test]
    fn empty_and_comments_only() {
        assert!(parse_decklist("").unwrap().is_empty());
        assert!(parse_decklist("// rien\n# nada\n").unwrap().is_empty());
    }
}
