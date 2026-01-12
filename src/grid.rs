use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Wall,
    Agent,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstaclePattern {
    Empty,           // Salle vide
    Single,          // Un pilier central
    Rooms,           // Plusieurs pièces avec portes
    ExitObstacle,    // Obstacle très proche de la sortie
    MultiObstacles,  // Plusieurs obstacles dispersés
    Labyrinth,       // Labyrinthe simple
    TwoExitsAdjacent, // Deux sorties adjacentes sur le mur droit // Non utilisé
    TwoExitsFar,     // Deux sorties éloignées sur le mur droit // Non utilisé
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellType>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_pattern(width, height, ObstaclePattern::Single)
    }
    
    pub fn new_with_pattern(width: usize, height: usize, pattern: ObstaclePattern) -> Self {
        let cells = vec![vec![CellType::Empty; width]; height];
        let mut grid = Grid {
            width,
            height,
            cells,
        };
        
        // Initialize with walls on borders
        grid.initialize_walls();
        
        // Add obstacles based on pattern
        grid.add_obstacles_pattern(pattern);
        
        // Add exit(s) on the right side based on pattern
        match pattern {
            ObstaclePattern::TwoExitsAdjacent | ObstaclePattern::TwoExitsFar => {
                grid.add_two_exits(pattern);
            }
            _ => {
                grid.add_exit();
            }
        }
        
        grid
    }
    
    fn initialize_walls(&mut self) {
        // Top and bottom walls
        for x in 0..self.width {
            self.cells[0][x] = CellType::Wall;
            self.cells[self.height - 1][x] = CellType::Wall;
        }
        
        // Left and right walls (except for exit)
        for y in 0..self.height {
            self.cells[y][0] = CellType::Wall;
            self.cells[y][self.width - 1] = CellType::Wall;
        }
    }
    
    fn add_obstacles_pattern(&mut self, pattern: ObstaclePattern) {
        match pattern {
            ObstaclePattern::Single => {
                // Un seul pilier
                let mid_height = self.height / 2;
                let quarter_width = self.width *5 / 6;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let y = (mid_height as i32 + dy) as usize;
                        let x = (quarter_width as i32 + dx) as usize;
                        if y < self.height && x < self.width {
                            self.cells[y][x] = CellType::Wall;
                        }
                    }
                }
            },
            
            ObstaclePattern::ExitObstacle => {
                let exit_y = self.height / 2;
                let obstacle_x = self.width - 8; 
                
                // Grand obstacle bloquant
                for dy in -5i32..=5 {
                    for dx in -3i32..=3 {
                        let y = exit_y as i32 + dy;
                        let x = obstacle_x as i32 + dx;
                        if y >= 0 && x >= 0 && (y as usize) < self.height && (x as usize) < self.width {
                            self.cells[y as usize][x as usize] = CellType::Wall;
                        }
                    }
                }
                
                for offset in 6..9 {
                    if exit_y >= offset {
                        for dx in 0..4 {
                            if obstacle_x + dx < self.width {
                                self.cells[exit_y - offset][obstacle_x + dx] = CellType::Empty;
                            }
                        }
                    }
                    if exit_y + offset < self.height {
                        for dx in 0..4 {
                            if obstacle_x + dx < self.width {
                                self.cells[exit_y + offset][obstacle_x + dx] = CellType::Empty;
                            }
                        }
                    }
                }
            },
            
            ObstaclePattern::MultiObstacles => {
                // Plusieurs obstacles de tailles variées dispersés
                let obstacles = [
                    (self.width / 5, self.height / 4, 2),      // (x, y, taille)
                    (self.width / 5, 3 * self.height / 4, 2),
                    (2 * self.width / 5, self.height / 2, 3),
                    (3 * self.width / 5, self.height / 3, 1),
                    (3 * self.width / 5, 2 * self.height / 3, 2),
                    (4 * self.width / 5, self.height / 2, 1),
                ];
                
                for (px, py, size) in obstacles.iter() {
                    let size = *size as i32;
                    for dy in -size..=size {
                        for dx in -size..=size {
                            let y = *py as i32 + dy;
                            let x = *px as i32 + dx;
                            if y >= 0 && x >= 0 && (y as usize) < self.height && (x as usize) < self.width {
                                self.cells[y as usize][x as usize] = CellType::Wall;
                            }
                        }
                    }
                }
            },
            
            ObstaclePattern::Rooms => {
                // Pièces avec portes
                let mid_x = self.width / 2;
                let mid_y = self.height / 2;
                
                // Mur au milieu
                for y in 0..self.height-1 {
                    self.cells[y][mid_x] = CellType::Wall;
                }
                
                // Portes dans le mur
                for dy in -2i32..=2 {
                    let y1 = (self.height / 3) as i32 + dy;
                    let y2 = (2 * self.height / 3) as i32 + dy;
                    if y1 >= 0 && (y1 as usize) < self.height {
                        self.cells[y1 as usize][mid_x] = CellType::Empty;
                    }
                    if y2 >= 0 && (y2 as usize) < self.height {
                        self.cells[y2 as usize][mid_x] = CellType::Empty;
                    }
                }
                
                // Obstacles dans chaque pièce
                let obstacles = [
                    (mid_x / 2, mid_y / 2),
                    (mid_x / 2, mid_y + mid_y / 2),
                ];
                
                for (ox, oy) in obstacles.iter() {
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            let y = (*oy as i32 + dy);
                            let x = (*ox as i32 + dx);
                            if y >= 0 && x >= 0 && (y as usize) < self.height && (x as usize) < self.width {
                                self.cells[y as usize][x as usize] = CellType::Wall;
                            }
                        }
                    }
                }
            },
            
            ObstaclePattern::Empty => {
                // Pas d'obstacles internes
            },
            
            ObstaclePattern::Labyrinth => {
                // Labyrinthe simple avec couloirs
                for y in 8..self.height-8 {
                    if y % 8 == 0 {
                        for x in 8..self.width-8 {
                            if x % 12 != 6 { // Laisser des passages
                                self.cells[y][x] = CellType::Wall;
                            }
                        }
                    }
                }
                
                // Murs verticaux
                for x in 8..self.width-8 {
                    if x % 12 == 0 {
                        for y in 8..self.height-8 {
                            if y % 8 != 4 { // Passages décalés
                                self.cells[y][x] = CellType::Wall;
                            }
                        }
                    }
                }
            },
            
            ObstaclePattern::TwoExitsAdjacent => {
                // Voir add_two_exits()
            },
            
            ObstaclePattern::TwoExitsFar => {
                // Voir add_two_exits()
            },
        }
    }
    
    fn add_exit(&mut self) {
        // Exit on the right wall, in the middle
        let exit_y = self.height / 2;
        for dy in -1i32..=1 {
            let y = (exit_y as i32 + dy);
            if y >= 0 && (y as usize) < self.height {
                self.cells[y as usize][self.width - 1] = CellType::Exit;
            }
        }
    }
    
    fn add_two_exits(&mut self, pattern: ObstaclePattern) {
        match pattern {
            ObstaclePattern::TwoExitsAdjacent => {
                let center_y = self.height / 2;
                
                for dy in 0..2 {
                    let y = center_y - 3 + dy;
                    if y < self.height {
                        self.cells[y][self.width - 1] = CellType::Exit;
                    }
                }
                
                for dy in 0..2 {
                    let y = center_y + 1 + dy;
                    if y < self.height {
                        self.cells[y][self.width - 1] = CellType::Exit;
                    }
                }
            },
            
            ObstaclePattern::TwoExitsFar => {
                let quarter_height = self.height / 4;

                for dy in 0..2 {
                    let y = quarter_height - 1 + dy;
                    if y < self.height {
                        self.cells[y][self.width - 1] = CellType::Exit;
                    }
                }

                for dy in 0..2 {
                    let y = 3 * quarter_height - 1 + dy;
                    if y < self.height {
                        self.cells[y][self.width - 1] = CellType::Exit;
                    }
                }
            },
            
            _ => {}
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<CellType> {
        if x < self.width && y < self.height {
            Some(self.cells[y][x])
        } else {
            None
        }
    }
    
    pub fn set(&mut self, x: usize, y: usize, cell_type: CellType) {
        if x < self.width && y < self.height {
            self.cells[y][x] = cell_type;
        }
    }
    
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.get(x, y), Some(CellType::Empty))
    }
    
    pub fn is_exit(&self, x: usize, y: usize) -> bool {
        matches!(self.get(x, y), Some(CellType::Exit))
    }
    
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        matches!(self.get(x, y), Some(CellType::Empty | CellType::Exit))
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn draw(&self, cell_size: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let px = x as f32 * cell_size;
                let py = y as f32 * cell_size;
                
                let color = match self.cells[y][x] {
                    CellType::Empty => Color::new(0.95, 0.95, 0.95, 1.0),
                    CellType::Wall => Color::new(0.2, 0.2, 0.2, 1.0),
                    CellType::Agent => Color::new(0.2, 0.5, 0.9, 1.0),
                    CellType::Exit => Color::new(0.2, 0.8, 0.2, 1.0),
                };
                
                draw_rectangle(px, py, cell_size, cell_size, color);
                draw_rectangle_lines(px, py, cell_size, cell_size, 0.5, Color::new(0.8, 0.8, 0.8, 1.0));
            }
        }
    }
}
