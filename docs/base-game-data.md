# Donnees de base du jeu

Ce document decrit les premieres donnees de gameplay communes a toutes les civilisations de Rusty Empires.

L'objectif est de poser une base claire pour un jeu de strategie tactique au tour par tour : unites, batiments et recherches technologiques. Les valeurs proposees ici sont indicatives et pourront etre ajustees pendant les phases de prototypage et d'equilibrage.

## Principes generaux

Rusty Empires repose sur une carte en grille, des unites deplacees au tour par tour et des combats resolus de maniere lisible.

Chaque unite possede :

- des points de vie ;
- une attaque ;
- une defense ;
- une portee ;
- une valeur de mouvement ;
- une vision ;
- un cout en ressources ;
- un ou plusieurs traits de gameplay.

Chaque batiment peut servir a :

- produire des unites ;
- debloquer des recherches ;
- fournir de la population ;
- proteger une zone ;
- stocker ou convertir des ressources.

Chaque recherche sert a :

- debloquer de nouvelles unites ;
- ameliorer des statistiques ;
- specialiser certaines strategies ;
- preparer les futurs arbres technologiques de civilisation.

## Ressources de base

| Ressource | Role principal |
|---|---|
| Nourriture | Creation des villageois, unites organiques, cavalerie |
| Bois | Batiments, archers, machines simples |
| Pierre | Defenses, tours, murs, batiments solides |
| Or | Technologies, unites avancees, economie |
| Fer | Infanterie lourde, cavalerie, armes avancees |

## Statistiques communes

| Statistique | Description |
|---|---|
| PV | Points de vie de l'unite ou du batiment |
| Attaque | Degats de base infliges lors d'une attaque |
| Defense | Reduction ou mitigation des degats recus |
| Portee | Distance minimale et maximale d'attaque |
| Mouvement | Nombre de cases que l'unite peut parcourir par tour |
| Vision | Rayon de detection autour de l'unite |
| Population | Place occupee par l'unite dans la limite de population |

---

# Unites de base

## Villageois

Unite economique principale. Le villageois recolte les ressources, construit les batiments et peut reparer les structures.

| Statistique | Valeur |
|---|---:|
| PV | 25 |
| Attaque | 3 |
| Defense | 0 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 50 |

Produit par : Forum

Traits :

- recolte les ressources ;
- construit les batiments ;
- repare les batiments ;
- faible au combat.

## Milicien

Unite militaire tres basique. Peu couteuse, utile en debut de partie ou pour defendre rapidement une position.

| Statistique | Valeur |
|---|---:|
| PV | 35 |
| Attaque | 6 |
| Defense | 1 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 35 |
| Fer | 10 |

Produit par : Caserne

Traits :

- unite de transition ;
- efficace contre les villageois ;
- faible contre les unites specialisees.

## Lancier

Infanterie defensive specialisee contre la cavalerie.

| Statistique | Valeur |
|---|---:|
| PV | 45 |
| Attaque | 8 |
| Defense | 2 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 40 |
| Bois | 25 |

Produit par : Caserne

Traits :

- bonus contre cavalerie ;
- bon rapport cout/defense ;
- faible contre les archers.

## Epeiste

Infanterie de ligne polyvalente. Plus couteuse que le lancier mais plus efficace en combat direct.

| Statistique | Valeur |
|---|---:|
| PV | 60 |
| Attaque | 10 |
| Defense | 3 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 60 |
| Fer | 25 |

Produit par : Caserne

Requis : Travail du fer

Traits :

- bonne unite de front ;
- efficace contre les unites legeres ;
- sensible aux archers bien proteges.

## Archer

Unite a distance fragile mais capable d'attaquer sans s'exposer immediatement.

| Statistique | Valeur |
|---|---:|
| PV | 30 |
| Attaque | 7 |
| Defense | 0 |
| Portee | 2-4 |
| Mouvement | 3 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 45 |
| Or | 20 |

Produit par : Champ de tir

Traits :

- attaque a distance ;
- fort derriere une ligne defensive ;
- faible au corps-a-corps ;
- ne peut pas attaquer une cible adjacente si la portee minimale est conservee.

## Frondeur

Unite a distance economique, efficace contre les archers mais moins polyvalente.

| Statistique | Valeur |
|---|---:|
| PV | 28 |
| Attaque | 5 |
| Defense | 1 |
| Portee | 2-3 |
| Mouvement | 3 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 35 |
| Pierre | 15 |

