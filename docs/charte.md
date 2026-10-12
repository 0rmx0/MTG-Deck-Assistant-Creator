# Charte graphique — Hellfire Deckbuilder

Identité : punk underground rebelle, gravure/scratchboard, clair-obscur
dramatique, masque cousu comme emblème.

Les tokens couleur sont encodés dans `crates/hellfire-gui/src/theme.rs`
(constantes `Theme::*`, valeurs RGB identiques aux hex de la charte).

## Territoire visuel

- Gravure/scratchboard au trait, hachures denses, grain charbon.
- Clair-obscur fort ; les éléments percent la pénombre par accents néon.
- Interdits : dégradés SaaS, ombres douces, coins très arrondis, pastels.

## Palette (règle 70/20/10)

| Rôle              | Nom          | Hex       | Constante           |
|-------------------|--------------|-----------|--------------------|
| Fond principal    | Noir bitume  | `#0A0A0A` | `BG_PRIMARY`       |
| Fond secondaire   | Charbon      | `#1A1714` | `BG_SECONDARY`     |
| Élévation         | Brun sombre  | `#3C2818` | `BG_ELEVATED`      |
| Texte principal   | Blanc craie  | `#F2EFE9` | `TEXT_PRIMARY`     |
| Texte secondaire | Gris os      | `#9A938A` | `TEXT_SECONDARY`   |
| Accent 1 (CTA)    | Corail rebelle | `#FF4E3A` | `CORAL`          |
| Accent 2 (liens)  | Cyan insurgé  | `#00E5E5` | `CYAN`            |
| Accent 3 (stats)  | Orange fournaise | `#FF8C00` | `EMBER`         |

~70 % noirs/bruns, ~20 % blancs/gris, ~10 % néon (corail dominant).
Jamais plus d'un accent néon par zone. Jamais de corail sur cyan.

## Couleurs MTG (adaptées sombres)

Blanc `#EDE5D0`, Bleu `#2E6FA3`, Noir `#4B3B57`, Rouge `#C43A2F`,
Vert `#3F7A4A`, Artefact `#8C8577`, Multicolore = dégradé corail→cyan.

## Typographie

- Titres/logo : Pirata One / Special Elite, MAJUSCULES, letter-spacing ≥ 0.08em.
- Corps/UI : Barlow Condensed.
- Chiffres/stats : IBM Plex Mono.

## Composants

- Cartes de deck : fond `#1A1714`, bordure 1 px `#3C2818`, coins 2 px.
  Hover : bordure corail, décalage −2 px (effet pochoir).
- Badge format : rectangle noir, texte cyan, coin biseauté.
- Boutons : primaire corail/texte noir, secondaire bordure gris os.
  Jamais de dégradé ni d'ombre portée.
- Courbe de mana : barres corail (créatures) / cyan (sorts) / orange (lands).
- Toasts : bandeau noir, bordure gauche 4 px (corail erreur / cyan info /
  orange succès).

## Motion

Transitions sèches 120–180 ms, `steps(3)` ou linéaire. Apparition des
cartes : fade 150 ms + offset 4 px. Aucun rebond, aucune parallaxe.

## Ton éditorial (VOX POPULI)

Tutoiement, vocabulaire de manif. Microcopy :

- « Recruter » — ajouter une carte (`Deck::recruit`)
- « Sabotage » — supprimer (`Deck::sabotage`)
- « Lever le poing » — valider le deck (`Deck::validate`)
- « Ce deck rassemble 12 000 manifestants ? Non : 60 cartes. » — erreur de
  taille minimale

Rester cru, direct, drôle. Pas d'anglais marketing creux.
