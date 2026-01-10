use crate::agent::Agent;
use crate::floor_field::FloorField;
use crate::grid::{CellType, Grid};
use rand::Rng;
use std::collections::HashSet;

pub struct Simulation {
    grid: Grid,
    floor_field: FloorField,
    agents: Vec<Agent>,
    k_s: f32,
    step_count: usize,
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
        }
    }
    
    pub fn step(&mut self) {
        if self.agents.is_empty() {
            return;
        }
        
        self.step_count += 1;
        
        // Phase 1: Each agent chooses next position
        let mut desired_positions = Vec::new();
        for agent in &self.agents {
            let next_pos = agent.choose_next_position(
                self.floor_field.distances(),
                self.grid.width(),
                self.grid.height(),
                |x, y| {
                    self.grid.is_walkable(x, y) || (x == agent.x && y == agent.y)
                },
                self.k_s,
            );
            desired_positions.push(next_pos);
        }
        
        // Phase 2: Resolve conflicts
        let mut occupied = HashSet::new();
        let mut new_agents = Vec::new();
        
        // First pass: check for conflicts
        let mut conflicts: Vec<Vec<usize>> = vec![Vec::new(); self.agents.len()];
        for (i, &next_pos) in desired_positions.iter().enumerate() {
            if let Some(pos) = next_pos {
                for (j, &other_pos) in desired_positions.iter().enumerate() {
                    if i != j && other_pos == Some(pos) {
                        conflicts[i].push(j);
                    }
                }
            }
        }
        
        // Second pass: move agents
        let mut rng = rand::thread_rng();
        for (i, agent) in self.agents.iter().enumerate() {
            // Clear current position
            self.grid.set(agent.x, agent.y, CellType::Empty);
            
            if let Some((nx, ny)) = desired_positions[i] {
                // Check if there's a conflict
                if !conflicts[i].is_empty() {
                    // Randomly decide if this agent wins the conflict
                    if rng.gen_bool(1.0 / (conflicts[i].len() + 1) as f64) && !occupied.contains(&(nx, ny)) {
                        occupied.insert((nx, ny));
                        
                        // Check if agent reached exit
                        if self.grid.is_exit(nx, ny) {
                            // Agent evacuated, don't add to new_agents
                            continue;
                        }
                        
                        let mut new_agent = *agent;
                        new_agent.x = nx;
                        new_agent.y = ny;
                        new_agents.push(new_agent);
                    } else {
                        // Lost conflict, stay in place
                        new_agents.push(*agent);
                    }
                } else {
                    // No conflict
                    if !occupied.contains(&(nx, ny)) {
                        occupied.insert((nx, ny));
                        
                        // Check if agent reached exit
                        if self.grid.is_exit(nx, ny) {
                            // Agent evacuated, don't add to new_agents
                            continue;
                        }
                        
                        let mut new_agent = *agent;
                        new_agent.x = nx;
                        new_agent.y = ny;
                        new_agents.push(new_agent);
                    } else {
                        // Position already taken, stay in place
                        new_agents.push(*agent);
                    }
                }
            } else {
                // No valid move, stay in place
                new_agents.push(*agent);
            }
        }
        
        // Update agents list
        self.agents = new_agents;
        
        // Update grid with new agent positions
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
