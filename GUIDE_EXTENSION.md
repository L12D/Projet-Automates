# Guide d'Extension et Personnalisation

## 🎨 Personnalisation Visuelle

### Changer les Couleurs

Dans `src/grid.rs`, méthode `draw()` :
```rust
let color = match self.cells[y][x] {
    CellType::Empty => Color::new(0.95, 0.95, 0.95, 1.0), // Gris clair
    CellType::Wall => Color::new(0.2, 0.2, 0.2, 1.0),     // Gris foncé
    CellType::Agent => Color::new(0.2, 0.5, 0.9, 1.0),    // Bleu
    CellType::Exit => Color::new(0.2, 0.8, 0.2, 1.0),     // Vert
};
```

**Suggestions :**
- Agents en rouge pour urgence : `Color::new(0.9, 0.2, 0.2, 1.0)`
- Exit en jaune-orange : `Color::new(1.0, 0.7, 0.0, 1.0)`
- Dégradé selon densité pour les agents

### Visualiser le Champ de Potentiel

Ajouter dans `src/grid.rs` :
```rust
pub fn draw_with_potential(&self, cell_size: f32, floor_field: &FloorField) {
    let max_dist = floor_field.distances().iter()
        .flat_map(|row| row.iter())
        .filter(|&&d| d.is_finite())
        .fold(0.0f32, |max, &d| max.max(d));
    
    for y in 0..self.height {
        for x in 0..self.width {
            let px = x as f32 * cell_size;
            let py = y as f32 * cell_size;
            
            if self.cells[y][x] == CellType::Empty {
                let dist = floor_field.distances()[y][x];
                if dist.is_finite() {
                    let intensity = 1.0 - (dist / max_dist);
                    let color = Color::new(0.2, intensity, 0.2, 0.5);
                    draw_rectangle(px, py, cell_size, cell_size, color);
                }
            }
            // ... reste du code
        }
    }
}
```

## 🏗️ Modifications de la Grille

### Ajouter des Obstacles Personnalisés

Dans `src/grid.rs`, méthode `add_obstacles()` :

```rust
fn add_obstacles(&mut self) {
    // Exemple 1 : Ligne de piliers
    for x in (5..55).step_by(10) {
        for dy in -1..=1 {
            let y = (self.height / 2) as i32 + dy;
            if y >= 0 && y < self.height as i32 {
                self.cells[y as usize][x] = CellType::Wall;
            }
        }
    }
    
    // Exemple 2 : Couloir étroit
    let corridor_start = self.width / 4;
    let corridor_end = 3 * self.width / 4;
    for x in corridor_start..corridor_end {
        self.cells[self.height / 3][x] = CellType::Wall;
        self.cells[2 * self.height / 3][x] = CellType::Wall;
    }
    // Laisser des passages
    for gap in [corridor_start + 10, corridor_end - 10] {
        self.cells[self.height / 3][gap] = CellType::Empty;
        self.cells[2 * self.height / 3][gap] = CellType::Empty;
    }
    
    // Exemple 3 : Salle avec colonnes
    for y in (10..30).step_by(8) {
        for x in (15..45).step_by(10) {
            self.cells[y][x] = CellType::Wall;
        }
    }
}
```

### Charger une Grille depuis un Fichier

Créer `src/grid_loader.rs` :
```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::grid::{Grid, CellType};

pub fn load_from_file(path: &str) -> Result<Grid, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut cells = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
        
        for ch in line.chars() {
            let cell = match ch {
                '#' => CellType::Wall,
                'E' => CellType::Exit,
                'A' => CellType::Agent,
                _ => CellType::Empty,
            };
            row.push(cell);
        }
        
        if !row.is_empty() {
            cells.push(row);
        }
    }
    
    let height = cells.len();
    let width = cells[0].len();
    
    Ok(Grid::from_cells(width, height, cells))
}
```

Format du fichier `map.txt` :
```
###########################
#                         #
#  ###      ###      ###  #
#                         #
#                         #
#  ###      ###      ###  #
#                         #
#                        E#
###########################
```

## 🤖 Comportements Avancés des Agents

### 1. Agents avec Vitesse Variable

