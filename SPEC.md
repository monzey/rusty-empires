# Rusty Empires - Specification fonctionnelle

## Vision du jeu

Rusty Empires est un jeu de strategie tactique au tour par tour, joue sur une carte composee de tuiles en vue isometrique 2D.

Le joueur dirige une faction, developpe son economie, construit des batiments, recrute des unites, recherche des technologies et affronte les factions adverses dans des combats positionnels.

L'experience vise un melange entre gestion legere d'empire, tactique militaire lisible et progression par ages.

## Principes de jeu

- Le jeu se deroule au tour par tour.
- Chaque joueur controle une faction.
- Chaque faction possede ses propres unites, son heros et son arbre de technologies.
- La carte est divisee en tuiles.
- Les tuiles peuvent contenir du terrain, une ressource, un batiment ou une unite.
- Les ressources servent a construire, recruter et rechercher des technologies.
- Le controle du territoire et des ressources est central dans la progression.

## Boucle de jeu

Un tour permet au joueur de prendre des decisions economiques, tactiques et strategiques.

Pendant son tour, le joueur peut notamment :

- Deplacer ses unites.
- Attaquer avec ses unites de combat ou son heros.
- Utiliser les capacites speciales disponibles.
- Construire des batiments avec ses villageois.
- Recruter de nouvelles unites dans les batiments adaptes.
- Echanger des ressources au marche.
- Recruter des mercenaires au marche.
- Lancer ou poursuivre des recherches technologiques.
- Terminer son tour.

A la fin du tour, les productions passives sont resolues :

- Les mines d'or produisent de l'or.
- Les fermes produisent de la nourriture.
- Les effets de technologies ou capacites de fin de tour sont appliques.

## Carte

La carte est une grille de tuiles affichee en vue isometrique 2D.

Chaque tuile peut definir :

- Un type de terrain.
- Une eventuelle ressource naturelle.
- Un batiment construit.
- Une unite presente.
- Des contraintes de deplacement ou de construction.

### Ressources naturelles

Certaines tuiles contiennent une ressource exploitable :

- Gisement d'or : permet la construction d'une mine d'or.
- Champ : permet la construction d'une ferme.

Ces ressources naturelles conditionnent le placement de certains batiments economiques.

## Ressources

Le jeu utilise au minimum les ressources suivantes :

- Or : utilise pour recruter, construire, echanger et financer certaines technologies.
- Nourriture : utilisee pour recruter des unites, soutenir l'economie et financer certaines technologies.
- Points de technologie : utilises pour rechercher des avancees dans l'arbre technologique.

Les quantites exactes, couts et rendements sont a definir pendant l'equilibrage.

## Factions

Le jeu contient plusieurs factions jouables.

Chaque faction possede :

- Une identite de gameplay propre.
- Des unites specifiques.
- Un heros unique.
- Des capacites ou avantages distinctifs.
- Un arbre de technologies adapte a ses forces et faiblesses.

Les factions doivent encourager des styles de jeu differents : expansion economique, pression militaire rapide, defense, controle de carte, superiorite technologique ou strategies hybrides.

## Unites

Il existe trois grandes categories d'unites.

### Villageois

Les villageois sont les unites economiques principales.

Ils permettent de :

- Construire des batiments.
- Etendre l'infrastructure de la faction.
- Participer indirectement a la production de ressources.

Les villageois ne sont pas concus comme des unites de combat principales.

### Unites de combat

Les unites de combat constituent l'armee standard de chaque faction.

Elles servent a :

- Attaquer les unites adverses.
- Defendre les positions importantes.
- Controler les ressources et les acces strategiques.
- Exploiter les forces propres a leur faction.

Certaines unites de combat peuvent etre exclusives a une faction.

### Heros

Chaque faction possede un heros unique.

Le heros est une unite speciale qui dispose :

- De statistiques ameliorees.
- D'une capacite speciale forte.
- D'un role tactique central.
- D'une identite liee a sa faction.

Le heros doit etre puissant sans rendre les autres unites inutiles.

## Statistiques des unites

Chaque unite possede au minimum :

- Attaque : determine sa puissance offensive.
- Defense : reduit ou limite les degats subis.
- Capacite speciale : apporte un effet tactique distinctif.

D'autres statistiques pourront etre ajoutees si necessaire, par exemple :

- Points de vie.
- Portee d'attaque.
- Points de mouvement.
- Cout de recrutement.
- Conditions de recrutement.

## Capacites speciales

Chaque unite dispose d'une capacite speciale.

Une capacite speciale peut par exemple :

- Infliger des degats bonus dans certaines conditions.
- Ameliorer temporairement une statistique.
- Proteger une unite alliee.
- Affaiblir une unite ennemie.
- Modifier le deplacement.
- Interagir avec un type de terrain ou de batiment.

Les capacites doivent rester lisibles et previsibles pour favoriser les decisions tactiques.

