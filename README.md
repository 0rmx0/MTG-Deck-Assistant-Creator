# MTG Deck Assistant Creator

Assistant de création de decks **Magic: The Gathering** assisté par IA (Rust, Windows / macOS / Linux), s'appuyant sur les API **Scryfall** et **EDHREC**.

## État actuel

Squelette applicatif avec interface graphique (egui) incluant un menu **Paramètres** :

- **Base de données** : sélection du moteur **SQLite** (chemin du fichier) ou **PostgreSQL** (hôte, port, base, utilisateur).
- **Fournisseur d'IA** : **Mistral AI**, **OpenAI**, **Anthropic** ou **Google AI**, avec modèle configurable (valeur par défaut par fournisseur) et **saisie de la clé API** (masquée, stockée sous `~/.config/mtg-deck-assistant/` avec permissions restreintes).
- Persistance des paramètres dans `settings.json`.

## Lancer

```bash
cargo run
```

Menu **Fichier → Paramètres…** pour configurer la base de données et le fournisseur d'IA.

## Licence

MIT — voir [LICENSE](LICENSE).
