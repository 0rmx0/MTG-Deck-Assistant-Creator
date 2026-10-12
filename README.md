# HELLFIRE DECKBUILDER

> « Ce deck rassemble 12 000 manifestants ? Non : 60 cartes. »

Concepteur de decks Magic: The Gathering assisté par IA — esthétique punk
underground, gravure au trait, clair-obscur. Rust, multiplateforme
(Windows / macOS / Linux), stockage SQLite ou PostgreSQL au choix,
données live via les API Scryfall et EDHREC.

## Démarrage

```bash
cargo run -p hellfire-tui
```

Stockage par défaut : SQLite (`hellfire.db`). Pour PostgreSQL :

```rust
PostgresStore::connect("postgres://user:pass@host/hellfire")
```

## Architecture

```
crates/
├── hellfire-core     Cartes, decks, formats, courbe de mana (pur, testable)
├── hellfire-api      Clients Scryfall + EDHREC (reqwest, async)
├── hellfire-storage  Persistance SQLite (rusqlite) / PostgreSQL
└── hellfire-tui      Interface terminal (ratatui) — charte Hellfire
```

- **hellfire-core** : modèle `Card` / `Deck`, validation par format
  (`validate` = « Lever le poing »), courbe de mana 0..7+, parsing `{2}{R}{R}`.
- **hellfire-api** : `ScryfallClient::search_card` (fuzzy), `search_query`
  (recherche avancée), `collection` (batch). `EdhrecClient::top_commanders`
  pour les recommandations Commander par identité de couleur.
- **hellfire-storage** : trait `DeckStore` implémenté pour SQLite (fichier
  ou mémoire) et PostgreSQL (JSONB).
- **hellfire-tui** : interface sombre selon la charte (voir
  `crates/hellfire-tui/src/theme.rs` — tokens couleur exacts) : 70 % noirs
  / bruns, 20 % blancs / gris, 10 % accents néon (corail dominant, cyan en
  contrepoint), courbe de mana en barres corail sur grille charbon.

## Tests

```bash
cargo test
```

## Charte graphique

L'identité visuelle (palette, typographie, ton éditorial VOX POPULI) est
décrite dans `docs/charte.md` et encodée dans
`crates/hellfire-tui/src/theme.rs`.

## Licence

MIT — voir LICENSE.
