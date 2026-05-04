# Clans Kharzun

Les Clans Kharzun sont une civilisation montagnarde, forgeuse et obstinee. Ils excellent dans la defense, les combats d'usure, le controle de passages et la destruction des positions fortifiees.

Cette faction doit offrir un gameplay plus lent que le Royaume Valdorien, mais plus robuste. Les Kharzun avancent peu, mais quand ils prennent une position, ils deviennent tres difficiles a deloger.

Ce document est aligne avec `docs/SPEC.md` : les ressources utilisees sont la nourriture, l'or et les points de technologie ; les recherches sont lancees depuis l'Universite ; le Marche sert uniquement a l'echange ; les unites de combat communes sont recrutees a la Caserne ; les unites speciales Kharzun utilisent leurs batiments speciaux lorsque precise.

---

# Resume

| Element | Valeur |
|---|---|
| Nom | Clans Kharzun |
| Archetype | Clans montagnards forgeurs |
| Style de jeu | Defense, endurance, controle de zone, siege |
| Rythme | Lent |
| Difficulte | Moyenne |
| Force principale | Resistance, fortifications, machines lourdes |
| Faiblesse principale | Mobilite faible et reaction lente |

---

# Identite de gameplay

Les Kharzun gagnent en controlant les points strategiques, en fortifiant leurs positions et en forcant l'adversaire a venir mourir contre leurs lignes defensives.

Leur gameplay repose sur trois piliers :

- des unites tres resistantes ;
- une excellente tenue de position ;
- une capacite superieure a briser les defenses ennemies.

Dans la nouvelle economie de Rusty Empires, leur theme de pierre, de fer et de forge n'est pas represente par des ressources separees. Il est represente par leurs bonus de batiments, leurs technologies, leurs unites lourdes et leurs couts eleves en or.

Ils sont faibles quand ils doivent reagir vite a plusieurs attaques dispersees. Leur armee est puissante, mais lente, chere et parfois previsible.

## Forces

- Infanterie lourde tres solide.
- Bonus sur les collines, montagnes et positions fortifiees.
- Batiments plus resistants.
- Machines de siege puissantes.
- Excellent controle de zone.

## Faiblesses

- Faible mobilite generale.
- Peu d'options de harcelement rapide.
- Peu d'outils d'exploration.
- Unites speciales couteuses en or.
- Vulnerable au contournement et aux attaques dispersees.

---

# Bonus et malus de civilisation

## Bonus de civilisation

| Bonus | Effet |
|---|---|
| Maitres de la pierre | Les batiments Kharzun ont +15% PV |
| Tenacite des clans | Les unites Kharzun gagnent +1 defense lorsqu'elles ne se sont pas deplacees ce tour-ci |
| Siege ancestral | Les unites de siege Kharzun infligent +20% de degats aux batiments |
| Montagnards | Les unites Kharzun ignorent le malus de mouvement leger des collines |
| Ouvrages profonds | Les Tours, Murs et Bastions Kharzun coutent 10% d'or en moins |

## Malus de civilisation

| Malus | Effet |
|---|---|
| Marche lente | Les unites militaires terrestres ont -1 mouvement si leur mouvement de base est superieur a 3 |
| Peu de cavalerie | Les unites rapides et montees coutent 20% de nourriture et d'or en plus si elles sont ajoutees au roster |
| Commerce limite | Les echanges au Marche sont 10% moins efficaces |
| Reactions lentes | Les unites Kharzun ne peuvent pas beneficier d'un mouvement bonus apres attaque |

## Capacite passive : Position inebranlable

Lorsqu'une unite Kharzun commence et termine son tour sans se deplacer, elle gagne jusqu'au debut de son prochain tour :

- +1 defense ;
- +10% de resistance aux degats a distance ;
- immunite aux effets de recul simples.

Cette capacite encourage le joueur a verrouiller une position et a forcer l'adversaire a engager le combat dans de mauvaises conditions.

---

# Heros de faction

## Brokkar Main-de-Fer

Brokkar est le heros des Clans Kharzun. C'est un seigneur-forgeron, chef de guerre et gardien des serments anciens.

Il est concu pour tenir une ligne, proteger une position et transformer un point de passage en mur infranchissable. Il est tres fort lorsqu'il est bien place, mais il reagit mal aux menaces multiples et rapides.

