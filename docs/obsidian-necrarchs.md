# Necrarques d'Obsidienne

Les Necrarques d'Obsidienne sont une civilisation sombre fondee sur la magie noire, l'attrition, la peur et la reutilisation des morts. Ils ne gagnent pas toujours les combats par la qualite brute de leurs unites, mais par leur capacite a user l'adversaire, recuperer une partie de leurs pertes et maintenir une pression constante.

Cette faction doit proposer un gameplay oppressant, different des armees disciplinees Valdoriennes, des positions Kharzun et de la mobilite Elyr. Les Necrarques acceptent de perdre des unites si cela epuise l'ennemi plus vite qu'eux.

Ce document est aligne avec `docs/SPEC.md` : les ressources principales sont la nourriture, l'or et les points de technologie ; les Cadavres sont une ressource tactique temporaire propre aux Necrarques ; les recherches sont lancees depuis l'Universite ; le Marche sert uniquement a l'echange ; les unites speciales Necrarques sont recrutees dans leurs batiments speciaux lorsque precise.

---

# Resume

| Element | Valeur |
|---|---|
| Nom | Necrarques d'Obsidienne |
| Archetype | Royaume necromantique et occulte |
| Style de jeu | Attrition, peur, reanimation, pression constante |
| Rythme | Moyen a agressif |
| Difficulte | Elevee |
| Force principale | Recuperation des pertes et affaiblissement ennemi |
| Faiblesse principale | Economie instable, unites fragiles, dependance aux soutiens |

---

# Identite de gameplay

Les Necrarques gagnent en transformant chaque bataille en echange favorable sur la duree. Meme lorsqu'ils perdent des troupes, ils peuvent recuperer des corps, affaiblir l'ennemi ou creer de nouvelles opportunites tactiques.

Leur gameplay repose sur trois piliers :

- des unites nombreuses et faciles a remplacer ;
- des effets de peur, malediction et affaiblissement ;
- une recuperation partielle apres les combats grace aux Cadavres.

Dans la nouvelle economie de Rusty Empires, les Necrarques n'utilisent pas le fer, la pierre ou le bois comme ressources globales. Leur theme d'obsidienne, d'ossements et de magie noire est represente par leurs batiments, technologies, unites, effets de peur et ressource tactique Cadavres.

Ils sont dangereux lorsqu'ils peuvent enchaîner les affrontements. En revanche, ils souffrent contre les attaques rapides sur leur economie, les degats de zone ou les armees capables de detruire leurs soutiens magiques.

## Forces

- Bonne pression militaire continue.
- Certaines unites coutent peu ou pas de nourriture.
- Recuperation partielle des pertes.
- Effets de peur et de malediction.
- Forts dans les combats longs et repetes.

## Faiblesses

- Economie alimentaire moins efficace.
- Unites de base faibles individuellement.
- Depend fortement de ses unites de soutien.
- Vulnerable aux attaques ciblees sur les Acolytes, Liches et le heros.
- Peu de mobilite explosive.

---

# Bonus et malus de civilisation

## Bonus de civilisation

| Bonus | Effet |
|---|---|
| Serviteurs sans faim | Les unites mort-vivantes coutent 75% de nourriture en moins |
| Reanimation mineure | Apres un combat gagne, 15% des pertes mort-vivantes legeres sont recuperees |
| Aura de terreur | Les unites ennemies adjacentes aux unites mort-vivantes d'elite perdent -1 attaque |
| Savoir d'obsidienne | Les technologies occultes coutent 10% d'or en moins |
| Marche des morts | Les unites mort-vivantes ignorent les malus de moral |

## Malus de civilisation

| Malus | Effet |
|---|---|
| Terres corrompues | Les Fermes Necrarques produisent 15% de nourriture en moins |
| Population stagnante | Les villageois coutent 10% de nourriture en plus |
| Commerce sinistre | Les echanges au Marche sont 10% moins efficaces |
| Corps fragiles | Les unites mort-vivantes legeres ont -10% PV |
| Dependence occulte | Certaines unites perdent en efficacite si aucun soutien magique n'est proche |

## Capacite passive : La mort nourrit la mort

Lorsqu'une unite organique meurt dans un rayon de 4 cases autour d'une unite de soutien Necrarque, elle genere un marqueur de Cadavre.

Les marqueurs de Cadavre peuvent servir a :

