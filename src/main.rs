mod grid;
mod agent;
mod floor_field;
mod simulation;

use macroquad::prelude::*;
use simulation::Simulation;
use grid::ObstaclePattern;

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;
const CELL_SIZE: f32 = 15.0;
const K_S: f32 = 2.0;
const STEPS_PER_SECOND: f64 = 30.0; 

#[derive(Debug, Clone, Copy)]
struct RoomConfig {
    name: &'static str,
    description: &'static str,
    pattern: ObstaclePattern,
}

const ROOM_CONFIGS: [RoomConfig; 8] = [
    RoomConfig { 
        name: "Salle vide", 
        description: "Aucun obstacle",
        pattern: ObstaclePattern::Empty 
    },
    RoomConfig { 
        name: "Pilier", 
        description: "Un seul obstacle proche de la sortie",
        pattern: ObstaclePattern::Single
    },
    RoomConfig { 
        name: "Pièces multiples", 
        description: "Murs et portes",
        pattern: ObstaclePattern::Rooms 
    },
    RoomConfig { 
        name: "Obstacle sortie", 
        description: "Goulot près de la sortie",
        pattern: ObstaclePattern::ExitObstacle 
    },
    RoomConfig { 
        name: "Multi-obstacles", 
        description: "Plusieurs obstacles dispersés",
        pattern: ObstaclePattern::MultiObstacles 
    },
    RoomConfig { 
        name: "Labyrinthe", 
        description: "Réseau de couloirs",
        pattern: ObstaclePattern::Labyrinth 
    },
    RoomConfig { 
        name: "Deux sorties adjacentes", 
        description: "Deux sorties côte à côte",
        pattern: ObstaclePattern::TwoExitsAdjacent 
    },
    RoomConfig { 
        name: "Deux sorties éloignées", 
        description: "Deux sorties espacées",
        pattern: ObstaclePattern::TwoExitsFar 
    },
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Projet Automate : Évacuation".to_owned(),
        window_width: 920, 
        window_height: 650, 

        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppState {
    Menu,           // Menu de départ
    Simulation,     // Simulation en cours
    Complete,       // Écran de fin
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::Menu;
    let mut selected_room = 0;
    let mut num_agents = 200;
    let mut agent_input = String::new();
    
    let mut simulation: Option<Simulation> = None;
    let mut paused = false;
    let mut step_by_step = false;
    let mut last_step_time = get_time();
    let step_interval = 1.0 / STEPS_PER_SECOND;
    let mut initial_agent_count = 200;
    
    loop {
        clear_background(WHITE);
        
        match app_state {
            AppState::Menu => {
                draw_menu(&mut selected_room, &mut num_agents, &mut agent_input, &mut app_state, &mut simulation, &mut initial_agent_count, &mut last_step_time);
            },
            
            AppState::Simulation => {
                let should_pause = is_key_pressed(KeyCode::Space);
                let should_step = is_key_pressed(KeyCode::S);
                let should_exit = is_key_pressed(KeyCode::Escape);
                
                if should_pause {
                    paused = !paused;
                }
                if should_step {
                    step_by_step = true;
                }
                if should_exit {
                    app_state = AppState::Menu;
                    simulation = None;
                    paused = false;
                }
                
                if let Some(ref mut sim) = simulation {
                    if !paused || step_by_step {
                        let current_time = get_time();
                        if step_by_step || current_time - last_step_time >= step_interval {
                            sim.step();
                            last_step_time = current_time;
                            step_by_step = false;
                        }
                    }
                    
                    let is_complete = sim.agent_count() == 0;
                    if is_complete {
                        app_state = AppState::Complete;
                        paused = true;
                    }
                    
                    sim.draw(CELL_SIZE);
                    
                    draw_text(
                        &format!("Agents: {} | Steps: {}", sim.agent_count(), sim.step_count()),
                        10.0, screen_height() - 50.0, 20.0, BLACK
                    );
                    
                    let room = ROOM_CONFIGS[selected_room];
                    draw_text(
                        &format!("Salle: {} | Population: {}", room.name, num_agents),
                        10.0, screen_height() - 30.0, 18.0, Color::new(0.2, 0.4, 0.8, 1.0)
                    );
                    
                    draw_text(
                        "[SPACE] Pause | [S] Step | [ESC] Menu",
                        10.0, screen_height() - 10.0, 16.0, DARKGRAY
                    );
                    
                    if paused && !is_complete {
                        draw_text("PAUSED", screen_width() / 2.0 - 40.0, 40.0, 30.0, RED);
                    }
                }
            },
            
            AppState::Complete => {
                // ============ ÉCRAN DE FIN ============
                if let Some(ref sim) = simulation {
                    // Afficher la grille finale
                    sim.draw(CELL_SIZE);
                    
                    // Overlay de fin
                    let screen_w = screen_width();
                    let screen_h = screen_height();
                    
                    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.6));
                    
                    let box_w = 500.0;
                    let box_h = 320.0;
                    let box_x = (screen_w - box_w) / 2.0;
                    let box_y = (screen_h - box_h) / 2.0;
                    
                    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.95, 0.95, 0.95, 1.0));
                    draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, Color::new(0.2, 0.7, 0.3, 1.0));
                    
                    draw_text("✓ ÉVACUATION TERMINÉE", box_x + 80.0, box_y + 50.0, 35.0, Color::new(0.2, 0.7, 0.3, 1.0));
                    
                    let stats_x = box_x + 40.0;
                    let mut y_offset = box_y + 100.0;
                    let line_height = 35.0;
                    
                    draw_text(
                        &format!("Agents évacués : {} / {}", initial_agent_count, initial_agent_count),
                        stats_x, y_offset, 25.0, BLACK
                    );
                    y_offset += line_height;
                    
                    draw_text(
                        &format!("Nombre d'étapes : {}", sim.step_count()),
                        stats_x, y_offset, 25.0, BLACK
                    );
                    y_offset += line_height;
                    
                    let efficiency = initial_agent_count as f32 / sim.step_count() as f32;
                    draw_text(
                        &format!("Efficacité : {:.2} agents/étape", efficiency),
                        stats_x, y_offset, 25.0, BLACK
                    );
                    y_offset += line_height + 10.0;
                    
                    draw_rectangle(box_x + 20.0, y_offset, box_w - 40.0, 60.0, Color::new(0.9, 0.95, 0.9, 1.0));
                    draw_text(
                        "Appuyez sur [ENTER] pour retourner au menu",
                        stats_x, y_offset + 35.0, 20.0, Color::new(0.3, 0.3, 0.3, 1.0)
                    );
                    
                    // Retour au menu
                    if is_key_pressed(KeyCode::Enter) {
                        app_state = AppState::Menu;
                        simulation = None;
                        paused = false;
                    }
                }
            },
        }
        
        next_frame().await
    }
}

