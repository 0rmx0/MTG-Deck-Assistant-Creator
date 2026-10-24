# Charte graphique R_M_X — Hellfire Deck Assistant

Identité : noir profond lo-fi + néons rares + couches cachées. Esthétique
lostwave/punk où la nudité technique est assumée et où ce qui est caché vaut
plus que ce qui est visible.

Les tokens couleur sont encodés dans `crates/hellfire-gui/src/theme.rs`
(constantes `Theme::*`, valeurs RGB identiques aux hex de la charte).

## Règles universelles

- **Dark-first** : fond `#111111`, surfaces `#222222`, texte `#FFFFFF` / `#CCCCCC`.
- **Règle 80/15/5** : ~80 % de noirs, ~15 % de blancs/gris, ~5 % d'accents
  (cyan `#00FFFF`, magenta `#FF00FF`, vert terminal `#00FF7F`).
  Le néon ne brille que sur du noir ; jamais plus d'un accent par zone.
- **Rayon unique 10 px**, halo discret `rgba(255,255,255,0.1)` (≈ `#1C1C1C`),
  hover `scale(1.05)` en `transform 0.2s ease` — le seul effet de mouvement
  autorisé (`animation_time = 0.2` dans egui).
- **Focus clavier visible en cyan** — l'accessibilité fait partie du style.
- **Lo-fi assumé** : texte machine (base64, IDs, stats) en monospace, vert
  terminal sur noir.
- **Une couche cachée minimum** (voir plus bas) — la signature R_M_X. Elle
  n'est jamais expliquée dans l'interface.

## Interdits

Fond clair, pastel, bleu corporate, dégradés SaaS, ombres portées dures,
expliquer ou spoiler les couches cachées, supprimer du contenu (on retire de
l'état, on n'efface pas).

## Palette

| Rôle             | Nom           | Hex       | Constante         |
|------------------|---------------|-----------|-------------------|
| Fond principal   | Noir profond  | `#111111` | `BG_PRIMARY`      |
| Surface          | Anthracite   | `#222222` | `BG_SECONDARY`    |
| Halo discret     | Blanc 10 %   | `#1C1C1C` | `HALO`            |
| Texte principal  | Blanc         | `#FFFFFF` | `TEXT_PRIMARY`    |
| Texte secondaire| Gris          | `#CCCCCC` | `TEXT_SECONDARY`  |
| Accent 1 (CTA/UX)| Cyan néon     | `#00FFFF` | `CYAN`            |
| Accent 2 (erreur)| Magenta néon  | `#FF00FF` | `MAGENTA`         |
| Accent 3 (données)| Vert terminal | `#00FF7F` | `TERMINAL`       |

## Couleurs MTG (adaptées sombres)

Blanc `#EDE5D0`, Bleu `#2E6FA3`, Noir `#4B3B57`, Rouge `#C43A2F`,
Vert `#3F7A4A`, Artefact `#8C8577`.

## Typographie

- Titres/logo : MAJUSCULES, police display (HelpMe en web ; fallback
  system-ui dans la GUI native).
- Corps/UI : system-ui / sans-serif brute — pas de police « design ».
- Données (base64, stats, mana, IDs) : monospace, vert terminal sur noir.

## Composants

- Panneaux/cartes : fond `#222`, bordure 1 px halo `#1C1C1C`, rayon 10 px.
- Boutons : primaire bordure et texte cyan, secondaire bordure et texte gris.
- Badges format : surface `#222`, texte cyan, rayon 10 px.
- Courbe de mana : barres cyan, valeurs en vert terminal monospace.
- Toasts : bandeau `#222`, bordure gauche 4 px
  (magenta erreur / cyan info / vert terminal succès).
- Sélection : fond `#222`, liseré cyan.

## Slogan

Un slogan pochoir orne le header : « NO GODS NO MASTERS NO CORPORATIONS ».

## Ton éditorial (VOX POPULI)

Tutoiement, vocabulaire de manif. Microcopy :

- « Recruter » — ajouter une carte (`Deck::recruit`)
- « Sabotage » — supprimer (`Deck::sabotage`)
- « Lever le poing » — valider le deck (`Deck::validate`)
- « Ce deck rassemble 12 000 manifestants ? Non : 60 cartes. » — erreur de
  taille minimale

Rester cru, direct, drôle. Pas d'anglais marketing creux.