- renforcer une reanimation apres combat ;
- alimenter certaines capacites occultes ;
- reduire le cout ou permettre la creation d'unites mort-vivantes legeres.

Cette capacite encourage le joueur Necrarque a combattre souvent et a transformer les pertes des deux camps en ressource tactique.

---

# Ressource tactique specifique : Cadavres

Les Cadavres ne sont pas une ressource economique classique. Ils existent sur la carte ou dans une reserve temporaire apres les combats.

## Generation

Un Cadavre est genere lorsqu'une unite organique meurt. Certaines unites mecaniques, spectrales ou invoquees ne generent pas de Cadavres.

## Utilisations possibles

| Utilisation | Effet |
|---|---|
| Reanimation | Recuperer ou creer des unites mort-vivantes legeres |
| Rituel | Alimenter une capacite magique puissante |
| Malediction | Renforcer certains debuffs |
| Recrutement occulte | Servir de prerequis pour certaines unites speciales |

## Limites

- Les Cadavres disparaissent apres quelques tours si non utilises.
- Les ennemis pourront plus tard avoir des technologies pour purifier les Cadavres.
- Les Cadavres doivent rester une ressource tactique, pas une economie infinie.

---

# Heros de faction

## Morvhal l'Inhume

Morvhal est le heros des Necrarques d'Obsidienne. C'est un ancien souverain enterre vivant, releve par les rituels d'obsidienne et devenu maitre des processions funebres.

Il n'est pas concu comme un duelliste pur. Sa force vient de sa capacite a transformer les morts proches en pression militaire et a rendre les combats longs favorables aux Necrarques.

| Statistique | Valeur |
|---|---:|
| PV | 95 |
| Attaque | 11 |
| Defense | 3 |
| Portee | 2-4 |
| Mouvement | 3 |
| Vision | 5 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 60 |
| Or | 220 |
| Cadavres | 2 |

Recrute par : Forum

Prerequis : Universite, Crypte d'obsidienne

Traits :

- heros ;
- soutien magique ;
- mort-vivant ;
- terreur ;
- reanimation.

Capacite speciale : Appel du tombeau

Une fois tous les 3 tours, Morvhal consomme jusqu'a 3 Cadavres dans un rayon de 4 cases.

Effets :

- cree un Serviteur osseux adjacent a Morvhal pour chaque Cadavre consomme ;
- les unites ennemies dans un rayon de 2 cases subissent -1 attaque jusqu'au debut du prochain tour Necrarque ;
- si aucun Cadavre n'est disponible, Morvhal peut tout de meme appliquer le malus d'attaque, mais ne cree aucune unite.

Role tactique :

- transformer une zone de combat en avantage ;
- maintenir la pression apres des pertes ;
- proteger les soutiens occultes ;
- punir les ennemis qui s'engagent trop longtemps.

---

# Unites uniques

## Serviteur osseux

Unite de base mort-vivante. Faible individuellement, mais peu couteuse et facile a remplacer.

| Statistique | Valeur |
|---|---:|
| PV | 28 |
| Attaque | 5 |
| Defense | 0 |
| Portee | 1 |
| Mouvement | 3 |
| Vision | 2 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 25 |
| Cadavres | 1 |

Recrute par : Crypte d'obsidienne

Prerequis : Rituels sombres

Traits :

- mort-vivant ;
- ignore le moral ;
- tres peu couteux ;
- faible seul ;
- peut etre reanime.

Capacite speciale : Corps remuable

Apres un combat gagne, chaque Serviteur osseux mort a 15% de chance d'etre recupere si une unite de soutien Necrarque a survecu au combat.

## Acolyte funeraire

Unite de soutien de base. Il manipule les Cadavres, affaiblit l'ennemi et permet aux armees Necrarques de rester dangereuses dans la duree.

| Statistique | Valeur |
|---|---:|
| PV | 40 |
| Attaque | 4 |
| Defense | 0 |
| Portee | 2-3 |
| Mouvement | 3 |
| Vision | 4 |
| Population | 1 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 70 |

Recrute par : Crypte d'obsidienne

Prerequis : Rituels sombres

Traits :

- soutien magique ;
- fragile ;
- genere et exploite les Cadavres ;
- essentiel pour l'attrition.

Capacite speciale : Lever les restes

