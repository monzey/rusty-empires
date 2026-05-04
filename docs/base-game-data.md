# Donnees de base du jeu

Ce document decrit les donnees communes a toutes les factions de Rusty Empires.

Il est aligne avec `docs/SPEC.md` : le jeu utilise une economie reduite, une base de batiments lisible, des recherches lancees depuis l'universite et des unites de combat principalement recrutees a la caserne.

Les valeurs de statistiques restent volontairement hautes pour servir de base d'equilibrage cote design.

---

# Ressources

## Ressources principales

| Ressource | Role principal |
|---|---|
| Nourriture | Recrutement des villageois et de nombreuses unites organiques |
| Or | Construction, recrutement avance, echanges et certaines recherches |
| Points de technologie | Cout principal des recherches lancees a l'universite |

## Ressources tactiques speciales

Certaines factions peuvent utiliser une ressource secondaire temporaire liee a leur gameplay.

Exemple : les Necrarques peuvent utiliser les Cadavres comme ressource de champ de bataille.

Ces ressources ne remplacent pas l'economie principale et ne doivent pas devenir des ressources globales permanentes.

---

# Statistiques communes

| Statistique | Description |
|---|---|
| PV | Points de vie de l'unite ou du batiment |
| Attaque | Degats de base infliges lors d'une attaque |
| Defense | Reduction ou mitigation des degats recus |
| Portee | Distance minimale et maximale d'attaque |
| Mouvement | Nombre de cases que l'unite peut parcourir par tour |
| Vision | Rayon de detection autour de l'unite ou du batiment |
| Population | Place occupee par l'unite dans la limite de population |

Chaque unite doit posseder une capacite principale clairement identifiable.

---

# Unites communes

Les unites communes forment le socle de toutes les factions. Elles sont volontairement simples pour laisser les unites uniques porter l'identite des civilisations.

Toutes les unites de combat communes sont recrutees a la caserne.

## Villageois

Unite economique principale. Le villageois construit les batiments et permet d'etendre l'infrastructure.

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

Capacite speciale : Construction

Permet de construire un batiment sur une tuile valide respectant les contraintes d'adjacence et de ressource naturelle.

## Soldat

Unite de melee commune. Le Soldat sert d'unite de ligne simple et fiable.

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
| Nourriture | 55 |
| Or | 15 |

Produit par : Caserne

Capacite speciale : Garde

Si le Soldat ne s'est pas deplace pendant son tour, il gagne +1 defense jusqu'au debut de son prochain tour.

## Archer

Unite a distance commune. L'Archer est fragile mais utile pour soutenir une ligne de front.

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
| Nourriture | 30 |
| Or | 35 |

Produit par : Caserne

Capacite speciale : Tir de soutien

L'Archer gagne +1 attaque contre une cible adjacente a une unite alliee.

---

# Heros

Chaque faction possede un heros unique. Les heros ne sont pas communs, mais les regles suivantes s'appliquent a tous :

- un seul heros par faction ;
- le heros a des statistiques superieures a une unite standard ;
- le heros possede une capacite forte liee a l'identite de sa faction ;
- le heros doit renforcer l'armee sans rendre les autres unites inutiles.

Regle de recrutement recommandee : le heros est recrute au Forum apres la construction d'une Universite.

---

# Batiments communs

## Forum

Batiment principal de la faction. Il produit les villageois et sert de point de depart au developpement.

| Statistique | Valeur |
|---|---:|
| PV | 600 |
| Defense | 4 |
| Taille | 2x2 |
| Vision | 4 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 250 |
| Nourriture | 100 |

Fonctions :

- produit les villageois ;
- permet de recruter le heros lorsque les prerequis sont remplis ;
- autorise la construction d'autres batiments autour de lui ;
- peut servir de condition de defaite selon le mode de jeu.

## Mine d'or

Batiment economique construit sur un gisement d'or.

| Statistique | Valeur |
|---|---:|
| PV | 220 |
| Defense | 1 |
| Taille | 1x1 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 60 |

Production : +40 or a la fin du tour du proprietaire.

Contraintes :

- constructible uniquement sur une tuile contenant un gisement d'or.

## Ferme

Batiment economique construit sur un champ.

| Statistique | Valeur |
|---|---:|
| PV | 180 |
| Defense | 1 |
| Taille | 1x1 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 40 |

