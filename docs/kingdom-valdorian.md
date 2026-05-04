# Royaume Valdorien

Le Royaume Valdorien est une civilisation humaine disciplinee, organisee autour de l'administration, de l'infanterie de ligne et de la superiorite tactique par la formation.

Cette faction sert de point d'equilibre pour Rusty Empires. Elle doit etre facile a comprendre pour un nouveau joueur, mais offrir assez de profondeur pour recompenser le placement, la coordination et la gestion economique.

Ce document est aligne avec `docs/SPEC.md` : les ressources utilisees sont la nourriture, l'or et les points de technologie ; les recherches sont lancees depuis l'Universite ; le Marche sert uniquement a l'echange ; les unites de combat communes sont recrutees a la Caserne ; les unites speciales Valdoriennes utilisent les batiments speciaux de la faction lorsque precise.

---

# Resume

| Element | Valeur |
|---|---|
| Nom | Royaume Valdorien |
| Archetype | Royaume humain discipline |
| Style de jeu | Polyvalent, economique, infanterie solide |
| Rythme | Moyen |
| Difficulte | Facile a moyenne |
| Force principale | Armee fiable, formations solides, economie stable |
| Faiblesse principale | Peu de furtivite, peu de mobilite extreme, gameplay previsible |

---

# Identite de gameplay

Les Valdoriens gagnent en tenant une ligne de bataille solide, en progressant methodiquement et en exploitant une economie plus stable que celle de leurs adversaires.

Leur gameplay repose sur trois piliers :

- une infanterie fiable ;
- des bonus de formation ;
- une economie efficace sans etre explosive.

Ils ne sont pas les meilleurs en mobilite, en siege ou en harcelement, mais ils possedent rarement de gros points faibles. Ils doivent etre la civilisation de reference pour comparer l'equilibrage des autres factions.

## Forces

- Infanterie de ligne tres efficace.
- Bonus lorsqu'une armee reste groupee.
- Economie stable grace aux technologies administratives.
- Bonnes defenses de base.
- Courbe de progression claire.

## Faiblesses

- Peu d'unites rapides specialisees.
- Peu d'outils de furtivite ou d'embuscade.
- Unites elites couteuses en or.
- Gameplay parfois previsible.
- Moins efficace si les formations sont brisees.

---

# Bonus et malus de civilisation

## Bonus de civilisation

| Bonus | Effet |
|---|---|
| Administration royale | Les batiments economiques coutent 10% d'or en moins |
| Discipline de ligne | Les unites d'infanterie gagnent +1 defense lorsqu'elles sont adjacentes a une unite alliee d'infanterie |
| Routes entretenues | Les unites gagnent +1 mouvement lorsqu'elles commencent leur tour a 3 cases ou moins d'un Forum ou d'un Poste administratif allie |
| Fiscalite stable | Les recherches economiques et administratives coutent 10% d'or en moins |

## Malus de civilisation

| Malus | Effet |
|---|---|
| Doctrine rigide | Les unites legeres et les eclaireurs ne beneficient pas des bonus de formation |
| Elite couteuse | Les unites speciales Valdoriennes coutent 15% d'or en plus que les equivalents standards |
| Peu de furtivite | Les unites Valdoriennes ne peuvent pas recevoir de trait de camouflage naturel |

## Capacite passive : Tenir la ligne

Lorsqu'une unite d'infanterie Valdorienne est adjacente a au moins deux unites alliees d'infanterie, elle gagne :

- +1 defense ;
- +10% de resistance aux degats a distance ;
- immunite aux effets de recul simples.

Cette capacite encourage le joueur a former une ligne coherente plutot qu'a disperser ses soldats.

---

# Heros de faction

## Aurelian, Marechal de Valdoria

Aurelian est le heros du Royaume Valdorien. Il incarne le commandement, la discipline et la coordination tactique.

Il n'est pas concu pour gagner seul les batailles. Sa force vient de sa capacite a transformer une ligne correcte en formation difficile a briser.

| Statistique | Valeur |
|---|---:|
| PV | 110 |
| Attaque | 14 |
| Defense | 5 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 4 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 120 |
| Or | 180 |

