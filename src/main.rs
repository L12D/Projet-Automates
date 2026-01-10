mod grid;
mod agent;
mod floor_field;
mod simulation;

use macroquad::prelude::*;
use simulation::Simulation;

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;
const CELL_SIZE: f32 = 15.0;
const NUM_AGENTS: usize = 200;
const K_S: f32 = 2.0; // Sensitivity parameter
const STEPS_PER_SECOND: f64 = 30.0; // Simulation speed

fn window_conf() -> Conf {
    Conf {
        window_title: "Floor-Field Evacuation".to_owned(),
        window_width: 920,  // Set your desired width
        window_height: 650, // Set your desired height
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new(GRID_WIDTH, GRID_HEIGHT, NUM_AGENTS, K_S);
    
    let mut paused = false;
    let mut step_by_step = false;
    let mut last_step_time = get_time();
    let step_interval = 1.0 / STEPS_PER_SECOND;
    
    loop {
        clear_background(WHITE);
        
        // Handle input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::S) {
            step_by_step = true;
        }
        if is_key_pressed(KeyCode::R) {
            simulation = Simulation::new(GRID_WIDTH, GRID_HEIGHT, NUM_AGENTS, K_S);
            last_step_time = get_time();
            paused = false;
        }
        
        // Update simulation
        if !paused || step_by_step {
            let current_time = get_time();
            if step_by_step || current_time - last_step_time >= step_interval {
                simulation.step();
                last_step_time = current_time;
                step_by_step = false;
            }
        }
        
        // Render
        simulation.draw(CELL_SIZE);
        
        // Display info
        draw_text(
            &format!("Agents: {} | Steps: {}", 
                simulation.agent_count(), 
                simulation.step_count()
            ),
            10.0, screen_height() - 30.0, 20.0, BLACK
        );
        
        draw_text(
            "Controls: [SPACE] Pause | [S] Step | [R] Reset",
            10.0, screen_height() - 10.0, 16.0, DARKGRAY
        );
        
        if paused {
            draw_text("PAUSED", screen_width() / 2.0 - 40.0, 40.0, 30.0, RED);
        }
        
        next_frame().await
    }
}