Produit par : Champ de tir

Traits :

- bonus contre archers ;
- peu couteux ;
- degats faibles contre les unites lourdes.

## Cavalier eclaireur

Unite rapide servant a explorer la carte, harceler l'economie ennemie et capturer les zones neutres.

| Statistique | Valeur |
|---|---:|
| PV | 60 |
| Attaque | 6 |
| Defense | 1 |
| Portee | 1 |
| Mouvement | 5 |
| Vision | 6 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 70 |

Produit par : Ecurie

Traits :

- tres mobile ;
- grande vision ;
- bon pour l'exploration ;
- faible contre les lanciers.

## Cavalier lourd

Unite mobile et puissante, efficace pour contourner les lignes ennemies et atteindre les unites a distance.

| Statistique | Valeur |
|---|---:|
| PV | 85 |
| Attaque | 12 |
| Defense | 3 |
| Portee | 1 |
| Mouvement | 5 |
| Vision | 4 |
| Population | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 80 |
| Or | 45 |
| Fer | 25 |

Produit par : Ecurie

Requis : Elevage militaire

Traits :

- charge puissante ;
- efficace contre archers et unites isolees ;
- couteux ;
- vulnerable aux lanciers.

## Belier

Machine de siege lente, concue pour detruire les batiments et absorber les tirs.

| Statistique | Valeur |
|---|---:|
| PV | 140 |
| Attaque | 18 |
| Defense | 5 |
| Portee | 1 |
| Mouvement | 2 |
| Vision | 2 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 120 |
| Fer | 60 |
| Or | 30 |

Produit par : Atelier de siege

Requis : Ingenierie de siege

Traits :

- bonus massif contre batiments ;
- resistant aux tirs d'archers ;
- tres lent ;
- faible contre infanterie melee.

## Catapulte

Machine de siege a distance. Inflige des degats de zone, mais reste fragile si elle est atteinte.

| Statistique | Valeur |
|---|---:|
| PV | 80 |
| Attaque | 22 |
| Defense | 1 |
| Portee | 3-6 |
| Mouvement | 2 |
| Vision | 4 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 160 |
| Fer | 70 |
| Or | 60 |

Produit par : Atelier de siege

Requis : Balistique

Traits :

- attaque a distance ;
- degats de zone ;
- forte contre groupes d'unites ;
- fragile au corps-a-corps ;
- ne peut pas attaquer les cases adjacentes.

---

# Batiments de base

## Forum

Batiment principal de la civilisation. Produit les villageois, sert de point de depot et represente le centre de depart.

| Statistique | Valeur |
|---|---:|
| PV | 600 |
| Defense | 4 |
| Taille | 2x2 |
| Population fournie | 10 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 250 |
| Pierre | 100 |

Fonctions :

- produit les villageois ;
- sert de depot pour les ressources ;
- debloque les premiers batiments ;
- condition de defaite possible si tous les forums sont detruits.

## Maison

Augmente la limite de population.

| Statistique | Valeur |
|---|---:|
| PV | 120 |
| Defense | 1 |
| Taille | 1x1 |
| Population fournie | 5 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 30 |

Fonctions :

- augmente la population maximale ;
- batiment economique essentiel ;
- fragile.

## Camp de bucheron

Point de depot pour le bois. Peut ameliorer la recolte du bois via certaines recherches.

| Statistique | Valeur |
|---|---:|
| PV | 180 |
| Defense | 1 |
| Taille | 1x1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 60 |

Fonctions :

- depot de bois ;
- reduit les trajets des villageois ;
- peut debloquer les technologies de recolte du bois.

## Moulin

Point de depot pour la nourriture. Sert aux technologies agricoles.

| Statistique | Valeur |
|---|---:|
| PV | 180 |
| Defense | 1 |
| Taille | 1x1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 60 |

Fonctions :

- depot de nourriture ;
- ameliore l'economie alimentaire ;
- requis pour certaines technologies economiques.

## Carriere

Point de depot pour la pierre et le fer.

| Statistique | Valeur |
|---|---:|
| PV | 200 |
| Defense | 1 |
| Taille | 1x1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 70 |

Fonctions :

- depot de pierre ;
- depot de fer ;
- facilite l'expansion vers les ressources minerales.

## Caserne

Premier batiment militaire. Produit l'infanterie de base.