Recrute par : Forum

Prerequis : Universite

Traits :

- heros ;
- infanterie lourde ;
- commandement ;
- formation.

Capacite speciale : Ordre royal

Une fois tous les 3 tours, Aurelian peut donner un ordre royal a toutes les unites alliees dans un rayon de 2 cases.

Effet pendant 1 tour :

- +1 attaque ;
- +1 defense si l'unite beneficie deja de Tenir la ligne ;
- les unites affectees ne peuvent pas etre repoussees par des effets de recul simples.

Role tactique :

- stabiliser une ligne de bataille ;
- preparer une poussee decisive ;
- proteger une position cle ;
- recompenser le joueur qui place bien ses unites.

---

# Unites uniques

## Legionnaire Valdorien

Le Legionnaire Valdorien est l'infanterie de ligne signature du royaume. Il ne remplace pas une unite commune dans la specification, mais devient disponible comme unite Valdorienne avancee.

| Statistique | Valeur |
|---|---:|
| PV | 70 |
| Attaque | 11 |
| Defense | 4 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 3 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 75 |
| Or | 55 |

Recrute par : Caserne

Prerequis : Doctrine legionnaire

Traits :

- infanterie lourde ;
- formation ;
- excellent en defense de ligne.

Capacite speciale : Formation de boucliers

Si le Legionnaire Valdorien est adjacent a au moins deux unites alliees d'infanterie, il gagne +2 defense au lieu de +1 via Tenir la ligne.

## Prefet de guerre

Unite de soutien militaire. Le Prefet de guerre ne doit pas etre utilise comme combattant principal, mais comme amplificateur d'armee.

| Statistique | Valeur |
|---|---:|
| PV | 55 |
| Attaque | 5 |
| Defense | 2 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 45 |
| Or | 120 |

Recrute par : Caserne legionnaire

Prerequis : Commandement royal

Traits :

- soutien ;
- commandement ;
- fragile s'il est isole ;
- augmente l'efficacite des formations.

Capacite speciale : Ordres coordonnes

Les unites d'infanterie alliees dans un rayon de 2 cases gagnent +1 attaque si elles sont adjacentes a une autre unite alliee.

Limite : un seul Prefet de guerre peut appliquer son aura a une meme unite.

## Arbaletrier royal

Unite a distance avancee. Plus lente et couteuse que l'Archer commun, mais plus efficace contre les cibles defensives.

| Statistique | Valeur |
|---|---:|
| PV | 35 |
| Attaque | 10 |
| Defense | 1 |
| Portee | 2-4 |
| Mouvement | 2 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 35 |
| Or | 85 |

Recrute par : Caserne legionnaire

Prerequis : Arbaletes royales

Traits :

- attaque a distance ;
- perforation d'armure ;
- faible mobilite ;
- efficace derriere une ligne d'infanterie.

Capacite speciale : Carreaux perforants

Ignore 1 point de defense de la cible lors d'une attaque a distance.

## Chevalier banneret

Unite de commandement mobile. Moins rapide qu'une cavalerie de harcelement, mais tres solide en soutien de ligne et en protection de flanc.

| Statistique | Valeur |
|---|---:|
| PV | 95 |
| Attaque | 13 |
| Defense | 4 |
| Portee | 1 |
| Mouvement | 4 |
| Vision | 4 |
| Population | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 110 |
| Or | 135 |

Recrute par : Caserne legionnaire

Prerequis : Noblesse militaire

Traits :

- unite lourde mobile ;
- commandement mineur ;
- protection de flanc ;
- couteux.

Capacite speciale : Banniere de ralliement

Les unites alliees adjacentes gagnent +1 defense pendant un tour apres que le Chevalier banneret a attaque.

---

# Batiments specifiques

## Poste administratif

Batiment economique propre aux Valdoriens. Il renforce la gestion locale et rend l'economie plus reguliere.

| Statistique | Valeur |
|---|---:|
| PV | 220 |
| Defense | 1 |
| Taille | 1x1 |
| Vision | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 120 |
| Nourriture | 40 |

Prerequis : Forum

Effets :