fn draw_menu(
    selected_room: &mut usize,
    num_agents: &mut usize,
    agent_input: &mut String,
    app_state: &mut AppState,
    simulation: &mut Option<Simulation>,
    initial_agent_count: &mut usize,
    last_step_time: &mut f64,
) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    
    // Fond
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.15, 0.20, 0.30, 1.0));
    
    // Titre
    draw_text("SIMULATION D'ÉVACUATION", screen_w / 2.0 - 250.0, 60.0, 45.0, WHITE);
    draw_text("Configuration de la simulation", screen_w / 2.0 - 180.0, 100.0, 22.0, Color::new(0.8, 0.8, 0.8, 1.0));

    let box_w = 700.0;
    let box_h = 580.0;
    let box_x = (screen_w - box_w) / 2.0;
    let box_y = 130.0;
    
    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.95, 0.95, 0.95, 1.0));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.5, 0.8, 1.0));
    
    draw_text("SÉLECTION DE SALLE", box_x + 220.0, box_y + 35.0, 28.0, Color::new(0.2, 0.4, 0.7, 1.0));
    
    // Navigation avec flèches pour sélectionner les sables
    if is_key_pressed(KeyCode::Up) && *selected_room > 0 {
        *selected_room -= 1;
    }
    if is_key_pressed(KeyCode::Down) && *selected_room < ROOM_CONFIGS.len() - 1 {
        *selected_room += 1;
    }
    
    let mut y_offset = box_y + 70.0;
    for (i, room) in ROOM_CONFIGS.iter().enumerate() {
        let is_selected = i == *selected_room;
        let item_x = box_x + 30.0;
        let item_w = box_w - 60.0;
        let item_h = 40.0;
        
        let bg_color = if is_selected {
            Color::new(0.6, 0.75, 1.0, 1.0)
        } else {
            Color::new(0.85, 0.85, 0.85, 1.0)
        };
        draw_rectangle(item_x, y_offset - 30.0, item_w, item_h, bg_color);
        
        if is_selected {
            draw_rectangle_lines(item_x, y_offset - 30.0, item_w, item_h, 2.0, Color::new(0.2, 0.4, 0.7, 1.0));
        }
        
        let text_color = if is_selected { Color::new(0.0, 0.2, 0.5, 1.0) } else { BLACK };
        
        draw_text(
            &format!("[{}] {} - {}", i + 1, room.name, room.description),
            item_x + 10.0, y_offset, 20.0, text_color
        );
        
        y_offset += 50.0;
    }
    
    // Section nombre d'agents
    draw_line(box_x + 30.0, y_offset, box_x + box_w - 30.0, y_offset, 1.0, GRAY);
    y_offset += 20.0;
    
    draw_text("NOMBRE D'AGENTS", box_x + 240.0, y_offset, 24.0, Color::new(0.2, 0.4, 0.7, 1.0));
    y_offset += 35.0;
    
    // Champ de saisie
    let input_x = box_x + 200.0;
    let input_y = y_offset - 25.0;
    let input_w = 300.0;
    let input_h = 40.0;
    
    draw_rectangle(input_x, input_y, input_w, input_h, WHITE);
    draw_rectangle_lines(input_x, input_y, input_w, input_h, 2.0, Color::new(0.3, 0.5, 0.8, 1.0));
    
    // Afficher la valeur
    let display_text = if agent_input.is_empty() {
        num_agents.to_string()
    } else {
        agent_input.clone()
    };
    draw_text(&display_text, input_x + 10.0, input_y + 28.0, 25.0, BLACK);
    
    // Capture de la saisie
    if let Some(character) = get_char_pressed() {
        if character.is_numeric() && agent_input.len() < 4 {
            agent_input.push(character);
        }
    }
    
    if is_key_pressed(KeyCode::Backspace) {
        if !agent_input.is_empty() {
            agent_input.pop();
        } else {
            *agent_input = num_agents.to_string();
            agent_input.pop();
        }
    }
    
    // Valider avec Enter
    if is_key_pressed(KeyCode::Enter) {
        // Valider la saisie
        if !agent_input.is_empty() {
            if let Ok(n) = agent_input.parse::<usize>() {
                if n > 0 && n <= 1000 {
                    *num_agents = n;
                }
            }
            agent_input.clear();
        }
        
        // Créer la simulation
        let room = ROOM_CONFIGS[*selected_room];
        *simulation = Some(Simulation::new_with_pattern(
            GRID_WIDTH,
            GRID_HEIGHT,
            *num_agents,
            K_S,
            room.pattern
        ));
        *initial_agent_count = *num_agents;
        *last_step_time = get_time();
        *app_state = AppState::Simulation;
    }
    
    let instructions_y = box_y + box_h + 25.0;
    draw_text(
        "[up or down] pour Sélectionner salle | Taper le nombre d'agents | [ENTER] Démarrer",
        box_x + 60.0, instructions_y, 18.0, WHITE
    );
}