| Statistique | Valeur |
|---|---:|
| PV | 350 |
| Defense | 2 |
| Taille | 2x2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 160 |

Produit :

- Milicien ;
- Lancier ;
- Epeiste, apres Travail du fer.

Recherches possibles :

- Discipline militaire ;
- Travail du fer ;
- Formation de ligne.

## Champ de tir

Produit les unites a distance.

| Statistique | Valeur |
|---|---:|
| PV | 300 |
| Defense | 1 |
| Taille | 2x2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 150 |

Produit :

- Archer ;
- Frondeur.

Recherches possibles :

- Archerie ;
- Empennage ;
- Arcs composites.

## Ecurie

Produit les unites montees.

| Statistique | Valeur |
|---|---:|
| PV | 320 |
| Defense | 1 |
| Taille | 2x2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 175 |

Produit :

- Cavalier eclaireur ;
- Cavalier lourd, apres Elevage militaire.

Recherches possibles :

- Elevage militaire ;
- Selle renforcee ;
- Charge coordonnee.

## Forge

Batiment technologique militaire. Ameliore les armes et armures.

| Statistique | Valeur |
|---|---:|
| PV | 350 |
| Defense | 2 |
| Taille | 2x2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 150 |
| Pierre | 75 |

Recherches possibles :

- Travail du fer ;
- Armes d'acier ;
- Armures renforcees ;
- Balistique.

## Atelier de siege

Produit les machines de siege.

| Statistique | Valeur |
|---|---:|
| PV | 320 |
| Defense | 1 |
| Taille | 2x2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 200 |
| Fer | 80 |

Produit :

- Belier, apres Ingenierie de siege ;
- Catapulte, apres Balistique.

Requis : Forge

## Tour de guet

Batiment defensif donnant de la vision et pouvant attaquer les ennemis proches.

| Statistique | Valeur |
|---|---:|
| PV | 250 |
| Defense | 3 |
| Taille | 1x1 |
| Portee | 2-5 |
| Vision | 6 |

Cout :

| Ressource | Valeur |
|---|---:|
| Bois | 50 |
| Pierre | 125 |

Fonctions :

- detecte les ennemis ;
- attaque a distance ;
- protege les zones economiques ;
- faible contre les machines de siege.

## Mur

Structure defensive servant a bloquer ou canaliser les deplacements ennemis.

| Statistique | Valeur |
|---|---:|
| PV | 180 |
| Defense | 5 |
| Taille | 1x1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Pierre | 25 |

Fonctions :

- bloque le passage ;
- protege les points strategiques ;
- vulnerable aux beliers et aux technologies de siege.

---

# Recherches de base

## Agriculture

Ameliore l'economie alimentaire.

| Champ | Valeur |
|---|---|
| Batiment | Moulin |
| Cout | 100 nourriture, 50 or |
| Effet | +15% vitesse de recolte de nourriture |

## Coupe organisee

Ameliore l'exploitation du bois.

| Champ | Valeur |
|---|---|
| Batiment | Camp de bucheron |
| Cout | 100 bois, 50 or |
| Effet | +15% vitesse de recolte du bois |

## Extraction miniere

Ameliore la recolte de pierre et de fer.

| Champ | Valeur |
|---|---|
| Batiment | Carriere |
| Cout | 100 bois, 75 or |
| Effet | +15% vitesse de recolte de pierre et de fer |

## Discipline militaire

Rend l'infanterie plus fiable en combat.

| Champ | Valeur |
|---|---|
| Batiment | Caserne |
| Cout | 120 nourriture, 80 or |
| Effet | +1 mouvement pour les unites d'infanterie legeres et de ligne |

## Travail du fer

Debloque l'epeiste et prepare les ameliorations militaires avancees.

| Champ | Valeur |
|---|---|
| Batiment | Forge |
| Cout | 150 fer, 100 or |
| Effet | Debloque Epeiste, +1 attaque pour l'infanterie melee |

## Formation de ligne

Ameliore la tenue des unites d'infanterie en groupe.

| Champ | Valeur |
|---|---|
| Batiment | Caserne |
| Cout | 180 nourriture, 120 or |
| Prerequis | Travail du fer |
| Effet | +1 defense pour Milicien, Lancier et Epeiste lorsqu'ils sont adjacents a une unite alliee d'infanterie |

## Archerie

Standardise l'entrainement des archers.

