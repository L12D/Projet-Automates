# Projet Automates Cellulaires

Simulation d'évacuation de foule avec modèle Floor-Field en Rust + Macroquad.

## Exécution

```bash
cargo run --release
```

## Contrôles

- **ESPACE** : Pause/Reprendre
- **S** : Avancer d'un pas (en pause)
- **R** : Réinitialiser

## Paramètres

Modifiables dans `src/main.rs` :
- `NUM_AGENTS` : Nombre d'agents (défaut: 100)
- `K_S` : Sensibilité du champ (défaut: 2.0)
- `GRID_WIDTH` / `GRID_HEIGHT` : Dimensions de la grille