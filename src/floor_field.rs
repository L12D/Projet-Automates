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
}
