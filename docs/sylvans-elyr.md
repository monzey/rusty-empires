# Sylvains d'Elyr

Les Sylvains d'Elyr sont une civilisation forestiere, mobile et insaisissable. Ils excellent dans l'exploration, le controle de carte, les attaques a distance et les embuscades.

Cette faction doit proposer un gameplay tres different du Royaume Valdorien et des Clans Kharzun. Les Elyr ne cherchent pas a tenir une ligne compacte ou a construire des positions imprenables. Ils gagnent en choisissant leurs combats, en frappant les points faibles et en disparaissant avant la riposte.

Ce document est aligne avec `docs/SPEC.md` : les ressources utilisees sont la nourriture, l'or et les points de technologie ; les recherches sont lancees depuis l'Universite ; le Marche sert uniquement a l'echange ; les unites de combat communes sont recrutees a la Caserne ; les unites speciales Elyr utilisent leurs batiments speciaux lorsque precise.

---

# Resume

| Element | Valeur |
|---|---|
| Nom | Sylvains d'Elyr |
| Archetype | Peuple forestier agile |
| Style de jeu | Mobilite, archers, vision, embuscades |
| Rythme | Rapide |
| Difficulte | Moyenne a elevee |
| Force principale | Controle de carte, harcelement et repositionnement |
| Faiblesse principale | Faible en combat frontal prolonge |

---

# Identite de gameplay

Les Elyr gagnent en evitant les affrontements defavorables. Leur armee est fragile si elle est coincee, mais tres dangereuse lorsqu'elle peut se repositionner.

Leur gameplay repose sur trois piliers :

- une excellente vision ;
- une mobilite superieure ;
- des attaques a distance et embuscades.

Dans la nouvelle economie de Rusty Empires, leur theme forestier n'est pas represente par une ressource bois. Il est represente par les terrains de foret, leurs bonus de deplacement, leurs batiments speciaux, leurs technologies et leurs unites mobiles.

Ils sont forts sur les cartes ouvertes, boisees ou avec plusieurs chemins d'acces. Ils sont plus faibles lorsqu'ils doivent attaquer une forteresse, tenir un point etroit ou subir une bataille frontale.

## Forces

- Tres bonne exploration.
- Archers puissants et mobiles.
- Bonus en foret.
- Capacite a harceler l'economie ennemie.
- Excellente detection des mouvements adverses.

## Faiblesses

- Infanterie fragile.
- Siege limite.
- Batiments moins resistants.
- Difficile a jouer si l'armee est encerclee.
- Moins efficace sur les cartes sans vegetation ou avec peu de chemins alternatifs.

---

# Bonus et malus de civilisation

## Bonus de civilisation

| Bonus | Effet |
|---|---|
| Enfants de la foret | Les unites Elyr ignorent le malus de mouvement des forets legeres |
| Oeil des clairieres | Les eclaireurs et unites a distance Elyr gagnent +1 vision |
| Tir mobile | Les unites a distance Elyr peuvent se deplacer de 1 case apres avoir attaque, si elles ne sont pas adjacentes a un ennemi |
| Economie sylvestre | Les Fermes adjacentes a une foret produisent +10 nourriture par tour |
| Embuscade naturelle | Les unites Elyr gagnent +1 attaque lorsqu'elles attaquent depuis une foret |

## Malus de civilisation

| Malus | Effet |
|---|---|
| Structures legeres | Les batiments Elyr ont -10% PV |
| Siege limite | Les unites de siege coutent 20% d'or en plus si elles sont ajoutees au roster |
| Front fragile | L'infanterie melee Elyr a -1 defense hors foret |
| Peu d'armure lourde | Les technologies defensives coutent 15% d'or en plus |

## Capacite passive : Frappe et repli

Lorsqu'une unite Elyr attaque une cible puis termine son tour sur une case non adjacente a une unite ennemie, elle gagne jusqu'au debut de son prochain tour :

- +1 defense contre les attaques a distance ;
- +1 vision ;
- +10% d'esquive contre la premiere attaque recue.

Cette capacite encourage le joueur a attaquer, se repositionner et eviter les engagements prolonges.

---

# Heros de faction

## Sylwen Oeil-des-Bois

Sylwen est l'heroine des Sylvains d'Elyr. Elle incarne la vision, la chasse et la maitrise du terrain.

Elle n'est pas concue pour tenir une ligne de front. Sa puissance vient de sa capacite a reveler les mouvements ennemis, marquer les cibles importantes et permettre aux unites Elyr de se repositionner.

