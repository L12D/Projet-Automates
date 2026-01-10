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
    let mut finish_time = None;
    let initial_agents = NUM_AGENTS;
    
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
            finish_time = None;
        }
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;  // Quitter
        }
        
        // Update simulation (sauf si terminée)
        if !simulation.is_finished() {
            if !paused || step_by_step {
                let current_time = get_time();
                if step_by_step || current_time - last_step_time >= step_interval {
                    simulation.step();
                    last_step_time = current_time;
                    step_by_step = false;
                }
            }
        } else if finish_time.is_none() {
            // Enregistrer le temps de fin
            finish_time = Some(get_time());
        }
        
        // Render
        simulation.draw(CELL_SIZE);
        
        // Display info
        if !simulation.is_finished() {
            draw_text(
                &format!("Agents: {} | Étapes: {}", 
                    simulation.agent_count(), 
                    simulation.step_count()
                ),
                10.0, screen_height() - 30.0, 20.0, BLACK
            );
        } else {
            // Afficher les statistiques finales
            let efficiency = initial_agents as f64 / simulation.step_count() as f64;
            
            draw_rectangle(
                screen_width() / 2.0 - 250.0,
                screen_height() / 2.0 - 100.0,
                500.0,
                200.0,
                Color::new(0.9, 0.9, 0.9, 0.95)
            );
            
            draw_rectangle_lines(
                screen_width() / 2.0 - 250.0,
                screen_height() / 2.0 - 100.0,
                500.0,
                200.0,
                3.0,
                Color::new(0.2, 0.7, 0.2, 1.0)
            );
            
            draw_text(
                "✅ ÉVACUATION TERMINÉE !",
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 - 60.0,
                30.0,
                Color::new(0.2, 0.7, 0.2, 1.0)
            );
            
            draw_text(
                &format!("🧍 Agents évacués : {}", initial_agents),
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 - 20.0,
                22.0,
                BLACK
            );
            
            draw_text(
                &format!("📊 Nombre d'étapes : {}", simulation.step_count()),
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 + 10.0,
                22.0,
                BLACK
            );
            
            draw_text(
                &format!("⚡ Efficacité : {:.2} agents/étape", efficiency),
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 + 40.0,
                22.0,
                BLACK
            );
            
            draw_text(
                "Appuyez sur [R] pour recommencer ou [Q/ESC] pour quitter",
                screen_width() / 2.0 - 220.0,
                screen_height() / 2.0 + 80.0,
                18.0,
                DARKGRAY
            );
        }
        
        draw_text(
            "Contrôles: [ESPACE] Pause | [S] Step | [R] Reset | [Q/ESC] Quitter",
            10.0, screen_height() - 10.0, 16.0, DARKGRAY
        );
        
        if paused && !simulation.is_finished() {
            draw_text("⏸ PAUSE", screen_width() / 2.0 - 50.0, 40.0, 30.0, RED);
        }
        
        next_frame().await
    }
}