Dans `src/agent.rs` :
```rust
#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub x: usize,
    pub y: usize,
    pub speed: f32,  // 0.5 = lent, 1.0 = normal, 2.0 = rapide
    pub steps_to_wait: usize,  // Compteur interne
}

impl Agent {
    pub fn can_move_this_step(&mut self) -> bool {
        if self.steps_to_wait > 0 {
            self.steps_to_wait -= 1;
            false
        } else {
            // Reset selon la vitesse
            self.steps_to_wait = (1.0 / self.speed) as usize;
            true
        }
    }
}
```

Dans `simulation.rs` :
```rust
for &i in &indices {
    if !self.agents[i].can_move_this_step() {
        continue;  // Cet agent ne bouge pas ce tour
    }
    // ... reste de la logique
}
```

### 2. Groupes d'Agents (Familles)

```rust
#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub x: usize,
    pub y: usize,
    pub group_id: Option<usize>,  // None = seul, Some(id) = en groupe
}

impl Agent {
    pub fn choose_next_position_with_group(
        &self,
        floor_field: &[Vec<f32>],
        group_members: &[Agent],  // Autres membres du groupe
        // ... autres params
    ) -> Option<(usize, usize)> {
        // Trouver la position moyenne du groupe
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        for member in group_members {
            avg_x += member.x as f32;
            avg_y += member.y as f32;
        }
        avg_x /= group_members.len() as f32;
        avg_y /= group_members.len() as f32;
        
        // Pondérer : 70% gradient, 30% cohésion du groupe
        // ... logique de mouvement hybride
    }
}
```

### 3. Comportement de Panique

```rust
impl Agent {
    pub fn calculate_panic_level(&self, local_density: f32) -> f32 {
        // Panique si densité > 0.6
        if local_density > 0.6 {
            (local_density - 0.6) * 2.5  // 0.0 à 1.0
        } else {
            0.0
        }
    }
    
    pub fn choose_with_panic(
        &self,
        floor_field: &[Vec<f32>],
        panic_level: f32,
        // ...
    ) -> Option<(usize, usize)> {
        if panic_level > 0.7 {
            // En panique : mouvement plus aléatoire
            if rand::random::<f32>() < panic_level {
                return self.choose_random_valid_move(...);
            }
        }
        // Sinon, mouvement normal
        self.choose_next_position(...)
    }
}
```

## 📊 Collecte de Statistiques

### Créer un Module de Métriques

`src/metrics.rs` :
```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct SimulationMetrics {
    pub start_time: Instant,
    pub evacuation_times: Vec<Duration>,  // Temps par agent
    pub density_history: Vec<f32>,  // Densité moyenne par étape
    pub bottlenecks: HashMap<(usize, usize), usize>,  // Compteur par cellule
}

impl SimulationMetrics {
    pub fn new() -> Self {
        SimulationMetrics {
            start_time: Instant::now(),
            evacuation_times: Vec::new(),
            density_history: Vec::new(),
            bottlenecks: HashMap::new(),
        }
    }
    
    pub fn record_evacuation(&mut self, agent_id: usize) {
        let elapsed = self.start_time.elapsed();
        self.evacuation_times.push(elapsed);
    }
    
    pub fn record_density(&mut self, agents_count: usize, total_cells: usize) {
        let density = agents_count as f32 / total_cells as f32;
        self.density_history.push(density);
    }
    
    pub fn record_position(&mut self, x: usize, y: usize) {
        *self.bottlenecks.entry((x, y)).or_insert(0) += 1;
    }
    
    pub fn get_average_evacuation_time(&self) -> Duration {
        let total: Duration = self.evacuation_times.iter().sum();
        total / self.evacuation_times.len() as u32
    }
    
    pub fn get_bottlenecks(&self, threshold: usize) -> Vec<((usize, usize), usize)> {
        self.bottlenecks.iter()
            .filter(|(_, &count)| count > threshold)
            .map(|(&pos, &count)| (pos, count))
            .collect()
    }
    
    pub fn export_to_csv(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(filename)?;
        writeln!(file, "step,density")?;
        
        for (step, density) in self.density_history.iter().enumerate() {
            writeln!(file, "{},{}", step, density)?;
        }
        
        Ok(())
    }
}
```