| Champ | Valeur |
|---|---|
| Batiment | Champ de tir |
| Cout | 100 bois, 75 or |
| Effet | +1 attaque pour les unites a distance |

## Empennage

Ameliore la precision des projectiles.

| Champ | Valeur |
|---|---|
| Batiment | Champ de tir |
| Cout | 150 bois, 100 or |
| Prerequis | Archerie |
| Effet | +1 portee maximale pour Archer |

## Arcs composites

Augmente la puissance des arcs avances.

| Champ | Valeur |
|---|---|
| Batiment | Champ de tir |
| Cout | 220 bois, 150 or, 80 fer |
| Prerequis | Empennage |
| Effet | +2 attaque pour Archer, mais +10 or au cout de creation |

## Elevage militaire

Permet de former des unites de cavalerie plus lourdes.

| Champ | Valeur |
|---|---|
| Batiment | Ecurie |
| Cout | 180 nourriture, 120 or |
| Effet | Debloque Cavalier lourd |

## Selle renforcee

Ameliore la resistance de la cavalerie.

| Champ | Valeur |
|---|---|
| Batiment | Ecurie |
| Cout | 200 nourriture, 150 or, 80 fer |
| Prerequis | Elevage militaire |
| Effet | +1 defense pour les unites de cavalerie |

## Charge coordonnee

Ameliore l'impact offensif de la cavalerie.

| Champ | Valeur |
|---|---|
| Batiment | Ecurie |
| Cout | 250 nourriture, 180 or |
| Prerequis | Selle renforcee |
| Effet | Les cavaliers infligent +20% de degats lors de leur premiere attaque apres un deplacement d'au moins 3 cases |

## Armes d'acier

Augmente les degats des unites de melee.

| Champ | Valeur |
|---|---|
| Batiment | Forge |
| Cout | 220 fer, 180 or |
| Prerequis | Travail du fer |
| Effet | +2 attaque pour infanterie melee et cavalerie melee |

## Armures renforcees

Ameliore la survie des unites de front.

| Champ | Valeur |
|---|---|
| Batiment | Forge |
| Cout | 220 fer, 150 or |
| Prerequis | Travail du fer |
| Effet | +2 defense pour infanterie melee et cavalerie melee |

## Ingenierie de siege

Debloque les premieres machines de siege.

| Champ | Valeur |
|---|---|
| Batiment | Forge |
| Cout | 200 bois, 150 fer, 150 or |
| Prerequis | Travail du fer |
| Effet | Debloque Atelier de siege et Belier |

## Balistique

Ameliore les tirs complexes et debloque la catapulte.

| Champ | Valeur |
|---|---|
| Batiment | Forge |
| Cout | 250 bois, 180 fer, 200 or |
| Prerequis | Ingenierie de siege, Empennage |
| Effet | Debloque Catapulte, +10% precision des attaques a distance |

## Fortifications

Renforce les defenses statiques.

| Champ | Valeur |
|---|---|
| Batiment | Forum |
| Cout | 250 pierre, 150 or |
| Prerequis | Travail du fer |
| Effet | +25% PV pour Mur et Tour de guet |

---

# Relations de debloquage principales

```text
Forum
├── Villageois
├── Maison
├── Camp de bucheron
├── Moulin
├── Carriere
├── Caserne
│   ├── Milicien
│   ├── Lancier
│   └── Epeiste ← Travail du fer
├── Champ de tir
│   ├── Archer
│   └── Frondeur
├── Ecurie
│   ├── Cavalier eclaireur
│   └── Cavalier lourd ← Elevage militaire
├── Forge
│   ├── Travail du fer
│   ├── Armes d'acier
│   ├── Armures renforcees
│   ├── Ingenierie de siege
│   └── Balistique
└── Atelier de siege ← Ingenierie de siege
    ├── Belier
    └── Catapulte ← Balistique
```

---

# Notes d'equilibrage initiales

- Le lancier doit rester rentable contre la cavalerie, meme avec un cout faible.
- L'archer doit etre fort en position protegee, mais vulnerable au contact.
- La cavalerie doit dominer la mobilite, sans pouvoir traverser gratuitement une ligne de lanciers.
- Les machines de siege doivent etre decisives contre les batiments, mais necessiter une escorte.
- Les recherches doivent creer des choix strategiques clairs, pas seulement des bonus automatiques.
- Les civilisations pourront modifier ces bases avec des unites uniques, des bonus et des technologies propres.
