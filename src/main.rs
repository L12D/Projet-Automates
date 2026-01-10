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
    let mut simulation_complete = false;
    let mut completion_time: Option<f64> = None;
    let initial_agent_count = NUM_AGENTS;
    
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
            simulation_complete = false;
            completion_time = None;
        }
        
        // Check if simulation is complete
        if !simulation_complete && simulation.agent_count() == 0 {
            simulation_complete = true;
            completion_time = Some(get_time());
            paused = true; // Auto-pause when complete
        }
        
        // Update simulation (only if not complete)
        if !simulation_complete && (!paused || step_by_step) {
            let current_time = get_time();
            if step_by_step || current_time - last_step_time >= step_interval {
                simulation.step();
                last_step_time = current_time;
                step_by_step = false;
            }
        }
        
        // Render
        simulation.draw(CELL_SIZE);
        
        // Display completion screen overlay
        if simulation_complete {
            let screen_w = screen_width();
            let screen_h = screen_height();
            
            // Semi-transparent overlay
            draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.6));
            
            // Main completion box
            let box_w = 500.0;
            let box_h = 320.0;
            let box_x = (screen_w - box_w) / 2.0;
            let box_y = (screen_h - box_h) / 2.0;
            
            draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.95, 0.95, 0.95, 1.0));
            draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, Color::new(0.2, 0.7, 0.3, 1.0));
            
            // Title
            let title = "✓ ÉVACUATION TERMINÉE";
            draw_text(title, box_x + 80.0, box_y + 50.0, 35.0, Color::new(0.2, 0.7, 0.3, 1.0));
            
            // Statistics
            let stats_x = box_x + 40.0;
            let mut y_offset = box_y + 100.0;
            let line_height = 35.0;
            
            draw_text(
                &format!("Agents évacués : {} / {}", initial_agent_count, initial_agent_count),
                stats_x, y_offset, 25.0, BLACK
            );
            y_offset += line_height;
            
            draw_text(
                &format!("Nombre d'étapes : {}", simulation.step_count()),
                stats_x, y_offset, 25.0, BLACK
            );
            y_offset += line_height;
            
            let efficiency = initial_agent_count as f32 / simulation.step_count() as f32;
            draw_text(
                &format!("Efficacité : {:.2} agents/étape", efficiency),
                stats_x, y_offset, 25.0, BLACK
            );
            y_offset += line_height + 10.0;
            
            // Controls
            draw_rectangle(box_x + 20.0, y_offset, box_w - 40.0, 60.0, Color::new(0.9, 0.95, 0.9, 1.0));
            draw_text(
                "Appuyez sur [R] pour recommencer",
                stats_x, y_offset + 35.0, 22.0, Color::new(0.3, 0.3, 0.3, 1.0)
            );
        }
        
        // Display info (only if not complete)
        if !simulation_complete {
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
        }
        
        next_frame().await
    }
}