Intégrer dans `simulation.rs` :
```rust
pub struct Simulation {
    // ... champs existants
    pub metrics: SimulationMetrics,
}

pub fn step(&mut self) {
    // ... logique existante
    
    // Enregistrer les métriques
    self.metrics.record_density(
        self.agents.len(), 
        self.grid.width() * self.grid.height()
    );
    
    for agent in &self.agents {
        self.metrics.record_position(agent.x, agent.y);
    }
    
    // Lors d'une évacuation
    if evacuated {
        self.metrics.record_evacuation(agent_id);
    }
}
```

## 🔬 Tests et Benchmarks

### Tests Unitaires

`src/tests/floor_field_tests.rs` :
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bfs_calculates_correct_distances() {
        let grid = Grid::new(10, 10);
        let floor_field = FloorField::new(&grid);
        
        // Vérifier que la sortie a distance 0
        assert_eq!(floor_field.distances()[5][9], 0.0);
        
        // Vérifier que les voisins ont distance 1
        assert_eq!(floor_field.distances()[4][9], 1.0);
        assert_eq!(floor_field.distances()[6][9], 1.0);
    }
    
    #[test]
    fn test_gradient_descent_finds_best_direction() {
        let agent = Agent::new(5, 5, 0);
        // ... tester que l'agent choisit la bonne direction
    }
}
```

### Benchmarks avec Criterion

`Cargo.toml` :
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "simulation_bench"
harness = false
```

`benches/simulation_bench.rs` :
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use automates_evacuation::simulation::Simulation;

fn benchmark_step(c: &mut Criterion) {
    let mut sim = Simulation::new(60, 40, 200, 2.0);
    
    c.bench_function("simulation_step", |b| {
        b.iter(|| {
            sim.step();
        })
    });
}

criterion_group!(benches, benchmark_step);
criterion_main!(benches);
```

Exécuter : `cargo bench`

## 🎮 Contrôles Supplémentaires

Dans `main.rs` :
```rust
// Ajouter ces contrôles
if is_key_pressed(KeyCode::D) {
    simulation.toggle_debug_view();  // Afficher le champ de potentiel
}

if is_key_pressed(KeyCode::Up) {
    simulation.increase_speed();  // Accélérer
}

if is_key_pressed(KeyCode::Down) {
    simulation.decrease_speed();  // Ralentir
}

if is_key_pressed(KeyCode::O) {
    simulation.add_random_obstacle();  // Ajouter obstacle dynamique
}

if is_key_pressed(KeyCode::P) {
    simulation.toggle_probabilistic_mode();  // Basculer mode
}
```

## 📦 Export et Replay

### Sauvegarder l'État

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SimulationState {
    pub agents: Vec<Agent>,
    pub step: usize,
    pub grid: SerializableGrid,
}

impl Simulation {
    pub fn save_state(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let state = SimulationState {
            agents: self.agents.clone(),
            step: self.step_count,
            grid: self.grid.to_serializable(),
        };
        
        let json = serde_json::to_string_pretty(&state)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
    
    pub fn load_state(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filename)?;
        let state: SimulationState = serde_json::from_str(&json)?;
        // Reconstruire la simulation
        Ok(simulation)
    }
}
```

## 🚀 Optimisations Avancées

### Parallélisation avec Rayon

`Cargo.toml` :
```toml
[dependencies]
rayon = "1.7"
```

`src/simulation.rs` :
```rust
use rayon::prelude::*;

// Calculer les mouvements en parallèle
let desired_moves: Vec<_> = indices.par_iter()
    .map(|&i| {
        let agent = &self.agents[i];
        (i, agent.choose_next_position(...))
    })
    .collect();
```

### Cache du Champ de Potentiel

```rust
pub struct Simulation {
    // ...
    floor_field_cache: Option<FloorField>,
    need_recalculate: bool,
}

impl Simulation {
    pub fn step(&mut self) {
        // Ne recalculer que si nécessaire
        if self.need_recalculate {
            self.floor_field = FloorField::new(&self.grid);
            self.need_recalculate = false;
        }
        // ... reste
    }
    
    pub fn add_obstacle(&mut self, x: usize, y: usize) {
        self.grid.set(x, y, CellType::Wall);
        self.need_recalculate = true;  // Marquer pour recalcul
    }
}
```

Bon développement ! 🦀
