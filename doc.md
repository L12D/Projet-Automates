# Projet Automates - Documentation

## Overview
Evacuation simulation using cellular automata and floor field models. Built with Rust and Macroquad.

## Project Files

### Source Files

#### `main.rs`
Entry point of the application. Handles:
- Window configuration and main event loop
- Menu interface for room selection and agent count
- Simulation state management (menu, running, complete)
- Rendering of grid, agents, and UI elements
- Pause/step-by-step controls

#### `agent.rs`
Defines the `Agent` struct and movement logic:
- Moore neighborhood navigation (8 directions)
- Position selection based on floor field gradients
- Randomized movement with phase offsets to prevent synchronization
- Probabilistic movement option for more natural behavior

#### `floor_field.rs`
Implements the `FloorField` struct for pathfinding:
- Computes distance field from exits using breadth-first search
- Considers cardinal and diagonal movements with proper costs
- Wall avoidance preference for natural path selection
- Dynamic field updates based on agent positions

#### `grid.rs`
Grid structure and obstacle patterns:
- Cell types: Empty, Wall, Agent, Exit
- 8 predefined room patterns (empty, single pillar, rooms, labyrinth, etc.)
- Grid initialization with borders and exits
- Cell state management and rendering

#### `simulation.rs`
Simulation engine that orchestrates the evacuation:
- Agent initialization with random placement
- Step-by-step simulation with conflict resolution
- Dynamic agent movement based on floor field
- Wave-like movement patterns using phase offsets
- Statistics tracking (step count, evacuation time)

### Configuration

#### `Cargo.toml`
Rust package manifest:
- Project name: `automates-evacuation`
- Dependencies: `macroquad` (graphics), `rand` (randomization)
- Optimization levels for dev and release builds