| Statistique | Valeur |
|---|---:|
| PV | 80 |
| Attaque | 12 |
| Defense | 2 |
| Portee | 2-4 |
| Mouvement | 4 |
| Vision | 7 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 100 |
| Or | 180 |

Recrute par : Forum

Prerequis : Universite

Traits :

- heros ;
- attaque a distance ;
- vision ;
- mobilite ;
- embuscade.

Capacite speciale : Repli rapide

Une fois tous les 3 tours, Sylwen donne un ordre de repli tactique a toutes les unites alliees dans un rayon de 2 cases.

Effet pendant 1 tour :

- les unites affectees peuvent se deplacer de 1 case apres avoir attaque ;
- les unites deja capables de se deplacer apres attaque gagnent +1 case de repli a la place ;
- les unites affectees gagnent +1 vision jusqu'au debut du prochain tour Elyr.

Role tactique :

- ouvrir une fenetre de hit and run ;
- sauver des archers exposes ;
- reveler et punir une cible cle ;
- permettre aux Elyr de choisir le rythme du combat.

---

# Unites uniques

## Pisteur d'Elyr

Le Pisteur d'Elyr est une unite d'exploration et de marquage. Il est fragile, mais essentiel pour controler la carte et preparer les attaques a distance.

| Statistique | Valeur |
|---|---:|
| PV | 40 |
| Attaque | 5 |
| Defense | 0 |
| Portee | 1 |
| Mouvement | 5 |
| Vision | 8 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 55 |
| Or | 45 |

Recrute par : Pavillon des sentiers

Prerequis : Sentiers caches

Traits :

- eclaireur ;
- grande vision ;
- furtivite legere ;
- faible en combat direct.

Capacite speciale : Marquage de proie

Le Pisteur peut marquer une unite ennemie visible dans un rayon de 4 cases. Jusqu'au debut du prochain tour Elyr, les attaques a distance contre cette cible gagnent +1 attaque.

## Archer aux epines

L'Archer aux epines est une unite a distance unique. Il inflige moins de degats bruts qu'une unite lourde, mais affaiblit progressivement les ennemis et aide les Elyr a garder la distance.

| Statistique | Valeur |
|---|---:|
| PV | 32 |
| Attaque | 8 |
| Defense | 0 |
| Portee | 2-4 |
| Mouvement | 3 |
| Vision | 5 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 45 |
| Or | 60 |

Recrute par : Caserne

Prerequis : Arcs vivants

Traits :

- attaque a distance ;
- controle ;
- mobile ;
- fragile au corps-a-corps.

Capacite speciale : Fleches irritantes

Lorsqu'il inflige des degats, l'Archer aux epines applique Epines pendant 1 tour. Une unite affectee par Epines subit -1 mouvement lors de son prochain tour.

## Danse-lame sylvain

Infanterie legere d'elite. Le Danse-lame est fragile, mais excellent pour achever les unites isolees et punir les lignes mal protegees.

| Statistique | Valeur |
|---|---:|
| PV | 50 |
| Attaque | 11 |
| Defense | 1 |
| Portee | 1 |
| Mouvement | 4 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 70 |
| Or | 75 |

Recrute par : Caserne

Prerequis : Lames de clairiere

Traits :

- infanterie legere ;
- tres mobile ;
- fort contre unites isolees ;
- faible contre infanterie lourde en formation.

Capacite speciale : Pas entre les branches

Le Danse-lame ignore les zones de controle ennemies lorsqu'il commence son tour dans une foret.

## Cavalier cerf

Unite legere tres mobile. Excellente pour contourner et menacer les archers, les soutiens, les batiments economiques ou les machines de siege isolees.

| Statistique | Valeur |
|---|---:|
| PV | 65 |
| Attaque | 10 |
| Defense | 1 |
| Portee | 1 |
| Mouvement | 6 |
| Vision | 5 |
| Population | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 100 |
| Or | 100 |

Recrute par : Sanctuaire racinaire

Prerequis : Pactes des betes

Traits :

- unite legere mobile ;
- tres rapide ;
- bon contre archers et siege ;
- vulnerable aux unites anti-mobilite.

Capacite speciale : Charge sylvestre

Si le Cavalier cerf attaque apres s'etre deplace d'au moins 4 cases ce tour-ci, il gagne +2 attaque pour cette attaque.

---

# Batiments specifiques

## Pavillon des sentiers

Batiment d'exploration propre aux Elyr. Il renforce la vision, produit les Pisteurs et facilite le controle de carte.

