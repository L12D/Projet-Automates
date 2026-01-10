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
    
    fn compute_distances(distances: &mut [Vec<f32>], exits: &[(usize, usize)], grid: &Grid) {
        let mut queue = VecDeque::new();
        
        // Initialize with exits at distance 0
        for &(x, y) in exits {
            distances[y][x] = 0.0;
            queue.push_back((x, y, 0.0));
        }
        
        let directions = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
        
        while let Some((x, y, dist)) = queue.pop_front() {
            for (dx, dy) in directions.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if nx < grid.width() && ny < grid.height() {
                        // Calculate new distance (diagonal = sqrt(2), straight = 1)
                        let step_cost = if dx.abs() + dy.abs() == 2 {
                            1.414 // sqrt(2)
                        } else {
                            1.0
                        };
                        let new_dist = dist + step_cost;
                        
                        // Check if cell is walkable (not a wall)
                        if let Some(cell_type) = grid.get(nx, ny) {
                            if cell_type != CellType::Wall && distances[ny][nx] > new_dist {
                                distances[ny][nx] = new_dist;
                                queue.push_back((nx, ny, new_dist));
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn distances(&self) -> &[Vec<f32>] {
        &self.distances
    }
}
