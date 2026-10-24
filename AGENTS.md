# AGENTS.md — Guide pour agents

Concepteur de decks Magic: The Gathering assisté par IA, écrit en Rust.
Workspace Cargo multi-crate, multiplateforme (Windows / macOS / Linux),
stockage SQLite ou PostgreSQL au choix, données via les API Scryfall et EDHREC.

## Commandes usuelles

- Build complet : `cargo build`
- Lancer la GUI : `cargo run -p hellfire-gui`
- Tests : `cargo test`
- Tests d'un crate : `cargo test -p hellfire-core`
- Vérifications avant commit : `cargo build && cargo test`
- Formatage : `cargo fmt` ; contrôle : `cargo fmt --check`
- Lint : `cargo clippy --all-targets`

## Architecture

```
crates/
├── hellfire-core     Cartes, decks, formats, courbe de mana (pur, testable)
├── hellfire-api      Clients Scryfall + EDHREC (reqwest, async)
├── hellfire-storage  Persistance SQLite (rusqlite) / PostgreSQL (JSONB)
└── hellfire-gui      Interface egui — paramètres, secret store, thème
```

- **hellfire-core** : modèle `Card` / `Deck`, validation par format
  (« Lever le poing »), courbe de mana 0..7+, parsing `{2}{R}{R}`. Pur, sans
  I/O : toute la logique métier testable vit ici.
- **hellfire-api** : `ScryfallClient` (`search_card` fuzzy, `search_query`,
  `collection` batch) et `EdhrecClient` (`top_commanders`). Async (tokio),
  reqwest avec rustls. Ne jamais faire d'I/O réseau ailleurs.
- **hellfire-storage** : trait `DeckStore` implémenté pour SQLite (fichier ou
  mémoire) et PostgreSQL (JSONB). Toute persistance passe par ce trait.
- **hellfire-gui** : application egui (`main.rs`, `deck_state.rs`,
  fenêtres `settings_window.rs` / `import_window.rs`, `theme.rs`).
  La GUI orchestre mais ne contient pas de logique métier.

Règles de dépendances : `core` ne dépend d'aucun autre crate du workspace ;
`api` et `storage` dépendent de `core` ; `gui` dépend des trois.

## Conventions

- Édition Rust 2021, versions gérées via `[workspace.dependencies]`
  dans le `Cargo.toml` racine. Nouvelle dépendance : l'ajouter au workspace,
  puis la référencer avec `.workspace = true`.
- Messages de commit en français, préfixés : `feat:`, `fix:`, `chore:`,
  `gui:`, `api:`, `storage:`, `core:`, `docs:`.
- Code en anglais, identifiants en anglais ; textes de l'interface en français.
- Erreurs : `thiserror` dans les crates de base, `anyhow` réservé aux bords
  (GUI, binaires).
- Pas de commentaires superflus ; le code et les tests servent de
  documentation.
- Les résultats de tests sont déterministes ; ne jamais dépendre d'un appel
  réseau réel dans les tests (les clients API ne sont pas testés contre
  Scryfall/EDHREC en direct).

## Charte graphique

L'identité visuelle (palette, typographie, ton VOX POPULI) est décrite dans
`docs/charte.md` et encodée dans `crates/hellfire-gui/src/theme.rs`
(constantes `Theme::*`). Toute modification visuelle passe par ces deux
fichiers, jamais par des couleurs ou polices codées en dur ailleurs.
Interdits : dégradés SaaS, ombres douces, coins très arrondis, pastels.

## Sécurité

Les clés API IA sont saisies masquées et stockées via
`crates/hellfire-gui/src/secret_store.rs` dans le répertoire de config avec
permissions restreintes (`0600`), persistées dans `settings.json`. Ne jamais
logger ni afficher une clé API, ne jamais la committer.
