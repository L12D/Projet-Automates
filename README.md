# Projet Automates Cellulaires - Évacuation de Foule

Simulation d'évacuation de foule utilisant un **automate cellulaire** avec **champ de potentiel** (Floor-Field model) en Rust + Macroquad.

## 🎯 Principe

### Automate Cellulaire
- **Grille discrète** : Espace divisé en cellules (Empty, Wall, Agent, Exit)
- **Règles locales** : Chaque agent décide de son mouvement selon son voisinage
- **Évolution synchrone** : Tous les agents se déplacent simultanément à chaque étape

### Champ de Potentiel (Floor-Field)
Le champ de potentiel guide les agents vers les sorties :
- **BFS (Breadth-First Search)** : Calcule la distance minimale de chaque cellule aux sorties
- **Sorties = Distance 0** : Attraction maximale
- **Murs = Distance ∞** : Infranchissables
- **Gradient Descent** : Les agents suivent le gradient (vont vers les distances plus petites)

**Avantages :**
- ✅ Évite naturellement les murs (pas de blocage contre obstacles)
- ✅ Distribution automatique vers les sorties
- ✅ Calcul efficace : O(W×H) pour le champ, O(1) par agent
- ✅ Comportement fluide et réaliste

## 🚀 Exécution

```bash
cargo run --release
```

## Contrôles

- **ESPACE** : Pause/Reprendre
- **S** : Avancer d'un pas (en pause)
- **R** : Réinitialiser

## Paramètres

Modifiables dans `src/main.rs` :
- `NUM_AGENTS` : Nombre d'agents (défaut: 200)
- `K_S` : Sensibilité du champ (défaut: 2.0) - non utilisé en mode déterministe
- `GRID_WIDTH` / `GRID_HEIGHT` : Dimensions de la grille (60×40)
- `STEPS_PER_SECOND` : Vitesse de simulation (30 fps)

## 🔧 Améliorations Apportées

### 1. Mouvement Déterministe (Gradient Descent)
Les agents suivent directement le gradient du champ de potentiel :
- Examine les 8 voisins (priorité aux directions cardinales)
- Choisit la cellule avec la distance minimale
- **Plus stable** que l'approche probabiliste aléatoire

### 2. Résolution des Conflits par Priorité
Quand plusieurs agents veulent la même cellule :
- **Priorité au plus proche de la sortie** (distance minimale)
- Évite les blocages causés par les conflits aléatoires
- Garantit une progression constante

### 3. Ordre Aléatoire d'Évaluation
- Les agents sont traités dans un ordre aléatoire à chaque étape
- Évite les biais de position dans la grille
- Équité entre tous les agents

### 4. Gestion Robuste des Évacuations
- Suppression immédiate des agents qui atteignent une sortie
- Mise à jour correcte de la grille après chaque étape
- Pas de "fuites" d'agents

## 📊 Structure du Code

```
src/
├── main.rs         - Boucle principale et rendu Macroquad
├── grid.rs         - Grille et types de cellules
├── floor_field.rs  - Calcul du champ de potentiel (BFS)
├── agent.rs        - Comportement et mouvement des agents
└── simulation.rs   - Logique de l'automate cellulaire
```

### Algorithmes Clés

**BFS (Breadth-First Search)** dans `floor_field.rs` :
```
1. Initialiser toutes les cellules à distance ∞
2. Mettre les sorties à distance 0
3. Propager par vagues (queue FIFO)
4. Distance(voisin) = Distance(cellule) + 1
```

**Gradient Descent** dans `agent.rs` :
```
Pour chaque agent :
  1. Examiner les 8 cellules voisines
  2. Choisir celle avec distance minimale
  3. Se déplacer si libre, sinon rester
```

**Résolution de Conflits** dans `simulation.rs` :
```
Si plusieurs agents veulent position (x,y) :
  1. Calculer leur distance à la sortie
  2. L'agent le plus proche gagne
  3. Les autres restent sur place
```

## 🎮 Mode Probabiliste (Optionnel)

Pour activer le mode probabiliste, modifiez dans `simulation.rs` :
```rust
use_probabilistic: true  // Au lieu de false
```

Dans ce mode :
- Les agents choisissent selon une probabilité : P = exp(-k_s × distance)
- Plus la distance est petite, plus la probabilité est élevée
- Comportement plus varié mais potentiellement moins stable