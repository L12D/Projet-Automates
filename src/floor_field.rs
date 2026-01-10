use std::collections::VecDeque;
use crate::grid::{Grid, CellType};

pub struct FloorField {
    distances: Vec<Vec<f32>>,
}

impl FloorField {
    pub fn new(grid: &Grid) -> Self {
        let width = grid.width();
        let height = grid.height();
        let mut distances = vec![vec![f32::INFINITY; width]; height];
        
        // Find all exit cells
        let mut exits = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if grid.is_exit(x, y) {
                    exits.push((x, y));
                }
            }
        }
        
        // BFS from all exit cells simultaneously
        Self::compute_distances(&mut distances, &exits, grid);
        
        FloorField { distances }
    }
    
    /// Recalcule le champ en tenant compte des agents comme obstacles temporaires
    pub fn update(&mut self, grid: &Grid, occupied_cells: &[(usize, usize)]) {
        let width = grid.width();
        let height = grid.height();
        
        // Réinitialiser
        for row in &mut self.distances {
            for cell in row.iter_mut() {
                *cell = f32::INFINITY;
            }
        }
        
        // Find exits
        let mut exits = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if grid.is_exit(x, y) {
                    exits.push((x, y));
                }
            }
        }
        
        // BFS avec agents comme obstacles temporaires
        Self::compute_distances_with_agents(&mut self.distances, &exits, grid, occupied_cells);
    }
    
    fn compute_distances(distances: &mut [Vec<f32>], exits: &[(usize, usize)], grid: &Grid) {
        Self::compute_distances_with_agents(distances, exits, grid, &[]);
    }
    
    fn compute_distances_with_agents(
        distances: &mut [Vec<f32>], 
        exits: &[(usize, usize)], 
        grid: &Grid,
        occupied: &[(usize, usize)]
    ) {
        let mut queue = VecDeque::new();
        
        // Initialize with exits at distance 0
        for &(x, y) in exits {
            distances[y][x] = 0.0;
            queue.push_back((x, y, 0.0));
        }
        
        // Moore neighborhood (8 directions) avec coûts euclidiens
        // Coût 1.0 pour les directions cardinales, sqrt(2) ≈ 1.414 pour les diagonales
        let directions = [
            // Directions cardinales - coût 1.0
            (0, -1, 1.0),    // Nord
            (1,  0, 1.0),    // Est
            (0,  1, 1.0),    // Sud
            (-1, 0, 1.0),    // Ouest
            // Directions diagonales - coût sqrt(2)
            (1, -1, 1.414),  // Nord-Est
            (1,  1, 1.414),  // Sud-Est
            (-1, 1, 1.414),  // Sud-Ouest
            (-1,-1, 1.414),  // Nord-Ouest
        ];
        
        while let Some((x, y, dist)) = queue.pop_front() {
            for &(dx, dy, cost) in directions.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if nx < grid.width() && ny < grid.height() {
                        let mut new_dist = dist + cost;
                        
                        // Vérifier si la cellule est marchable
                        if let Some(cell_type) = grid.get(nx, ny) {
                            // Pas un mur et pas occupée par un autre agent
                            let is_occupied = occupied.iter().any(|&(ox, oy)| ox == nx && oy == ny);
                            
                            if cell_type != CellType::Wall && !is_occupied {
                                // Bonus si la cellule est adjacente à un mur (effet bordure)
                                // Simule le comportement de longer les murs près de la sortie
                                let near_wall = Self::is_near_wall(nx, ny, grid);
                                if near_wall && new_dist < 10.0 { // Seulement près de la sortie
                                    new_dist -= 0.3; // Petit bonus d'attraction
                                }
                                
                                if distances[ny][nx] > new_dist {
                                    distances[ny][nx] = new_dist;
                                    queue.push_back((nx, ny, new_dist));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn is_near_wall(x: usize, y: usize, grid: &Grid) -> bool {
        let neighbors = [
            (0, -1), (1, 0), (0, 1), (-1, 0),
            (1, -1), (1, 1), (-1, 1), (-1, -1),
        ];
        
        for (dx, dy) in neighbors.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                
                if let Some(cell_type) = grid.get(nx, ny) {
                    if cell_type == CellType::Wall {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    pub fn distances(&self) -> &[Vec<f32>] {
        &self.distances
    }
    
    /// Trouve la meilleure direction basée sur le gradient
    pub fn get_best_direction(&self, x: usize, y: usize) -> Option<(i32, i32)> {
        let current_dist = self.distances[y][x];
        
        if current_dist.is_infinite() {
            return None;
        }
        
        let mut best_dir = None;
        let mut best_dist = current_dist;
        
        // Priorité aux directions cardinales (évite les diagonales qui peuvent bloquer)
        let directions = [
            (0, -1),   // Nord
            (1, 0),    // Est
            (0, 1),    // Sud
            (-1, 0),   // Ouest
            (1, -1),   // Nord-Est
            (1, 1),    // Sud-Est
            (-1, 1),   // Sud-Ouest
            (-1, -1),  // Nord-Ouest
        ];
        
        for &(dx, dy) in &directions {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            
            if ny < self.distances.len() && nx < self.distances[0].len() {
                let neighbor_dist = self.distances[ny][nx];
                
                // Suit le gradient (descente vers distance minimale)
                if neighbor_dist < best_dist {
                    best_dist = neighbor_dist;
                    best_dir = Some((dx, dy));
                }
            }
        }
        
        best_dir
    }
}
