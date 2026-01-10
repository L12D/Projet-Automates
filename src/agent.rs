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
    
    /// Get Moore neighborhood (8 directions) - Priorité aux directions cardinales
    pub fn get_neighbors(&self) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        // D'abord les directions cardinales (plus stables)
        let directions = [
            (0, -1),   // Nord
            (1,  0),   // Est
            (0,  1),   // Sud
            (-1, 0),   // Ouest
            (1, -1),   // Nord-Est
            (1,  1),   // Sud-Est
            (-1, 1),   // Sud-Ouest
            (-1,-1),   // Nord-Ouest
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
    
    /// Choisit la meilleure position basée sur le gradient du champ de potentiel
    pub fn choose_next_position(
        &self,
        floor_field: &[Vec<f32>],
        grid_width: usize,
        grid_height: usize,
        is_walkable: impl Fn(usize, usize) -> bool,
    ) -> Option<(usize, usize)> {
        let current_dist = floor_field[self.y][self.x];
        
        // Si on est déjà à l'infini, aucun chemin possible
        if current_dist.is_infinite() {
            return None;
        }
        
        let neighbors = self.get_neighbors();
        let mut best_move = None;
        let mut best_dist = current_dist;
        
        // Chercher le voisin avec la distance minimale (gradient descent)
        for (nx, ny) in neighbors {
            if nx < grid_width && ny < grid_height && is_walkable(nx, ny) {
                let distance = floor_field[ny][nx];
                
                // Suivre le gradient (aller vers la distance plus petite)
                if distance < best_dist {
                    best_dist = distance;
                    best_move = Some((nx, ny));
                }
            }
        }
        
        best_move
    }
    
    /// Version avec probabilités pour un comportement plus naturel (optionnel)
    pub fn choose_next_position_probabilistic(
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
                    // Probabilité inversement proportionnelle à la distance
                    // Plus la distance est petite, plus la probabilité est grande
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
