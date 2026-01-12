# Simulation d'Automate Cellulaire - Évacuation d'Agents

## Vue d'ensemble
Simulation d'agents autonomes qui évacuent un environnement vers une sortie en utilisant un champ de potentiel. Les agents utilisent des automates cellulaires pour décider de leurs mouvements.

## Concepts clés à présenter (5-7 minutes)

### 1. La Grille et les Types de Cellules (1 min)
- **Grille 2D** : Environnement discret composé de cellules
- **4 types de cellules** :
  - `Empty` : Cellules vides (blanc cassé) où les agents peuvent se déplacer
  - `Wall` : Obstacles (gris foncé) infranchissables
  - `Agent` : Position actuelle des agents (bleu)
  - `Exit` : Sortie (vert) - objectif final

### 2. Les Patterns d'Obstacles (1 min)
Démontrer la variété des scénarios :
- `Empty` : Salle vide
- `Single` : Pilier central unique
- `Rooms` : Pièces avec portes (mur vertical avec passages)
- `ExitObstacle` : Obstacle près de la sortie (goulot d'étranglement)
- `MultiObstacles` : Obstacles dispersés de tailles variées
- `Labyrinth` : Couloirs en grille

### 3. Le Champ de Potentiel (1.5 min)
**Concept central** :
- Chaque cellule vide a une "valeur" représentant sa distance à la sortie
- La sortie a la valeur la plus basse (0)
- Les valeurs augmentent en s'éloignant de la sortie
- Les murs bloquent la propagation du potentiel

**Calcul** : Algorithme de diffusion (type Dijkstra)
- Propage depuis la sortie vers toute la grille
- Prend en compte les obstacles

### 4. Règles de Mouvement des Agents (1.5 min)
**Principe** : Descendre le gradient du potentiel
- À chaque étape, l'agent examine ses **8 voisins** (Moore)
- **Règle de décision** :
  - Choisir le voisin avec le **potentiel le plus faible**
  - Vérifier que la cellule est **walkable** (pas de mur, pas d'autre agent)
  
**Gestion des collisions** :
- Si plusieurs agents veulent la même cellule → résolution (premier arrivé, ou attente)
- Si le meilleur voisin est occupé → attendre ou choisir 2ème meilleur

### 5. Interaction entre Cellules (30 sec)
- **Locale** : Un agent ne "voit" que ses 8 voisins immédiats
- **Pas de communication globale** : Chaque agent prend sa décision individuellement
- **Émergence** : Comportement collectif émerge des décisions locales

### 6. Démonstration Live (1-2 min)
Montrer en action :
1. Lancer avec un pattern simple (`Single`)
2. Observer les agents converger vers la sortie
3. Montrer un cas complexe (`ExitObstacle` ou `Labyrinth`)
4. Pointer les comportements intéressants :
   - Files d'attente naturelles
   - Contournement d'obstacles
   - Évitement dynamique entre agents

## Points techniques bonus si temps restant
- **Performance** : Champ de potentiel calculé une seule fois (statique)
- **Scalabilité** : Fonctionne avec des centaines d'agents
- **Modularité** : Patterns d'obstacles facilement extensibles

## Timing estimé : 5-7 minutes
✅ Oui, largement suffisant pour une vidéo structurée et dynamique !

## Conseils pour la vidéo
1. **Commencer** par une démo visuelle immédiate (10 sec)
2. **Expliquer** les concepts en montrant le code pertinent
3. **Alterner** entre théorie et visualisation
4. **Terminer** par le cas le plus impressionnant (Labyrinth avec beaucoup d'agents)