- les Mines d'or et Fermes dans un rayon de 4 cases produisent +10% de ressources ;
- reduit de 5% le cout en or des recherches economiques et administratives ;
- ne se cumule pas avec un autre Poste administratif sur les memes cases.

Role :

Le Poste administratif donne aux Valdoriens un avantage economique stable, mais demande une bonne planification du placement.

## Caserne legionnaire

Batiment militaire special Valdorien. Elle represente la professionnalisation militaire du royaume.

| Statistique | Valeur |
|---|---:|
| PV | 420 |
| Defense | 3 |
| Taille | 2x2 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 240 |
| Nourriture | 120 |

Prerequis : Doctrine legionnaire

Produit :

- Prefet de guerre ;
- Arbaletrier royal ;
- Chevalier banneret.

Effets :

- les unites speciales produites ici commencent avec +1 experience si un systeme d'experience est implemente ;
- les unites Valdoriennes adjacentes a la Caserne legionnaire gagnent +1 defense lorsqu'elles defendent ce batiment.

Note de recrutement :

- Le Legionnaire Valdorien reste recrute a la Caserne commune apres Doctrine legionnaire.
- Les unites speciales de commandement et d'elite sont recrutees a la Caserne legionnaire.

---

# Technologies specifiques

Toutes les recherches Valdoriennes sont lancees depuis l'Universite. Les batiments indiques sont des prerequis, pas les lieux de recherche.

## Routes royales

Les Valdoriens structurent leur territoire autour d'axes de circulation entretenus.

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 100 or |
| Prerequis | Forum |
| Effet | Les unites gagnent +1 mouvement si elles commencent leur tour a 3 cases ou moins d'un Forum ou d'un Poste administratif allie |

## Fiscalite organisee

Ameliore la stabilite economique du royaume.

| Champ | Valeur |
|---|---|
| Cout | 140 points de technologie, 120 or |
| Prerequis | Poste administratif, Routes royales |
| Effet | Les recherches economiques et administratives coutent 10% d'or en moins |

## Doctrine legionnaire

Standardise l'equipement et les formations de l'infanterie Valdorienne.

| Champ | Valeur |
|---|---|
| Cout | 180 points de technologie, 160 nourriture, 140 or |
| Prerequis | Caserne, Discipline militaire |
| Effet | Debloque Legionnaire Valdorien et Caserne legionnaire |

## Commandement royal

Forme des officiers capables de coordonner les lignes de bataille.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 140 nourriture, 180 or |
| Prerequis | Caserne legionnaire, Doctrine legionnaire |
| Effet | Debloque Prefet de guerre |

## Arbaletes royales

Introduit des armes a distance plus puissantes, adaptees aux armees disciplinees.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 220 or |
| Prerequis | Caserne legionnaire, Archerie organisee |
| Effet | Debloque Arbaletrier royal |

## Noblesse militaire

Mobilise les familles nobles du royaume comme unites lourdes professionnelles.

| Champ | Valeur |
|---|---|
| Cout | 260 points de technologie, 180 nourriture, 260 or |
| Prerequis | Caserne legionnaire, Fiscalite organisee |
| Effet | Debloque Chevalier banneret |

## Codex tactique

Formalise les doctrines de combat Valdoriennes.

| Champ | Valeur |
|---|---|
| Cout | 300 points de technologie, 220 nourriture, 250 or |
| Prerequis | Commandement royal |
| Effet | Les unites d'infanterie Valdoriennes gagnent +1 attaque lorsqu'elles attaquent une cible deja adjacente a une unite alliee |

## Edits royaux

Renforce la coordination entre economie, recherche et armee.

| Champ | Valeur |
|---|---|
| Cout | 280 points de technologie, 300 or |
| Prerequis | Fiscalite organisee |
| Effet | Les Forums et Postes administratifs augmentent de 1 la population maximale chacun, si le systeme de population est implemente |

## Decrets de fortification

Adapte les defenses communes a la doctrine administrative Valdorienne.

| Champ | Valeur |
|---|---|
| Cout | 240 points de technologie, 220 or |
| Prerequis | Fortifications, Poste administratif |
| Effet | Les Tours de guet et Murs dans un rayon de 4 cases d'un Poste administratif gagnent +1 defense |