| Statistique | Valeur |
|---|---:|
| PV | 180 |
| Defense | 0 |
| Taille | 1x1 |
| Vision | 5 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 50 |
| Or | 120 |

Prerequis : Forum

Produit :

- Pisteur d'Elyr.

Effets :

- revele les cases proches ;
- les unites Elyr commencant leur tour dans un rayon de 3 cases gagnent +1 vision jusqu'a la fin du tour ;
- peut etre construit pres des forets pour etendre la presence Elyr.

## Sanctuaire racinaire

Batiment economique, tactique et legerement defensif. Il soutient les zones forestieres et permet de recruter les Cavaliers cerfs.

| Statistique | Valeur |
|---|---:|
| PV | 240 |
| Defense | 1 |
| Taille | 2x2 |
| Vision | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 90 |
| Or | 160 |

Prerequis : Economie sylvestre

Produit :

- Cavalier cerf.

Effets :

- les Fermes dans un rayon de 4 cases produisent +10 nourriture si elles sont adjacentes a une foret ;
- les unites Elyr adjacentes au Sanctuaire racinaire recuperent 5 PV au debut du tour ;
- ne peut etre construit que sur ou pres d'une zone forestiere.

Role :

Le Sanctuaire racinaire donne aux Elyr une economie locale forte, mais dependante de la carte.

---

# Technologies specifiques

Toutes les recherches Elyr sont lancees depuis l'Universite. Les batiments indiques sont des prerequis, pas les lieux de recherche.

## Sentiers caches

Les Elyr utilisent des chemins invisibles aux autres peuples pour se deplacer et observer.

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 80 or |
| Prerequis | Pavillon des sentiers |
| Effet | Debloque Pisteur d'Elyr, +1 vision pour les eclaireurs Elyr |

## Veilleurs des clairieres

Organise les eclaireurs en reseau de surveillance.

| Champ | Valeur |
|---|---|
| Cout | 160 points de technologie, 120 or |
| Prerequis | Sentiers caches |
| Effet | Les Pavillons des sentiers gagnent +2 vision et detectent les unites camouflees proches |

## Arcs vivants

Les artisans Elyr cultivent des arcs flexibles et puissants a partir de bois rituel.

| Champ | Valeur |
|---|---|
| Cout | 160 points de technologie, 140 or |
| Prerequis | Caserne, Archerie organisee |
| Effet | Debloque Archer aux epines |

## Tir mouvant

Les archers Elyr apprennent a tirer tout en conservant leur capacite de repositionnement.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 160 nourriture, 160 or |
| Prerequis | Arcs vivants |
| Effet | Les unites a distance Elyr peuvent se deplacer de 1 case apres une attaque si elles ne sont pas adjacentes a un ennemi |

## Lames de clairiere

Forme des combattants capables de se battre dans les sous-bois et d'exploiter les ouvertures.

| Champ | Valeur |
|---|---|
| Cout | 180 points de technologie, 140 nourriture, 160 or |
| Prerequis | Caserne, Discipline militaire |
| Effet | Debloque Danse-lame sylvain |

## Pactes des betes

Les Elyr nouent des liens avec les grands cerfs des forets anciennes.

| Champ | Valeur |
|---|---|
| Cout | 240 points de technologie, 220 nourriture, 220 or |
| Prerequis | Sentiers caches, Sanctuaire racinaire |
| Effet | Debloque Cavalier cerf |

## Cartographie vivante

Les cartes Elyr evoluent avec les mouvements des eclaireurs et les signes de la foret.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 220 or |
| Prerequis | Veilleurs des clairieres |
| Effet | Les unites Elyr gagnent +1 mouvement lorsqu'elles commencent leur tour dans une case revelee mais hors vision ennemie connue |

## Embuscade coordonnee

Les Elyr apprennent a concentrer leurs tirs sur des cibles piegees ou mal positionnees.

| Champ | Valeur |
|---|---|
| Cout | 260 points de technologie, 180 nourriture, 240 or |
| Prerequis | Tir mouvant, Veilleurs des clairieres |
| Effet | Les attaques depuis une foret ou contre une cible marquee gagnent +1 attaque supplementaire |

## Racines nourricieres

Ameliore les sanctuaires forestiers et rend l'economie Elyr plus durable.

| Champ | Valeur |
|---|---|
| Cout | 240 points de technologie, 180 nourriture, 180 or |
| Prerequis | Economie sylvestre, Sanctuaire racinaire |
| Effet | Les Sanctuaires racinaires augmentent leur soin a 8 PV et leur bonus de production locale a +15 nourriture |