| Statistique | Valeur |
|---|---:|
| PV | 130 |
| Attaque | 13 |
| Defense | 7 |
| Portee | 1 |
| Mouvement | 2 |
| Vision | 3 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 120 |
| Or | 200 |

Recrute par : Forum

Prerequis : Universite

Traits :

- heros ;
- infanterie lourde ;
- defense ;
- commandement de siege.

Capacite speciale : Ancrage du clan

Une fois tous les 3 tours, Brokkar peut ancrer une position dans un rayon de 2 cases.

Effet pendant 1 tour :

- les unites alliees dans la zone gagnent +2 defense si elles ne se deplacent pas ;
- les Murs, Tours et Bastions dans la zone gagnent +1 defense ;
- les unites affectees ne peuvent pas etre repoussees par des effets de recul simples.

Role tactique :

- verrouiller un choke point ;
- proteger les machines de siege ;
- absorber une attaque decisive ;
- forcer l'adversaire a contourner plutot qu'a attaquer frontalement.

---

# Unites uniques

## Brise-bouclier Kharzun

Le Brise-bouclier est une infanterie lourde offensive specialisee dans la destruction des lignes defensives ennemies.

| Statistique | Valeur |
|---|---:|
| PV | 75 |
| Attaque | 12 |
| Defense | 4 |
| Portee | 1 |
| Mouvement | 2 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 70 |
| Or | 65 |

Recrute par : Caserne

Prerequis : Clans de la forge

Traits :

- infanterie lourde ;
- anti-defense ;
- lent ;
- tres efficace contre les unites en formation.

Capacite speciale : Fracasse-garde

Lorsqu'il attaque une unite ayant au moins 3 points de defense, le Brise-bouclier ignore 1 point de defense et applique un malus de -1 defense a la cible jusqu'a la fin du tour.

## Garde runique

Unite defensive d'elite. La Garde runique est concue pour tenir les passages et proteger les machines de siege.

| Statistique | Valeur |
|---|---:|
| PV | 90 |
| Attaque | 9 |
| Defense | 6 |
| Portee | 1 |
| Mouvement | 2 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 75 |
| Or | 90 |

Recrute par : Forge de clan

Prerequis : Serments runiques

Traits :

- infanterie lourde defensive ;
- excellente en choke point ;
- tres lente ;
- faible contre les degats de zone repetes.

Capacite speciale : Serment de pierre

Si la Garde runique est sur une colline, une montagne, une case adjacente a un Mur, une Tour, un Bastion ou un autre batiment allie, elle gagne +1 defense supplementaire.

## Belier de fer

Machine de siege unique. Plus lente, plus couteuse, mais beaucoup plus resistante qu'une machine commune.

| Statistique | Valeur |
|---|---:|
| PV | 190 |
| Attaque | 22 |
| Defense | 7 |
| Portee | 1 |
| Mouvement | 1 |
| Vision | 2 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 60 |
| Or | 170 |

Recrute par : Forge de clan

Prerequis : Fonderies de siege

Traits :

- siege lourd ;
- tres resistant ;
- bonus massif contre batiments ;
- extremement lent.

Capacite speciale : Blindage frontal

Les attaques a distance venant d'une unite situee dans l'arc frontal du Belier de fer infligent 30% de degats en moins.

## Mortier des profondeurs

Machine de siege a distance. Elle tire lentement, mais peut briser des groupes compacts et des defenses statiques.

| Statistique | Valeur |
|---|---:|
| PV | 95 |
| Attaque | 26 |
| Defense | 2 |
| Portee | 3-6 |
| Mouvement | 1 |
| Vision | 4 |
| Population | 4 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 70 |
| Or | 220 |

Recrute par : Forge de clan

Prerequis : Poudre noire des mines

Traits :

- siege a distance ;
- degats de zone ;
- forte contre fortifications ;
- tres faible si engage au corps-a-corps.

Capacite speciale : Tir de demolition

Inflige +30% de degats aux Murs, Tours, Bastions et batiments defensifs. Les unites adjacentes a la cible subissent 40% des degats initiaux.

---

# Batiments specifiques

## Forge de clan

Batiment militaire special propre aux Kharzun. Il represente leurs traditions de forge, d'armure et de siege.

| Statistique | Valeur |
|---|---:|
| PV | 450 |
| Defense | 3 |
| Taille | 2x2 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 90 |
| Or | 240 |

Prerequis : Forum

Produit :

- Garde runique ;
- Belier de fer ;
- Mortier des profondeurs.

