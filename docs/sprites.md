# Integration des sprites isometriques

Ce document decrit le contrat entre la logique de grille et le rendu des sprites.

## Principe

La grille ne depend jamais de la taille reelle des images.

Chaque tuile possede une position logique unique : le centre du losange superieur. Cette position sert pour le gameplay, le picking souris, le placement des unites, le placement des batiments et l'ordre de dessin.

Le sprite PNG est ensuite dessine autour de cette position avec un scale et un offset de rendu.

## Taille logique de grille

Les dimensions logiques d'une tuile correspondent uniquement au losange superieur jouable :

```rust
TILE_WIDTH = 128.0
TILE_HEIGHT = 64.0
```

Conversion grille vers monde :

```text
world_x = (x - y) * TILE_WIDTH / 2
world_y = -(x + y) * TILE_HEIGHT / 2
```

Cette conversion ignore la taille du PNG, le padding, l'ombre et l'epaisseur du bloc.

## Taille reelle du PNG

Le fichier `tile.png` peut contenir :

- du padding transparent
- une ombre
- une epaisseur de bloc
- des pixels decoratifs hors du losange jouable

Dans le code, cette taille est documentee separement :

```rust
TILE_PNG_WIDTH = 512.0
TILE_PNG_HEIGHT = 512.0
```

Ces valeurs servent seulement au rendu du sprite. Elles ne doivent pas modifier l'espacement de grille.

## Scale du sprite

Le sprite est reduit via :

```rust
TILE_SPRITE_SCALE = 0.295
```

Objectif : faire correspondre la largeur visible du losange superieur du PNG a la largeur logique `TILE_WIDTH`.

Si les tiles ont des gaps, augmenter legerement `TILE_SPRITE_SCALE`.

Si les tiles se chevauchent trop, diminuer legerement `TILE_SPRITE_SCALE`.

## Offset vertical du sprite

Le PNG est centre par Bevy sur sa transform. Comme le centre du PNG n'est pas le centre du losange superieur, un offset de rendu est applique :

```rust
TILE_SPRITE_Y_OFFSET = -20.0
```

Objectif : placer le centre du losange superieur sur la position logique de la tuile.

Cet offset ne change pas la grille. Il change seulement ou l'image est dessinee.

## Placement des unites

Les unites ne sont pas placees au centre du PNG de tuile.

Elles sont placees sur la position logique de grille, puis ajustees avec un offset dedie :

```rust
unit_position = grid_to_world(position, z) + UNIT_Y_OFFSET
```

`UNIT_Y_OFFSET` sert a poser visuellement les pieds de l'unite sur le losange superieur.

Changer `TILE_SPRITE_SCALE` ou `TILE_SPRITE_Y_OFFSET` ne doit pas casser le placement logique des unites.

## Regle importante

Ne jamais recalculer la grille a partir de la taille reelle de l'image.

La taille PNG, le scale du sprite et l'offset vertical sont des parametres de rendu. `TILE_WIDTH` et `TILE_HEIGHT` sont des parametres de gameplay.

## Code concerne

- `src/app/constants.rs` : constantes de grille, PNG, scale et offsets.
- `src/app/grid.rs` : conversion grille/monde et monde/grille.
- `src/app/map_view.rs` : rendu des sprites de tiles.
- `src/app/setup.rs` : spawn initial des unites et batiments.
- `src/app/sync.rs` : mise a jour du placement des unites apres mouvement.