Consomme jusqu'a 2 marqueurs de Cadavre dans un rayon de 3 cases pour creer un Serviteur osseux adjacent a l'Acolyte. Cette capacite a un temps de recharge de 2 tours.

## Chevalier sepulcral

Unite lourde mort-vivante. Moins rapide que les unites mobiles classiques, mais terrifiante et difficile a arreter sans anti-lourd.

| Statistique | Valeur |
|---|---:|
| PV | 85 |
| Attaque | 12 |
| Defense | 3 |
| Portee | 1 |
| Mouvement | 4 |
| Vision | 3 |
| Population | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 125 |
| Cadavres | 1 |

Recrute par : Crypte d'obsidienne

Prerequis : Chevalerie noire

Traits :

- mort-vivant ;
- unite lourde ;
- terreur ;
- ignore le moral ;
- vulnerable aux unites anti-lourdes.

Capacite speciale : Aura de terreur

Les unites ennemies adjacentes subissent -1 attaque. Les unites deja affectees par une malediction subissent aussi -1 defense.

## Liche d'obsidienne

Unite magique d'elite. Elle renforce les morts-vivants, affaiblit l'ennemi et augmente la recuperation des pertes.

| Statistique | Valeur |
|---|---:|
| PV | 65 |
| Attaque | 9 |
| Defense | 1 |
| Portee | 2-4 |
| Mouvement | 3 |
| Vision | 5 |
| Population | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 180 |
| Cadavres | 2 |

Recrute par : Crypte d'obsidienne

Prerequis : Sorcellerie d'obsidienne

Traits :

- soutien magique elite ;
- attaque a distance ;
- amplifie les morts-vivants ;
- cible prioritaire pour l'adversaire.

Aura : Volonte du tombeau

Les unites mort-vivantes alliees dans un rayon de 2 cases gagnent +1 attaque.

Capacite speciale : Reanimation superieure

Apres un combat gagne dans lequel la Liche a participe ou etait proche, la recuperation des pertes mort-vivantes legeres passe de 15% a 25%.

## Char funebre

Machine rituelle lente. Elle ne sert pas seulement au combat : elle transporte des Cadavres et soutient les armees d'attrition.

| Statistique | Valeur |
|---|---:|
| PV | 110 |
| Attaque | 6 |
| Defense | 3 |
| Portee | 1 |
| Mouvement | 2 |
| Vision | 3 |
| Population | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 140 |
| Cadavres | 1 |

Recrute par : Crypte d'obsidienne

Prerequis : Processions macabres

Traits :

- support lourd ;
- transport de Cadavres ;
- lent ;
- renforce l'attrition.

Capacite speciale : Reliquaire mobile

Stocke jusqu'a 4 Cadavres. Les Acolytes, Liches et Morvhal proches peuvent utiliser ces Cadavres comme s'ils etaient sur des cases adjacentes.

---

# Batiments specifiques

## Crypte d'obsidienne

Batiment central des Necrarques. Elle produit les unites mort-vivantes et stocke les Cadavres.

| Statistique | Valeur |
|---|---:|
| PV | 360 |
| Defense | 2 |
| Taille | 2x2 |
| Vision | 3 |

Cout :

| Ressource | Valeur |
|---|---:|
| Nourriture | 60 |
| Or | 220 |

Prerequis : Forum

Produit :

- Serviteur osseux ;
- Acolyte funeraire ;
- Chevalier sepulcral ;
- Liche d'obsidienne ;
- Char funebre.

Effets :

- stocke jusqu'a 6 Cadavres ;
- les unites mort-vivantes produites ici coutent 10% d'or en moins si au moins 3 Cadavres sont stockes ;
- agit comme point de ralliement des morts-vivants.

Note de recherche :

Les recherches occultes sont lancees a l'Universite. La Crypte d'obsidienne sert de prerequis et de batiment de production.

## Obelisque de peur

Batiment de controle de zone. Il affaiblit les ennemis proches et protege les positions Necrarques.

| Statistique | Valeur |
|---|---:|
| PV | 280 |
| Defense | 2 |
| Taille | 1x1 |
| Vision | 5 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 170 |
| Cadavres | 1 |

Prerequis : Maledictions gravees

Effets :

- les unites ennemies dans un rayon de 3 cases subissent -1 vision ;
- les unites ennemies adjacentes a l'Obelisque subissent -1 attaque ;
- les unites mort-vivantes alliees dans un rayon de 2 cases ignorent les malus de terrain lies a la peur ou au moral.

