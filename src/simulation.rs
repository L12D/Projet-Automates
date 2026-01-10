use crate::agent::Agent;
use crate::floor_field::FloorField;
use crate::grid::{CellType, Grid};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

pub struct Simulation {
    grid: Grid,
    floor_field: FloorField,
    agents: Vec<Agent>,
    k_s: f32,
    step_count: usize,
    use_probabilistic: bool,
}

impl Simulation {
    pub fn new(width: usize, height: usize, num_agents: usize, k_s: f32) -> Self {
        let mut grid = Grid::new(width, height);
        let floor_field = FloorField::new(&grid);
        
        // Place agents randomly in empty cells
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        let mut placed = 0;
        
        while placed < num_agents {
            let x = rng.gen_range(1..width - 1);
            let y = rng.gen_range(1..height - 1);
            
            if grid.is_empty(x, y) {
                let agent = Agent::new(x, y, placed);
                grid.set(x, y, CellType::Agent);
                agents.push(agent);
                placed += 1;
            }
        }
        
        Simulation {
            grid,
            floor_field,
            agents,
            k_s,
            step_count: 0,
            use_probabilistic: false, // Mode déterministe par défaut (plus stable)
        }
    }
    
    pub fn step(&mut self) {
        if self.agents.is_empty() {
            return;
        }
        
        self.step_count += 1;
        
        // Ordre aléatoire pour éviter les biais
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..self.agents.len()).collect();
        indices.shuffle(&mut rng);
        
        // Phase 1: Déterminer les mouvements désirés
        let mut desired_moves: HashMap<usize, Option<(usize, usize)>> = HashMap::new();
        
        // Variation temporelle : pas tous les agents ne bougent à chaque pas
        let time_factor = (self.step_count as f32 * 0.1).sin();
        
        for &i in &indices {
            let agent = &self.agents[i];
            
            // Chaque agent a une "phase" légèrement différente
            let should_move = (time_factor + agent.phase_offset * 6.28).sin() > -0.3;
            
            if !should_move && rng.gen::<f32>() < 0.3 { // 30% de chance de ne pas bouger
                desired_moves.insert(i, None);
                continue;
            }
            
            let next_pos = if self.use_probabilistic {
                agent.choose_next_position_probabilistic(
                    self.floor_field.distances(),
                    self.grid.width(),
                    self.grid.height(),
                    |x, y| self.grid.is_walkable(x, y) || (x == agent.x && y == agent.y),
                    self.k_s,
                )
            } else {
                // Mode déterministe : suit directement le gradient
                agent.choose_next_position(
                    self.floor_field.distances(),
                    self.grid.width(),
                    self.grid.height(),
                    |x, y| self.grid.is_walkable(x, y) || (x == agent.x && y == agent.y),
                )
            };
            
            desired_moves.insert(i, next_pos);
        }
        
        // Phase 2: Résolution des conflits avec priorité
        let mut target_counts: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        
        for (&i, &next_pos) in &desired_moves {
            if let Some(pos) = next_pos {
                target_counts.entry(pos).or_insert_with(Vec::new).push(i);
            }
        }
        
        // Phase 3: Effectuer les mouvements
        let mut evacuated_indices = Vec::new();
        let mut moved = vec![false; self.agents.len()];
        
        // D'abord, effacer toutes les positions actuelles
        for agent in &self.agents {
            self.grid.set(agent.x, agent.y, CellType::Empty);
        }
        
        // Traiter les mouvements par ordre de priorité
        for &i in &indices {
            if let Some(next_pos) = desired_moves[&i] {
                let (nx, ny) = next_pos;
                
                // Vérifier les conflits
                let conflicts = target_counts.get(&(nx, ny)).map(|v| v.len()).unwrap_or(0);
                
                if conflicts == 1 {
                    // Pas de conflit, mouvement garanti
                    self.agents[i].x = nx;
                    self.agents[i].y = ny;
                    moved[i] = true;
                    
                    // Vérifier si l'agent atteint la sortie
                    if self.grid.is_exit(nx, ny) {
                        evacuated_indices.push(i);
                    }
                } else if conflicts > 1 {
                    // Conflit : priorité au plus proche de la sortie
                    let contestants = target_counts.get(&(nx, ny)).unwrap();
                    let mut best_agent = i;
                    let mut best_dist = self.floor_field.distances()[self.agents[i].y][self.agents[i].x];
                    
                    for &contestant in contestants {
                        let dist = self.floor_field.distances()[self.agents[contestant].y][self.agents[contestant].x];
                        if dist < best_dist {
                            best_dist = dist;
                            best_agent = contestant;
                        }
                    }
                    
                    // Seul le meilleur bouge
                    if best_agent == i {
                        self.agents[i].x = nx;
                        self.agents[i].y = ny;
                        moved[i] = true;
                        
                        if self.grid.is_exit(nx, ny) {
                            evacuated_indices.push(i);
                        }
                    }
                }
            }
        }
        
        // Retirer les agents évacués (en ordre décroissant pour éviter les décalages d'index)
        evacuated_indices.sort_by(|a, b| b.cmp(a));
        for i in evacuated_indices {
            self.agents.remove(i);
        }
        
        // Remettre les agents sur la grille
        for agent in &self.agents {
            self.grid.set(agent.x, agent.y, CellType::Agent);
        }
    }
    
    pub fn draw(&self, cell_size: f32) {
        self.grid.draw(cell_size);
    }
    
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
    
    pub fn step_count(&self) -> usize {
        self.step_count
    }
}