---

# Arbre technologique Elyr

```text
Universite
├── Sentiers caches ← Pavillon des sentiers
│   ├── Pisteur d'Elyr
│   ├── Veilleurs des clairieres
│   │   ├── Cartographie vivante
│   │   └── Embuscade coordonnee
│   └── Pactes des betes ← Sanctuaire racinaire
│       └── Cavalier cerf
│
├── Arcs vivants ← Caserne + Archerie organisee
│   ├── Archer aux epines
│   └── Tir mouvant
│       └── Embuscade coordonnee
│
├── Lames de clairiere ← Caserne + Discipline militaire
│   └── Danse-lame sylvain
│
└── Racines nourricieres ← Sanctuaire racinaire
```

---

# Plan de jeu recommande

## Debut de partie

Les Elyr doivent explorer vite et prendre de l'information avant l'adversaire. Leur debut de partie repose sur la vision, les forets et les petites escarmouches.

Objectifs :

- construire rapidement un Pavillon des sentiers ;
- rechercher Sentiers caches ;
- produire un ou deux Pisteurs ;
- identifier les ressources et chemins d'acces ennemis ;
- eviter les combats frontaux.

## Milieu de partie

Les Elyr deviennent dangereux lorsque leurs archers et leur mobilite commencent a se combiner.

Objectifs :

- rechercher Arcs vivants ;
- produire des Archers aux epines ;
- utiliser les Pisteurs pour marquer les cibles ;
- harceler les villageois, les soutiens et les batiments economiques ;
- poser un Sanctuaire racinaire pres d'une zone boisee importante.

## Fin de partie

Les Elyr doivent eviter la bataille frontale totale. Ils cherchent a isoler les cibles, epuiser l'adversaire et attaquer les flancs.

Objectifs :

- utiliser Tir mouvant pour maintenir la distance ;
- combiner Pisteurs et Archers aux epines ;
- envoyer les Cavaliers cerfs sur les arrieres ;
- utiliser les Danse-lames pour punir les unites isolees ;
- utiliser Sylwen pour declencher les replis decisifs ;
- eviter les murs, bastions et formations lourdes sans support.

---

# Contres et vulnerabilites

## Fort contre

| Cible | Raison |
|---|---|
| Armees lentes | Les Elyr peuvent choisir quand engager |
| Machines de siege isolees | Les Cavaliers cerfs et Danse-lames peuvent les atteindre vite |
| Economies exposees | Leur mobilite permet le harcelement |
| Factions dependantes des formations | Les Elyr peuvent forcer la dispersion |

## Faible contre

| Menace | Raison |
|---|---|
| Fortifications lourdes | Leur siege est limite |
| Unites rapides anti-archers | Elles peuvent les punir si mal positionnes |
| Degats de zone | Leurs unites ont peu de PV |
| Cartes fermees | Elles reduisent la valeur de la mobilite et de la vision |

---

# Notes d'equilibrage

- Les Elyr doivent avoir l'impression d'etre toujours un pas en avance, mais punissables lorsqu'ils sont attrapes.
- Sylwen doit permettre des repositionnements decisifs, mais ne doit pas rendre toute l'armee intouchable.
- Le Tir mobile doit etre fort tactiquement, mais ne doit pas rendre les archers impossibles a engager.
- Le Pisteur doit etre utile meme sans attaquer, grace a la vision et au marquage.
- Le Cavalier cerf doit exceller contre les cibles isolees, mais perdre contre des unites bien placees.
- Les Elyr doivent avoir une vraie faiblesse contre les fortifications et le siege lourd.
- La faction doit recompenser les bons placements plus que les statistiques brutes.

---

# Direction artistique

## Themes visuels

- bois clair et vivant ;
- tissus verts, bruns, ocres et blancs ;
- arcs organiques ;
- armures legeres en cuir, bois et metal fin ;
- silhouettes fines et mobiles ;
- architectures suspendues ou integrees aux arbres ;
- symboles de feuilles, cerfs, lunes et racines.

## Ambiance

Les Sylvains d'Elyr doivent evoquer une civilisation elegante, vigilante et difficile a saisir. Ils ne sont pas simplement des archers : ce sont des chasseurs, guides, gardiens de forets et maitres du terrain.

## Mots-cles

- foret ;
- mobilite ;
- vision ;
- embuscade ;
- arc ;
- sentier ;
- repli ;
- clairiere ;
- chasse ;
- harcelement.