Limite :

- ne peut pas etre construit a moins de 5 cases d'un autre Obelisque de peur.

## Fosse commune

Batiment occulte. Il permet de convertir les pertes en avantage tactique.

| Statistique | Valeur |
|---|---:|
| PV | 220 |
| Defense | 1 |
| Taille | 1x1 |
| Vision | 2 |

Cout :

| Ressource | Valeur |
|---|---:|
| Or | 100 |

Prerequis : Rituels sombres

Effets :

- stocke jusqu'a 4 Cadavres ;
- augmente de 5% la chance de recuperation des Serviteurs osseux dans un rayon de 4 cases ;
- peut servir de point de creation pour Lever les restes si un Acolyte est proche.

---

# Technologies specifiques

Toutes les recherches Necrarques sont lancees depuis l'Universite. Les batiments indiques sont des prerequis, pas les lieux de recherche.

## Rituels sombres

Premiere technologie occulte. Elle debloque les bases de la necromancie militaire.

| Champ | Valeur |
|---|---|
| Cout | 120 points de technologie, 120 or |
| Prerequis | Crypte d'obsidienne |
| Effet | Debloque Serviteur osseux, Acolyte funeraire et Fosse commune |

## Marches funebres

Les morts avancent sans fatigue, guides par les murmures des cryptes.

| Champ | Valeur |
|---|---|
| Cout | 160 points de technologie, 160 or |
| Prerequis | Rituels sombres |
| Effet | Les unites mort-vivantes gagnent +1 mouvement lorsqu'elles commencent leur tour a 3 cases ou moins d'une Crypte, Fosse commune, Liche, Morvhal ou Char funebre |

## Maledictions gravees

Les Necrarques inscrivent des glyphes de peur sur l'obsidienne et les ossements.

| Champ | Valeur |
|---|---|
| Cout | 180 points de technologie, 180 or |
| Prerequis | Rituels sombres |
| Effet | Debloque Obelisque de peur, les Acolytes peuvent appliquer Malediction mineure |

## Chevalerie noire

Les anciens cavaliers morts sont releves et lies a des armures funeraires.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 220 or, 2 Cadavres |
| Prerequis | Marches funebres |
| Effet | Debloque Chevalier sepulcral |

## Processions macabres

Les Necrarques apprennent a transporter les morts et les reliques directement sur le champ de bataille.

| Champ | Valeur |
|---|---|
| Cout | 220 points de technologie, 200 or |
| Prerequis | Marches funebres |
| Effet | Debloque Char funebre |

## Sorcellerie d'obsidienne

Les mages d'obsidienne lient les ames mortes aux cristaux noirs.

| Champ | Valeur |
|---|---|
| Cout | 280 points de technologie, 260 or, 3 Cadavres |
| Prerequis | Maledictions gravees, Processions macabres |
| Effet | Debloque Liche d'obsidienne |

## Legions sans fin

Les Necrarques perfectionnent les rituels permettant de relever des morts en masse.

| Champ | Valeur |
|---|---|
| Cout | 340 points de technologie, 350 or, 5 Cadavres |
| Prerequis | Sorcellerie d'obsidienne |
| Effet | La recuperation des pertes mort-vivantes legeres passe de 15% a 20%, ou de 25% a 30% avec une Liche proche |

## Pacte du tombeau

Les unites mort-vivantes deviennent plus dangereuses lorsqu'elles combattent autour des lieux de mort.

| Champ | Valeur |
|---|---|
| Cout | 240 points de technologie, 220 or, 2 Cadavres |
| Prerequis | Maledictions gravees, Fosse commune |
| Effet | Les unites mort-vivantes gagnent +1 attaque si elles commencent leur tour a 2 cases ou moins d'une Fosse commune ou d'un Cadavre |

## Voile de desespoir

Les ennemis proches des structures occultes perdent confiance et coordination.

| Champ | Valeur |
|---|---|
| Cout | 260 points de technologie, 260 or |
| Prerequis | Maledictions gravees, Obelisque de peur |
| Effet | Les Obelisques de peur reduisent aussi de -1 la defense des ennemis deja maudits dans leur zone |

---

# Arbre technologique Necrarque

