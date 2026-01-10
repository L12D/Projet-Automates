use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Wall,
    Agent,
    Exit,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellType>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![CellType::Empty; width]; height];
        let mut grid = Grid {
            width,
            height,
            cells,
        };
        
        // Initialize with walls on borders
        grid.initialize_walls();
        
        // Add exit on the right side
        grid.add_exit();
        
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
        
        // Add some internal obstacles
        self.add_obstacles();
    }
    
    fn add_obstacles(&mut self) {
        // Add a few pillars to make it more interesting
        let mid_height = self.height / 2;
        let quarter_width = self.width / 4;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                let y = (mid_height as i32 + dy) as usize;
                let x = (quarter_width as i32 + dx) as usize;
                if y < self.height && x < self.width {
                    self.cells[y][x] = CellType::Wall;
                }
            }
        }
    }
    
    fn add_exit(&mut self) {
        // Exit on the right wall, in the middle
        let exit_y = self.height / 2;
        for dy in -1..=1 {
            let y = (exit_y as i32 + dy) as usize;
            if y < self.height {
                self.cells[y][self.width - 1] = CellType::Exit;
            }
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
