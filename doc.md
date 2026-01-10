# Documentation du Projet

## Architecture du Code

### `src/main.rs`
Point d'entrée du programme. Gère la boucle principale de rendu avec Macroquad, les contrôles utilisateur (pause, step, reset) et l'affichage des informations à l'écran.

### `src/grid.rs`
Définit la grille 2D avec les types de cellules (`Empty`, `Wall`, `Agent`, `Exit`). Initialise le terrain avec les murs de bordure, obstacles internes et la sortie. Gère l'affichage visuel de la grille.

### `src/agent.rs`
Définit la structure `Agent` et son comportement. Implémente la sélection probabiliste de la prochaine position basée sur le champ de distance (formule $P = e^{-k_S \cdot S}$) en utilisant le voisinage de Moore (8 directions).

### `src/floor_field.rs`
Calcule le champ statique de distance vers la sortie via l'algorithme BFS (Breadth-First Search). Chaque cellule contient sa distance euclidienne à la sortie la plus proche, utilisée pour guider les agents.

### `src/simulation.rs`
Orchestre la simulation complète. Gère le placement initial aléatoire des agents, exécute les étapes de simulation (choix de position + résolution de conflits), et retire les agents ayant atteint la sortie.

## Flux d'Exécution

1. **Initialisation** : Création de la grille, calcul du champ de distance, placement des agents
2. **Boucle de simulation** :
   - Chaque agent choisit sa prochaine case (probabiliste)
   - Résolution des conflits (sélection aléatoire si plusieurs agents veulent la même case)
   - Déplacement effectif et retrait des agents évacués
3. **Affichage** : Rendu visuel de l'état actuel