Effets :

- les unites Kharzun produites dans une Caserne adjacente gagnent +1 experience si un systeme d'experience est implemente ;
- les unites de siege Kharzun adjacentes a la Forge de clan gagnent +1 defense ;
- compte comme batiment special pour les prerequis de technologies Kharzun.

Note de recrutement :

- Le Brise-bouclier Kharzun est recrute a la Caserne commune apres Clans de la forge.
- Les unites speciales lourdes et de siege sont recrutees a la Forge de clan.

## Bastion de pierre

Batiment defensif unique. Il sert a verrouiller des passages et renforcer les lignes Kharzun.

| Statistique | Valeur |
|---|---:|
| PV | 520 |
| Defense | 6 |
| Taille | 2x2 |
| Portee | 2-4 |
| Vision | 5 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 260 |
| Nourriture | 80 |

Prerequis : Architecture de montagne

Fonctions :

- attaque a distance moderee ;
- bloque fortement les passages ;
- donne +1 defense aux unites alliees adjacentes ;
- vulnerable aux machines de siege lourdes.

Limite :

- ne peut pas etre construit a moins de 5 cases d'un autre Bastion de pierre.

---

# Technologies specifiques

Toutes les recherches Kharzun sont lancees depuis l'Universite. Les batiments indiques sont des prerequis, pas les lieux de recherche.

## Mines profondes

Les Kharzun optimisent leurs infrastructures autour des ressources essentielles.

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 100 or |
| Prerequis | Mine d'or ou Ferme |
| Effet | Les Mines d'or et Fermes adjacentes a un batiment Kharzun produisent +10 ressources par tour |

## Architecture de montagne

Renforce les constructions Kharzun avec des techniques ancestrales.

| Champ | Valeur |
|---|---|
| Cout | 160 points de technologie, 140 or |
| Prerequis | Mines profondes, Forge de clan |
| Effet | +15% PV pour les batiments Kharzun, debloque Bastion de pierre |

## Clans de la forge

Organise les guerriers-forgerons en compagnies militaires.

| Champ | Valeur |
|---|---|
| Cout | 180 points de technologie, 140 nourriture, 140 or |
| Prerequis | Caserne, Discipline militaire |
| Effet | Debloque Brise-bouclier Kharzun et Forge de clan |

## Armures gravees

Renforce les armures lourdes avec des plaques gravees et ajustees a chaque clan.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 200 or |
| Prerequis | Clans de la forge, Forge de clan |
| Effet | +1 defense pour les unites d'infanterie lourde Kharzun |

## Serments runiques

Les guerriers jurent de tenir leur position jusqu'a la mort.

| Champ | Valeur |
|---|---|
| Cout | 240 points de technologie, 160 nourriture, 220 or |
| Prerequis | Armures gravees, Architecture de montagne |
| Effet | Debloque Garde runique, ameliore Position inebranlable avec +5% resistance supplementaire |

## Fonderies de siege

Les Kharzun developpent des ateliers capables de produire des machines de siege lourdement blindees.

| Champ | Valeur |
|---|---|
| Cout | 260 points de technologie, 260 or |
| Prerequis | Forge de clan, Clans de la forge |
| Effet | Debloque Belier de fer, +10% PV pour les unites de siege Kharzun |

## Poudre noire des mines

Une technologie dangereuse issue des profondeurs. Elle permet aux Kharzun de briser les fortifications les plus solides.

| Champ | Valeur |
|---|---|
| Cout | 320 points de technologie, 320 or |
| Prerequis | Fonderies de siege, Fortifications |
| Effet | Debloque Mortier des profondeurs |

## Chants des enclumes

Les forges rythment la marche des armees Kharzun et renforcent leur discipline.

| Champ | Valeur |
|---|---|
| Cout | 280 points de technologie, 180 nourriture, 240 or |
| Prerequis | Serments runiques |
| Effet | Les unites Kharzun adjacentes a une unite de siege alliee gagnent +1 attaque |

## Forteresses vivantes

Les positions defensives Kharzun deviennent presque impossibles a prendre sans siege lourd.

| Champ | Valeur |
|---|---|
| Cout | 340 points de technologie, 300 or |
| Prerequis | Architecture de montagne, Serments runiques, Bastion de pierre |
| Effet | Les Murs, Tours de guet et Bastions de pierre Kharzun gagnent +20% PV et +1 defense |