```text
Universite
└── Rituels sombres ← Crypte d'obsidienne
    ├── Serviteur osseux
    ├── Acolyte funeraire
    ├── Fosse commune
    ├── Marches funebres
    │   ├── Processions macabres
    │   │   ├── Char funebre
    │   │   └── Sorcellerie d'obsidienne
    │   │       ├── Liche d'obsidienne
    │   │       └── Legions sans fin
    │   └── Chevalerie noire
    │       └── Chevalier sepulcral
    └── Maledictions gravees
        ├── Obelisque de peur
        ├── Pacte du tombeau
        ├── Voile de desespoir
        └── Sorcellerie d'obsidienne
```

---

# Plan de jeu recommande

## Debut de partie

Les Necrarques doivent atteindre rapidement Rituels sombres pour commencer a convertir les combats en avantage d'attrition.

Objectifs :

- construire une Crypte d'obsidienne ;
- construire une Universite ;
- rechercher Rituels sombres ;
- produire quelques Serviteurs osseux ;
- garder les Acolytes en securite ;
- chercher de petits combats favorables pour generer des Cadavres.

## Milieu de partie

La faction devient dangereuse lorsque les Cadavres commencent a alimenter les unites et les rituels.

Objectifs :

- utiliser Lever les restes pour maintenir la pression ;
- poser une Fosse commune pres d'une zone de combat ;
- rechercher Maledictions gravees ou Marches funebres selon le besoin ;
- produire des Chevaliers sepulcraux pour menacer les lignes arrieres ;
- proteger les Acolytes et les Chars funebres.

## Fin de partie

Les Necrarques cherchent a transformer le champ de bataille en zone maudite ou chaque mort les renforce.

Objectifs :

- deployer Liches, Chars funebres et Morvhal ;
- maintenir des reserves de Cadavres ;
- utiliser les Obelisques de peur pour affaiblir les positions ennemies ;
- recycler les pertes avec Legions sans fin ;
- forcer plusieurs combats successifs plutot qu'une seule bataille decisive.

---

# Contres et vulnerabilites

## Fort contre

| Cible | Raison |
|---|---|
| Armees cheres | L'attrition rend les pertes adverses plus douloureuses |
| Factions defensives lentes | Les Necrarques peuvent accumuler des Cadavres autour des sieges |
| Combats repetes | Leur recuperation devient rentable sur la duree |
| Unites sensibles au moral | Les effets de peur reduisent leur efficacite |

## Faible contre

| Menace | Raison |
|---|---|
| Assassinats de soutien | Les Acolytes, Liches et Morvhal sont essentiels |
| Harcelement economique | Leur economie alimentaire est mediocre |
| Degats de zone | Les Serviteurs osseux ont peu de PV |
| Purification ou anti-magie | Peut neutraliser les Cadavres et maledictions |
| Rush tres rapide | Ils ont besoin d'un minimum de mise en place |

---

# Notes d'equilibrage

- Les Serviteurs osseux doivent etre nombreux, mais jamais forts seuls.
- Les Cadavres doivent etre utiles sans devenir une economie infinie.
- Les Acolytes doivent etre importants, mais leur perte ne doit pas rendre l'armee totalement inutile.
- Morvhal doit etre dangereux dans une zone chargee en Cadavres, mais pas dominant sans preparation.
- Les Liches doivent etre puissantes, visibles et prioritaires pour l'adversaire.
- La recuperation apres combat doit recompenser les bons engagements, pas annuler gratuitement les erreurs.
- Les Necrarques doivent donner une sensation de pression constante, pas d'invincibilite.

---

# Direction artistique

## Themes visuels

- obsidienne noire ;
- os blanchis ;
- lueurs violettes, bleues ou vertes ;
- cryptes anguleuses ;
- bannieres dechirees ;
- silhouettes maigres et menaçantes ;
- armures funeraires ;
- glyphes graves dans la pierre sombre.

## Ambiance

Les Necrarques d'Obsidienne doivent evoquer une civilisation froide, rituelle et inexorable. Ils ne sont pas seulement des monstres : ce sont des nobles, mages et pretres qui considerent la mort comme une matiere premiere politique et militaire.

## Mots-cles

- obsidienne ;
- crypte ;
- mort ;
- rituel ;
- peur ;
- attrition ;
- cadavres ;
- reanimation ;
- liche ;
- inexorable.
