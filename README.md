# HELLFIRE DECKBUILDER

> « Ce deck rassemble 12 000 manifestants ? Non : 60 cartes. »

Concepteur de decks **Magic: The Gathering** assisté par IA — esthétique punk
underground, gravure au trait, clair-obscur. Rust, multiplateforme
(Windows / macOS / Linux), stockage SQLite ou PostgreSQL au choix,
données live via les API **Scryfall** et **EDHREC**.

## Lancer

Interface graphique (egui) :

```bash
cargo run -p hellfire-gui
```

Menu **Fichier → Paramètres…** (GUI) pour configurer :

- **Base de données** : moteur **SQLite** (chemin de fichier) ou
  **PostgreSQL** (hôte, port, base, utilisateur).
- **Fournisseur d'IA** : **Mistral AI**, **OpenAI**, **Anthropic** ou
  **Google AI**, modèle configurable (valeur par défaut par fournisseur).
- **Clé API** : saisie masquée, stockée dans le répertoire de config avec
  permissions restreintes (`0600`). Persistance dans `settings.json`.

## Architecture

```
crates/
├── hellfire-core     Cartes, decks, formats, courbe de mana (pur, testable)
├── hellfire-api      Clients Scryfall + EDHREC (reqwest, async)
├── hellfire-storage  Persistance SQLite (rusqlite) / PostgreSQL
└── hellfire-gui      Interface graphique (egui) — paramètres, secret store
```

- **hellfire-core** : modèle `Card` / `Deck`, validation par format
  (« Lever le poing »), courbe de mana 0..7+, parsing `{2}{R}{R}`.
- **hellfire-api** : `ScryfallClient::search_card` (fuzzy), `search_query`
  (recherche avancée), `collection` (batch). `EdhrecClient::top_commanders`
  pour les recommandations Commander par identité de couleur.
- **hellfire-storage** : trait `DeckStore` implémenté pour SQLite (fichier
  ou mémoire) et PostgreSQL (JSONB).
- **hellfire-gui** : application egui avec menu Paramètres (moteur DB,
  fournisseur IA, clé API masquée) et secret store local sécurisé.

## Tests

```bash
cargo test
```

## Charte graphique

L'identité visuelle (palette, typographie, ton éditorial VOX POPULI) est
décrite dans `docs/charte.md` et encodée dans
`crates/hellfire-gui/src/theme.rs`.

## Licence

MIT — voir [LICENSE](LICENSE).