---

# Arbre technologique Kharzun

```text
Universite
├── Mines profondes
│   └── Architecture de montagne ← Forge de clan
│       ├── Bastion de pierre
│       └── Forteresses vivantes
│
├── Clans de la forge ← Caserne + Discipline militaire
│   ├── Brise-bouclier Kharzun
│   ├── Forge de clan
│   ├── Armures gravees
│   │   └── Serments runiques
│   │       ├── Garde runique
│   │       ├── Chants des enclumes
│   │       └── Forteresses vivantes
│   └── Fonderies de siege
│       └── Belier de fer
│           └── Poudre noire des mines
│               └── Mortier des profondeurs
│
└── Poudre noire des mines ← Fonderies de siege + Fortifications
```

---

# Plan de jeu recommande

## Debut de partie

Les Kharzun doivent securiser rapidement leurs ressources proches et poser une base difficile a attaquer. Leur debut de partie n'est pas explosif, mais il devient tres solide si leurs positions economiques sont protegees.

Objectifs :

- construire une Mine d'or ou une Ferme rapidement ;
- poser une Caserne ;
- rechercher Clans de la forge ;
- produire des Soldats pour tenir les points d'acces ;
- eviter de disperser les unites ;
- utiliser le terrain pour ralentir l'adversaire.

## Milieu de partie

Les Kharzun commencent a devenir dangereux avec la Forge de clan et Architecture de montagne.

Objectifs :

- produire des Brise-boucliers pour casser les lignes ennemies ;
- construire un Bastion de pierre sur un point strategique ;
- rechercher Armures gravees ;
- preparer Fonderies de siege ;
- forcer l'adversaire a attaquer une position fortifiee.

## Fin de partie

Les Kharzun cherchent a avancer lentement avec une armee compacte soutenue par des machines de siege.

Objectifs :

- proteger les Beliers de fer avec Gardes runiques ;
- utiliser les Mortiers des profondeurs contre les groupes et defenses ;
- verrouiller les points de passage avec Bastions ;
- utiliser Brokkar pour tenir les combats decisifs ;
- progresser case par case plutot que chercher une attaque rapide.

---

# Contres et vulnerabilites

## Fort contre

| Cible | Raison |
|---|---|
| Infanterie melee | Les Kharzun encaissent mieux les combats prolonges |
| Defenses statiques | Leur siege est tres puissant contre les batiments |
| Armees frontales | Leur positionnement defensif les avantage |
| Cartes avec choke points | Les Bastions et Gardes runiques verrouillent les passages |

## Faible contre

| Menace | Raison |
|---|---|
| Harcelement mobile | Leur armee reagit lentement |
| Archers mobiles | Ils peuvent etre kites si mal proteges |
| Degats de zone repetes | Leurs formations compactes y sont vulnerables |
| Controle de carte rapide | Les Kharzun prennent du temps a s'etendre |

---

# Notes d'equilibrage

- Les Kharzun doivent etre excellents quand ils tiennent une position, mais mediocres quand ils doivent courir apres l'ennemi.
- Brokkar doit rendre une position tres difficile a prendre, mais ne doit pas permettre de couvrir toute la carte.
- Le Brise-bouclier ne doit pas remplacer toutes les unites melee : il doit etre fort contre les cibles defensives, pas universel.
- La Garde runique doit etre impressionnante defensivement, mais punissable par le siege et le contournement.
- Les machines de siege Kharzun doivent etre plus fortes que les machines communes, mais beaucoup plus lentes.
- Le Bastion de pierre doit etre un outil de controle de carte, pas une condition de victoire automatique.
- La faction doit donner une sensation de puissance lourde, pas d'invincibilite.

---

# Direction artistique

## Themes visuels

- pierre sombre ;
- metal grave ;
- runes lumineuses discretes ;
- forges souterraines ;
- marteaux, enclumes et boucliers massifs ;
- silhouettes trapues et lourdement armees ;
- forteresses angulaires taillees dans la roche.

## Ambiance

Les Kharzun doivent evoquer une civilisation ancienne, rude et fiere. Ils ne sont pas barbares : ils sont artisans, soldats, mineurs et architectes. Chaque unite doit donner l'impression d'etre construite pour durer.

## Mots-cles

- pierre ;
- forge ;
- clan ;
- serment ;
- endurance ;
- siege ;
- montagne ;
- bastion ;
- enclume ;
- resistance.
