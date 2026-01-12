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
        
        Self::compute_distances(&mut distances, &exits, grid);
        
        FloorField { distances }
    }

    pub fn update(&mut self, grid: &Grid, occupied_cells: &[(usize, usize)]) {
        let width = grid.width();
        let height = grid.height();
        
        for row in &mut self.distances {
            for cell in row.iter_mut() {
                *cell = f32::INFINITY;
            }
        }
        
        let mut exits = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if grid.is_exit(x, y) {
                    exits.push((x, y));
                }
            }
        }
        

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
        
        for &(x, y) in exits {
            distances[y][x] = 0.0;
            queue.push_back((x, y, 0.0));
        }
        

        let directions = [
            // Directions cardinales - coût 1.0
            (0, -1, 1.0),    // Haut
            (1,  0, 1.0),    // Gauche
            (0,  1, 1.0),    // Bas
            (-1, 0, 1.0),    // Droite
            // Directions diagonales - coût sqrt(2)
            (1, -1, 1.414),  // Haut-Droite
            (1,  1, 1.414),  // Bas-Gauche
            (-1, 1, 1.414),  // Bas-Droite
            (-1,-1, 1.414),  // Haut-Droite
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
                            let is_occupied = occupied.iter().any(|&(ox, oy)| ox == nx && oy == ny);
                            
                            if cell_type != CellType::Wall && !is_occupied {
                                let near_wall = Self::is_near_wall(nx, ny, grid);
                                if near_wall && new_dist < 10.0 {
                                    new_dist -= 0.3; 
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
        

        let directions = [
            (0, -1),   // Haut
            (1, 0),    // Gauche
            (0, 1),    // Bas
            (-1, 0),   // Droite
            (1, -1),   // Haut-Gauche
            (1, 1),    // Bas-Gauche
            (-1, 1),   // Bas-Droite
            (-1, -1),  // Haut-Droite
        ];
        
        for &(dx, dy) in &directions {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            
            if ny < self.distances.len() && nx < self.distances[0].len() {
                let neighbor_dist = self.distances[ny][nx];
                
                if neighbor_dist < best_dist {
                    best_dist = neighbor_dist;
                    best_dir = Some((dx, dy));
                }
            }
        }
        
        best_dir
    }
}