## Batiments

Les batiments structurent l'economie, le recrutement et la progression technologique.

### Mine d'or

La mine d'or permet de produire de l'or a chaque fin de tour.

Regles :

- Constructible uniquement sur une tuile contenant un gisement d'or.
- Produit de l'or a la fin de chaque tour du proprietaire.
- Le rendement exact est a definir.

### Ferme

La ferme permet de produire de la nourriture a chaque fin de tour.

Regles :

- Constructible uniquement sur une tuile contenant un champ.
- Produit de la nourriture a la fin de chaque tour du proprietaire.
- Le rendement exact est a definir.

### Forum

Le forum est le batiment central de la faction.

Il permet de :

- Creer des villageois.
- Servir de point de depart au developpement de la base.
- Autoriser la construction d'autres batiments sur les tuiles adjacentes.

Regle de construction :

- Certains batiments doivent etre construits adjacent au forum ou a une infrastructure autorisee, selon les regles d'expansion retenues.

### Caserne

La caserne permet de recruter les unites de combat de la faction.

Elle donne acces :

- Aux unites militaires standards.
- Aux unites specifiques a la faction, si elles sont debloquees.
- Aux ameliorations ou prerequis militaires, si le systeme le demande.

### Marche

Le marche permet d'interagir avec l'economie globale.

Il permet de :

- Echanger des ressources.
- Convertir de l'or en nourriture ou inversement, selon un taux a definir.
- Recruter des mercenaires.

Les mercenaires sont des unites non disponibles dans le roster normal de la faction.

Ils permettent de compenser temporairement une faiblesse ou de surprendre l'adversaire, mais doivent avoir un cout ou une contrainte qui limite leur usage abusif.

### Universite

L'universite permet de generer des points de technologie.

Elle permet de :

- Produire des points de technologie.
- Debloquer l'acces aux recherches avancees.
- Soutenir la progression dans l'arbre technologique.

Le rythme de production des points de technologie est a definir.

## Technologies et ages

Le jeu contient un arbre de technologies organise par age et par faction.

Chaque age represente une etape de progression.

Les technologies peuvent :

- Ameliorer les statistiques des unites.
- Debloquer de nouvelles unites.
- Debloquer de nouveaux batiments ou options de construction.
- Ameliorer la production de ressources.
- Ameliorer les capacites speciales.
- Renforcer l'identite d'une faction.

Chaque faction dispose d'un arbre de technologies propre ou partiellement specifique.

L'objectif est que deux factions ne progressent pas exactement de la meme maniere, meme si elles partagent certaines bases communes.

## Combat

Le combat est tactique et se resout au tour par tour.

Une unite peut attaquer une cible ennemie valide selon les regles de portee, de position et d'action disponible.

La resolution de combat prend en compte au minimum :

- L'attaque de l'unite attaquante.
- La defense de l'unite cible.
- Les effets de capacites speciales.
- Les bonus ou malus de technologies.
- Les eventuels bonus de faction.

Les formules exactes de degats sont a definir.

## Construction

Les constructions sont realisees par les villageois.

Une construction doit respecter :

- Le type de tuile requis.
- Les contraintes de ressource naturelle, si le batiment en depend.
- Les contraintes d'adjacence, notamment autour du forum.
- Le cout en ressources.
- Les prerequis technologiques ou d'age, si applicables.

## Recrutement

Les unites sont recrutees dans les batiments correspondants.

- Les villageois sont crees au forum.
- Les unites de combat sont recrutees a la caserne.
- Les mercenaires sont recrutes au marche.
- Le heros de faction est recrute ou obtenu selon une regle a definir.

Chaque recrutement consomme des ressources et peut etre limite par des prerequis d'age, de technologie ou de batiment.

## Conditions de victoire

Les conditions de victoire exactes sont a definir.

Options possibles :

- Eliminer toutes les unites et batiments adverses.
- Detruire le forum ennemi.
- Controler certains objectifs de carte pendant plusieurs tours.
- Atteindre une condition technologique ou economique avancee.

La condition principale doit rester compatible avec un jeu tactique au tour par tour et encourager l'affrontement pour le controle de la carte.

## Objectifs de conception

- Rendre chaque tour significatif.
- Donner de l'importance au placement sur la carte.
- Differencier clairement les factions.
- Faire coexister economie, technologie et combat.
- Garder des regles lisibles pour le joueur.
- Permettre l'extension future avec de nouvelles factions, unites, batiments et technologies.

## Points a definir

- Nombre de factions de depart.
- Liste exacte des unites par faction.
- Identite et capacite de chaque heros.
- Valeurs des statistiques de base.
- Formule de combat.
- Cout et rendement des batiments.
- Nombre d'ages.
- Structure detaillee des arbres technologiques.
- Conditions de victoire retenues.
- Regles exactes de recrutement du heros.