---

# Arbre technologique Valdorien

```text
Universite
├── Routes royales
│   └── Fiscalite organisee
│       ├── Noblesse militaire
│       └── Edits royaux
│
├── Doctrine legionnaire ← Caserne + Discipline militaire
│   ├── Legionnaire Valdorien
│   ├── Caserne legionnaire
│   └── Commandement royal
│       ├── Prefet de guerre
│       └── Codex tactique
│
├── Arbaletes royales ← Caserne legionnaire + Archerie organisee
│   └── Arbaletrier royal
│
├── Noblesse militaire ← Caserne legionnaire + Fiscalite organisee
│   └── Chevalier banneret
│
└── Decrets de fortification ← Fortifications + Poste administratif
```

---

# Plan de jeu recommande

## Debut de partie

Le joueur Valdorien doit poser rapidement une economie stable : Forum, Mine d'or ou Ferme, Caserne, puis Universite.

Objectifs :

- securiser les ressources proches ;
- produire quelques Soldats et Archers ;
- construire un Poste administratif pres des ressources principales ;
- rechercher Routes royales pour ameliorer le controle de territoire ;
- eviter de disperser ses unites.

## Milieu de partie

Le royaume devient puissant lorsque Doctrine legionnaire est obtenue.

Objectifs :

- debloquer le Legionnaire Valdorien ;
- construire une Caserne legionnaire ;
- recruter un Prefet de guerre pour renforcer les formations ;
- ajouter des Arbaletriers royaux derriere la ligne ;
- utiliser Aurelian pour gagner les combats decisifs.

## Fin de partie

Les Valdoriens cherchent a avancer lentement mais surement avec une armee structuree.

Objectifs :

- former une ligne de Legionnaires ;
- soutenir la ligne avec Prefets et Arbaletriers ;
- utiliser les Chevaliers bannerets pour proteger les flancs ;
- renforcer les positions avec Tours, Murs et Decrets de fortification ;
- avancer avec une formation compacte plutot que chercher une attaque dispersee.

---

# Contres et vulnerabilites

## Fort contre

| Cible | Raison |
|---|---|
| Armees mal organisees | Les bonus de formation donnent l'avantage |
| Rushs faibles | Les defenses et la discipline stabilisent le debut de partie |
| Infanterie legere | Les Legionnaires gagnent les combats prolonges |
| Positions defensives moyennes | Les formations Valdoriennes progressent efficacement avec soutien |

## Faible contre

| Menace | Raison |
|---|---|
| Harcelement mobile | Les Valdoriens sont solides mais pas tres rapides |
| Degats de zone | Les formations compactes y sont vulnerables |
| Embuscades | Peu d'outils de detection specialises |
| Attrition magique | Les unites elites coutent cher a remplacer |

---

# Notes d'equilibrage

- Le Legionnaire doit etre meilleur que le Soldat en formation, mais pas beaucoup plus fort seul.
- Le Prefet de guerre doit etre utile sans devenir obligatoire dans chaque armee.
- Aurelian doit renforcer les formations, pas remplacer une armee.
- Les bonus economiques doivent etre fiables, mais moins puissants que ceux d'une faction purement economique.
- Les Valdoriens doivent rester la faction de reference pour mesurer les autres civilisations.
- Leur faiblesse principale doit venir de leur predictibilite et de leur dependance au placement.

---

# Direction artistique

## Themes visuels

- bannieres bleues, blanches et or ;
- armures propres et standardisees ;
- architecture de pierre claire ;
- routes pavees ;
- boucliers rectangulaires ou en amande ;
- symboles royaux simples : couronne, lion, soleil ou tour.

## Ambiance

Le Royaume Valdorien doit evoquer un royaume stable, militaire et administratif. Il ne doit pas sembler brutal ou mystique, mais organise, rationnel et confiant dans ses institutions.

## Mots-cles

- discipline ;
- ordre ;
- ligne ;
- royaume ;
- administration ;
- formation ;
- stabilite ;
- commandement.
