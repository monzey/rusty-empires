# Rusty Empires - Specification fonctionnelle

## Vision du jeu

Rusty Empires est un jeu de strategie tactique au tour par tour, joue sur une carte composee de tuiles en vue isometrique 2D.

Le joueur dirige une faction, developpe son economie, construit des batiments, recrute des unites, recherche des technologies et affronte les factions adverses dans des combats positionnels.

L'experience vise un melange entre gestion d'empire, tactique militaire lisible, controle de territoire et progression technologique par faction.

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
- Lancer ou poursuivre des recherches technologiques a l'universite.
- Terminer son tour.

A la fin du tour, les productions passives sont resolues :

- Les mines d'or produisent de l'or.
- Les fermes produisent de la nourriture.
- Les universites produisent des points de technologie.
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

### Bonus et malus de terrain

Le terrain peut modifier les statistiques defensives d'une unite presente sur la tuile.

- Plaine : aucun bonus ni malus.
- Batiment allie sur la tuile : +40% de defense pour l'unite qui occupe cette tuile.

Les bonus ou malus de defense sont appliques a la defense de la cible avant le calcul des degats.

## Ressources

Le jeu utilise les ressources principales suivantes :

- Or : utilise pour recruter, construire, echanger et financer certaines technologies.
- Nourriture : utilisee pour recruter des unites, soutenir l'economie et financer certaines technologies.
- Points de technologie : utilises pour rechercher des avancees dans l'arbre technologique.

Certaines factions peuvent utiliser une ressource tactique secondaire liee a leur gameplay. Exemple : les Necrarques peuvent utiliser des Cadavres comme ressource temporaire de champ de bataille.

Les quantites exactes, couts et rendements sont a definir pendant l'equilibrage.

## Factions

Le jeu contient plusieurs factions jouables.

Chaque faction possede :

- Une identite de gameplay propre.
- Des unites specifiques.
- Un heros unique.
- Des capacites ou avantages distinctifs.
- Un arbre de technologies adapte a ses forces et faiblesses.

Les factions doivent encourager des styles de jeu differents : expansion economique, pression militaire rapide, defense, controle de carte, superiorite technologique, attrition ou strategies hybrides.

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

Les unites de combat communes sont recrutees a la caserne. Certaines unites speciales de faction peuvent etre recrutees dans des batiments speciaux.

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

- Points de vie.
- Attaque.
- Defense.
- Portee d'attaque.
- Points de mouvement.
- Vision.
- Cout de recrutement.
- Conditions de recrutement.
- Capacite speciale.

Les valeurs utilisees dans les documents de gameplay servent de base d'equilibrage et pourront etre ajustees pendant le prototypage.

## Capacites speciales

Chaque unite dispose d'une capacite speciale lisible.

Une capacite speciale peut par exemple :

- Infliger des degats bonus dans certaines conditions.
- Ameliorer temporairement une statistique.
- Proteger une unite alliee.
- Affaiblir une unite ennemie.
- Modifier le deplacement.
- Interagir avec un type de terrain ou de batiment.

Recommandation de conception : une unite doit avoir une capacite principale claire. Les effets secondaires doivent rester limites afin que le joueur comprenne rapidement le role de chaque unite.

## Batiments

Les batiments structurent l'economie, le recrutement, la defense et la progression technologique.

### Batiments communs

- Forum : batiment central, cree les villageois et autorise l'expansion.
- Mine d'or : produit de l'or, constructible sur gisement d'or.
- Ferme : produit de la nourriture, constructible sur champ.
- Caserne : recrute les unites de combat communes et certaines unites de faction debloquees.
- Marche : permet uniquement l'echange de marchandises et la conversion de ressources.
- Universite : produit les points de technologie et permet de lancer les recherches.

### Batiments defensifs communs

- Tour de guet : donne de la vision et attaque a distance.

### Batiments speciaux de faction

Chaque faction peut disposer d'un ou plusieurs batiments speciaux. Ces batiments peuvent :

- Recruter des unites speciales.
- Soutenir une mecanique de faction.
- Renforcer une zone de controle.
- Debloquer certains effets tactiques.

## Technologies

Le jeu contient un arbre de technologies commun et des branches specifiques par faction.

Les technologies peuvent :

- Ameliorer les statistiques des unites.
- Debloquer de nouvelles unites.
- Debloquer de nouveaux batiments ou options de construction.
- Ameliorer la production de ressources.
- Ameliorer les capacites speciales.
- Renforcer l'identite d'une faction.

Toutes les recherches sont lancees depuis l'universite. Les autres batiments peuvent servir de prerequis, mais ne sont pas le lieu principal de recherche.

## Combat

Le combat est tactique et se resout au tour par tour.

Une unite peut attaquer une cible ennemie valide selon les regles de portee, de position et d'action disponible.

La resolution de combat prend en compte au minimum :

- L'attaque de l'unite attaquante.
- La defense de l'unite cible.
- Les bonus ou malus de terrain de la tuile de la cible.
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
- Les prerequis technologiques ou de batiment, si applicables.

## Recrutement

Les unites sont recrutees dans les batiments correspondants.

- Les villageois sont crees au forum.
- Les unites de combat communes sont recrutees a la caserne.
- Les unites speciales de faction peuvent etre recrutees dans leurs batiments speciaux.
- Le heros de faction est recrute ou obtenu selon une regle a definir.
- Le marche ne recrute pas d'unites ; il sert uniquement a l'echange de marchandises.

Chaque recrutement consomme des ressources et peut etre limite par des prerequis de technologie ou de batiment.

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

- Nombre final de factions de depart.
- Liste exacte des unites communes et de faction.
- Identite et capacite finale de chaque heros.
- Formule de combat.
- Cout et rendement des batiments.
- Structure detaillee des arbres technologiques.
- Conditions de victoire retenues.
- Regles exactes de recrutement du heros.