Production : +45 nourriture a la fin du tour du proprietaire.

Contraintes :

- constructible uniquement sur une tuile contenant un champ.

## Caserne

Batiment militaire commun. La Caserne recrute les unites de combat communes et certaines unites de faction debloquees.

| Statistique | Valeur |
|---|---:|
| PV | 350 |
| Defense | 2 |
| Taille | 2x2 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 180 |
| Nourriture | 80 |

Produit :

- Soldat ;
- Archer ;
- certaines unites de faction si elles sont debloquees par technologie.

## Marche

Batiment economique. Le Marche sert uniquement a l'echange de marchandises.

| Statistique | Valeur |
|---|---:|
| PV | 300 |
| Defense | 1 |
| Taille | 2x2 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 160 |
| Nourriture | 80 |

Fonctions :

- echanger de l'or contre de la nourriture ;
- echanger de la nourriture contre de l'or ;
- ameliorer la flexibilite economique.

Le Marche ne recrute pas de mercenaires dans la version actuelle de la specification.

## Universite

Batiment technologique. Toutes les recherches sont lancees depuis l'Universite.

| Statistique | Valeur |
|---|---:|
| PV | 320 |
| Defense | 1 |
| Taille | 2x2 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 220 |
| Nourriture | 120 |

Fonctions :

- produit +20 points de technologie a la fin du tour ;
- permet de lancer les recherches communes ;
- permet de lancer les recherches propres aux factions ;
- sert de prerequis recommande pour recruter le heros.

---

# Batiments defensifs communs

## Tour de guet

Batiment defensif donnant de la vision et une attaque a distance.

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
| Or | 140 |
| Nourriture | 40 |

Capacite speciale : Tir defensif

La Tour de guet peut attaquer une unite ennemie dans sa portee une fois par tour.

# Recherches communes

Toutes les recherches communes sont lancees depuis l'Universite.

## Agriculture

| Champ | Valeur |
|---|---|
| Cout | 80 points de technologie, 60 nourriture |
| Prerequis | Ferme |
| Effet | Les fermes produisent +15 nourriture par tour |

## Fiscalite

| Champ | Valeur |
|---|---|
| Cout | 100 points de technologie, 80 or |
| Prerequis | Marche |
| Effet | Les mines d'or produisent +10 or par tour |

## Discipline militaire

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 80 nourriture |
| Prerequis | Caserne |
| Effet | Les Soldats gagnent +1 attaque |

## Archerie organisee

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 80 or |
| Prerequis | Caserne |
| Effet | Les Archers gagnent +1 attaque |

## Armures renforcees

| Champ | Valeur |
|---|---|
| Cout | 160 points de technologie, 120 or |
| Prerequis | Caserne |
| Effet | Les unites de combat communes gagnent +1 defense |

## Fortifications

| Champ | Valeur |
|---|---|
| Cout | 180 points de technologie, 140 or |
| Prerequis | Tour de guet |
| Effet | Les Tours de guet gagnent +20% PV |

## Coordination tactique

| Champ | Valeur |
|---|---|
| Cout | 200 points de technologie, 120 nourriture, 120 or |
| Prerequis | Universite |
| Effet | Les heros reduisent de 1 tour le temps de recharge de leur capacite principale, jusqu'a un minimum de 1 tour |

---

# Relations de debloquage principales

```text
Forum
├── Villageois
├── Heros de faction ← Universite
├── Mine d'or ← Gisement d'or
├── Ferme ← Champ
├── Caserne
│   ├── Soldat
│   └── Archer
├── Marche
├── Universite
│   ├── Recherches communes
│   └── Recherches de faction
└── Tour de guet
```

---

# Notes d'equilibrage initiales

- Le Soldat doit rester simple et fiable, sans voler le role des unites uniques.
- L'Archer doit etre fort en soutien, mais vulnerable au contact.
- Le Villageois doit pouvoir se defendre faiblement, sans devenir une unite militaire.
- Les Tours doivent proteger une zone sans bloquer toute la partie.
- Le Marche doit donner de la flexibilite, mais pas remplacer une bonne economie.
- L'Universite doit etre un objectif important, car elle ouvre les recherches et les heros.
- Les factions doivent apporter la complexite principale du jeu via leurs unites, heros, batiments speciaux et technologies propres.
