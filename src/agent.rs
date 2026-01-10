use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub x: usize,
    pub y: usize,
}

impl Agent {
    pub fn new(x: usize, y: usize, _id: usize) -> Self {
        Agent { x, y }
    }
    
    /// Get Moore neighborhood (8 directions)
    pub fn get_neighbors(&self) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
        
        for (dx, dy) in directions.iter() {
            let new_x = self.x as i32 + dx;
            let new_y = self.y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }
        
        neighbors
    }
    
    /// Choose next position based on floor field probabilities
    pub fn choose_next_position(
        &self,
        floor_field: &[Vec<f32>],
        grid_width: usize,
        grid_height: usize,
        is_walkable: impl Fn(usize, usize) -> bool,
        k_s: f32,
    ) -> Option<(usize, usize)> {
        let neighbors = self.get_neighbors();
        let mut valid_moves = Vec::new();
        let mut probabilities = Vec::new();
        let mut total_prob = 0.0;
        
        for (nx, ny) in neighbors {
            if nx < grid_width && ny < grid_height && is_walkable(nx, ny) {
                let distance = floor_field[ny][nx];
                
                // Skip if distance is infinite (unreachable)
                if distance.is_finite() {
                    // Probability: P = exp(-k_s * S)
                    let prob = (-k_s * distance).exp();
                    valid_moves.push((nx, ny));
                    probabilities.push(prob);
                    total_prob += prob;
                }
            }
        }
        
        // If no valid moves, stay in place
        if valid_moves.is_empty() {
            return None;
        }
        
        // Normalize probabilities and choose randomly
        let mut rng = rand::thread_rng();
        let mut roll: f32 = rng.gen::<f32>() * total_prob;
        
        for (i, &prob) in probabilities.iter().enumerate() {
            roll -= prob;
            if roll <= 0.0 {
                return Some(valid_moves[i]);
            }
        }
        
        // Fallback to last valid move
        valid_moves.last().copied()
    }
}
