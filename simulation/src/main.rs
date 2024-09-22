use std::sync::{Arc, Mutex};
use std::thread;

use nannou::prelude::*;

mod player;
use player::{Player, PlayerType};

fn main() {
    let mut chaser_score: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    // this will be a separate thread
    nannou::app(simulation).update(next_step).run();
}

const SMARTCHASER: usize = 1;
const CLOSECHASER: usize = 1; 
const HIGHCHASER: usize = 1; 
const HIGHREWARDS: usize = 50; 
const LOWREWARDS: usize = 50; 
 
pub const HIGHVALUE: f32 = 200.0;
pub const LOWVALUE: f32 = 50.0;

struct Simulation {
    all_players_vector: Vec<Player>, 
}

fn simulation(app: &App) -> Simulation {

        // Initialize the window
        app.new_window()
        .size(2400, 1200)
        .view(view)
        .build()
        .unwrap();

    let mut all_players_vector = Vec::new();

    // Create HighReward and LowReward
    create_players(1, HIGHREWARDS, PlayerType::HighReward, HIGHVALUE, &mut all_players_vector);
    create_players(100, LOWREWARDS, PlayerType::LowReward, LOWVALUE, &mut all_players_vector);

    // Create chasers
    create_chasers(HIGHCHASER, PlayerType::ChaseHighest, &mut all_players_vector);
    create_chasers(CLOSECHASER, PlayerType::ChaseClosest, &mut all_players_vector);
    create_chasers(SMARTCHASER, PlayerType::ChaseSmart, &mut all_players_vector);

    Simulation { all_players_vector }
}

fn next_step(app: &App, simulation: &mut Simulation, _update: Update) {

    for i in 0..simulation.all_players_vector.len() {
        
        let highrewards = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::HighReward); 
        let lowrewards = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::LowReward); 
        let players = simulation.all_players_vector[i].players(&simulation.all_players_vector, i); 

        match simulation.all_players_vector[i].player_type {
            PlayerType::ChaseHighest => {
                let chase_result = simulation.all_players_vector[i].chase_highest_rewards(&highrewards, &lowrewards);
                simulation.all_players_vector[i].direction += chase_result.direction;
                simulation.all_players_vector[i].target_id = chase_result.target_id;
            }
            PlayerType::ChaseClosest => {
                let chase_result = simulation.all_players_vector[i].chase_closest_rewards(&highrewards, &lowrewards);
                simulation.all_players_vector[i].direction += chase_result.direction;
                simulation.all_players_vector[i].target_id = chase_result.target_id;
            }
            PlayerType::ChaseSmart => {
                let chase_result = simulation.all_players_vector[i].chase_smart_rewards(&highrewards, &lowrewards);
                simulation.all_players_vector[i].direction += chase_result.direction;
                simulation.all_players_vector[i].target_id = chase_result.target_id;
            }
            PlayerType::HighReward | PlayerType::LowReward => {
                
                if simulation.all_players_vector[i].get_consumed(&players) {
                    simulation.all_players_vector[i].player_type = PlayerType::Consumed;
                    // update score
                }
                simulation.all_players_vector[i].direction += vec2(0.0, 0.0);
            }
            _ => {}
        }

        simulation.all_players_vector[i].update(); 
        // prevent players to leave the screen
        let screen_right = app.window_rect().right() as f32; // half of width pixels
        let screen_top = app.window_rect().top() as f32; // half of height pixels
        simulation.all_players_vector[i].edge(screen_top, screen_right);
    }
}

fn view(app: &App, model: &Simulation, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for i in 0..model.all_players_vector.len() {
        model.all_players_vector[i].show(&draw); 
    }
    draw.to_frame(app, &frame).unwrap();
}

fn create_player(id: i16, player_type: PlayerType, value: f32) -> Player {
    let player = Player::new(
        (random_f32() - 0.5) * 1800.0, 
        (random_f32() - 0.5) * 800.0,
        20.0,
        player_type,
        id,
        value
    );    
    player
}

fn create_players(id_start: i16, count: usize, player_type: PlayerType, value: f32, all_players: &mut Vec<Player>) {
    let mut id = id_start;
    for _ in 0..count {
        let player = create_player(id, player_type.clone(), value);
        id += 1;
        all_players.push(player);
    }
}

fn create_chasers(count: usize, player_type: PlayerType, all_players: &mut Vec<Player>) {
    for _ in 0..count {
        let player = create_player(0, player_type.clone(), 0.0);
        all_players.push(player);
    }
}