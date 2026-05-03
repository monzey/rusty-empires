# Rusty Empires

Rusty Empires est un projet de jeu de strategie tactique au tour par tour, inspire par l'experience d'Age of Empires sur Nintendo DS.

L'objectif initial est de construire un moteur tactique en Rust avec Bevy, avant de viser un jeu complet. Le projet se concentre d'abord sur les fondations : carte en grille, gestion des unites, combats tactiques et civilisations jouables.

## Objectifs

- Developper un moteur de jeu tactique au tour par tour.
- Construire une carte en grille avec cases, terrains et placement d'unites.
- Implementer un systeme de combat lisible et extensible.
- Representer plusieurs civilisations avec leurs propres caracteristiques.
- Garder une architecture simple, testable et adaptee a l'ECS de Bevy.

## Stack technique

- Langage : Rust
- Moteur : Bevy
- Environnement de developpement : devenv / Nix
- Paradigme principal : ECS, via l'architecture de Bevy
- Plateforme cible initiale : Linux desktop
- Type de jeu : strategie tactique au tour par tour

## Fonctionnalites prevues

### Carte en grille

- Generation ou chargement d'une carte composee de tuiles.
- Types de terrain avec proprietes de gameplay.
- Coordonnees de grille pour le placement et le deplacement.

### Combat tactique

- Unites avec points de vie, attaque, defense et portee.
- Resolution de combat au tour par tour.
- Regles suffisamment modulaires pour supporter plusieurs types d'unites.

### Civilisations

- Donnees de civilisation separees de la logique moteur.
- Bonus ou variations possibles selon la civilisation.
- Base extensible pour ajouter unites, batiments et technologies.

## Etat du projet

Le projet est en phase de conception et de prototypage. Le depot servira d'abord a valider les choix techniques autour de Bevy, de l'ECS et des systemes tactiques principaux.

## Lancement

Entrer dans l'environnement de developpement :

```bash
devenv shell
```

Lancer le jeu :

```bash
cargo run
```

Commandes utiles fournies par devenv :

```bash
devenv shell check
devenv shell run
devenv shell fmt
devenv shell lint
```

## Roadmap initiale

1. Initialiser le projet Rust et Bevy.
2. Afficher une carte en grille minimale.
3. Ajouter une selection d'unite.
4. Ajouter le deplacement au tour par tour.
5. Ajouter une premiere resolution de combat.
6. Structurer les donnees de civilisations.

## Note

Ce projet est un jeu original inspire par les jeux de strategie tactique historiques. Il n'utilise pas les assets, noms, marques ou contenus proprietaires d'Age of Empires.
