# Projet : Automates Cellulaires & Systèmes Complexes

## Sujet : Modélisation Stochastique d'Évacuation de Foule (Modèle Floor-Field)

**Cours :** Automates et Systèmes Complexes
**Langage obligatoire :** Rust
**Bibliothèque utilisée :** Macroquad

---

## 1. Introduction et Objectifs

L'objectif de ce projet est d'étudier l'émergence de comportements collectifs (bouchons, flux de sortie, ondes de compression) à partir de règles de transition locales simples. Nous utiliserons un modèle d'automate cellulaire de type **Floor-Field** (Champ de Sol).

Contrairement aux simulations basées sur des forces physiques (modèles continus), ce modèle discrétise l'espace en une grille, où chaque cellule guide les agents vers la sortie via un potentiel de distance.

## 2. Spécifications du Modèle

### 2.1 La Grille

Le système est une grille 2D composée de cellules pouvant prendre 4 états :

- `VIDE` : Case libre.
- `MUR` : Obstacle infranchissable.
- `AGENT` : Un piéton (entité mobile).
- `SORTIE` : La cible à atteindre (les agents disparaissent lorsqu'ils l'atteignent).

### 2.2 Le Champ Statique ($S$)

Chaque cellule contient une valeur numérique $S_{i,j}$ représentant sa distance à la sortie.

- Ce champ est pré-calculé une seule fois via un algorithme de parcours en largeur (**BFS**) ou de **Dijkstra**.
- Les murs bloquent la propagation du champ (distance infinie).

### 2.3 Règle de Transition Probabiliste

À chaque tour de simulation (discrétisation du temps), chaque agent évalue les cases de son voisinage (Moore - 8 directions). La probabilité $P$ de choisir une cellule cible $(x, y)$ est définie par :

$$
P_{x,y} = N \cdot \exp(-k_S \cdot S_{x,y})
$$

Où :

* $S_{x,y}$ est la valeur du champ de distance.
* $k_S$ est le paramètre de **sensibilité** (plus $k_S$ est élevé, plus l'agent suit strictement le chemin le plus court).
* $N$ est une constante de normalisation.

### 2.4 Gestion des Conflits

L'automate est synchrone. Si deux agents choisissent simultanément la même cellule cible :

1. Un agent est choisi aléatoirement (ou selon un ordre de priorité) pour gagner la place.
2. L'autre agent reste immobile sur sa case d'origine pour ce tour.

## 3. Implémentation (Rust)

1. **Moteur :** Créer une structure `Grid` gérant la matrice des cellules et le champ statique.
2. **Algorithmique :** Implémenter le calcul du champ de distance à l'initialisation.
3. **Logique :** Gérer le déplacement synchrone des agents et la résolution des collisions.
4. **Visualisation :** Utiliser une bibliothèque graphique (ex: `macroquad` ou `pixels`) pour afficher l'évacuation en temps réel.

## 4. Analyse des systèmes complexes

- Étudier l'impact du paramètre $k_S$ sur la fluidité.
- Mesurer le temps d'évacuation total en fonction du nombre d'agents initial.
- Mettre en évidence le phénomène de congestion (goulot d'étranglement) aux abords de la sortie.

