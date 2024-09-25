// #![allow(unused)] // silence unused warnings while exploring (to comment out)

use nannou_osc as osc;
use nannou::prelude::*;
mod player;
use player::{Player, PlayerType};

fn main() {
    nannou::app(simulation).update(next_step).exit(end).run();
}

const SMARTCHASER: usize = 1;
const CLOSECHASER: usize = 1; 
const HIGHCHASER: usize = 1; 
const HIGHREWARDS: usize = 50; 
const LOWREWARDS: usize = 50; 
const MAXITERATIONS: usize = 3; 

pub const HIGHVALUE: f32 = 200.0;
pub const LOWVALUE: f32 = 50.0;

struct Simulation {
    all_players_vector: Vec<Player>, 
    chaser_scores: Scores,
    scores_left: usize,
    iteration: usize,
    max_iterations: usize,
    sender: osc::Sender<osc::Connected>,
}

#[derive(Debug)]
pub struct Scores {
    smart_score: f32,
    high_score: f32,
    close_score: f32
}

fn simulation(app: &App) -> Simulation {

    // Initialize the window
    app.new_window()
        .size(2400, 1200)
        .view(view)
        .build()
        .unwrap();

    let mut all_players_vector: Vec<Player> = Vec::new();
    let iteration = 0;
    let max_iterations = MAXITERATIONS;
    let scores_left = HIGHREWARDS + LOWREWARDS;

    let chaser_scores = Scores {
        close_score: 0.0,
        high_score: 0.0,
        smart_score: 0.0,
    };

    let port = 9007;
    let target_addr = format!("{}:{}", "127.0.0.1", port);

    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");



    // Create HighReward and LowReward
    create_rewards(1, HIGHREWARDS, PlayerType::HighReward, HIGHVALUE, &mut all_players_vector);
    create_rewards(100, LOWREWARDS, PlayerType::LowReward, LOWVALUE, &mut all_players_vector);

    // Create chasers
    create_chasers(HIGHCHASER, PlayerType::ChaseHighest, &mut all_players_vector);
    create_chasers(CLOSECHASER, PlayerType::ChaseClosest, &mut all_players_vector);
    create_chasers(SMARTCHASER, PlayerType::ChaseSmart, &mut all_players_vector);

    Simulation {all_players_vector, chaser_scores, scores_left, iteration, max_iterations, sender}
}

fn next_step(app: &App, simulation: &mut Simulation, _update: Update) {
    
    for i in 0..simulation.all_players_vector.len() {
        
        let highrewards: Vec<&Player> = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::HighReward); 
        let lowrewards: Vec<&Player> = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::LowReward); 
        let players: Vec<&Player> = simulation.all_players_vector[i].players(&simulation.all_players_vector, i); 

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
                
                if simulation.all_players_vector[i].get_consumed(&players, &mut simulation.chaser_scores) {
                    simulation.all_players_vector[i].player_type = PlayerType::Consumed;
                    simulation.scores_left -= 1;
                }
                simulation.all_players_vector[i].direction += vec2(0.0, 0.0);
            }
            _ => {}
        }

        println!("Chaser Scores: {:?}", simulation.chaser_scores);

        simulation.all_players_vector[i].update(); 

        if simulation.scores_left == 0 {
            reset_all(simulation);
        }

        if simulation.iteration == simulation.max_iterations {
            app.quit();
            return; // critical for the app to gracefully close
        }

        // prevent players to leave the screen, not needed in current setup
        // let screen_right = app.window_rect().right() as f32; // half of width pixels
        // let screen_top = app.window_rect().top() as f32; // half of height pixels
        // simulation.all_players_vector[i].edge(screen_top, screen_right);
    }
}



fn view(app: &App, simulation: &Simulation, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BEIGE);

    for i in 0..simulation.all_players_vector.len() {
        simulation.all_players_vector[i].show(&draw); 
    }

    let osc_addr = "/simulation/scores".to_string();
    let args = vec![
        osc::Type::Float(simulation.chaser_scores.smart_score),
        osc::Type::Float(simulation.chaser_scores.high_score),
        osc::Type::Float(simulation.chaser_scores.close_score)]; 
    let packet = (osc_addr, args);
    simulation.sender.send(packet).ok();

    draw.to_frame(app, &frame).unwrap();
}

fn create_player(id: i16, player_type: PlayerType, value: f32, pos_x: f32, pos_y: f32) -> Player {

    let player = Player::new(
        pos_x, 
        pos_y,
        15.0,
        player_type,
        id,
        value
    );    
    player
}

fn create_rewards(id_start: i16, count: usize, player_type: PlayerType, value: f32, all_players: &mut Vec<Player>) {
    
    let mut id = id_start;
    for _ in 0..count {
        let player = create_player(id, player_type.clone(), value, (random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
        id += 1;
        all_players.push(player);
    }
}

fn create_chasers(count: usize, player_type: PlayerType, all_players: &mut Vec<Player>) {
    for _ in 0..count {
        let player = create_player(0, player_type.clone(), 0.0, (random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
        all_players.push(player);
    }
}

fn reset_all(simulation: &mut Simulation) {
    
    let mut high_reward_count = 0; 

    for i in 0..simulation.all_players_vector.len() {
        
        if simulation.all_players_vector[i].player_type == PlayerType::Consumed {
            
            if high_reward_count < HIGHREWARDS {
                simulation.all_players_vector[i].player_type = PlayerType::HighReward;
                simulation.all_players_vector[i].position = vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
                simulation.all_players_vector[i].value = HIGHVALUE;
                high_reward_count += 1;
            } else {
                simulation.all_players_vector[i].player_type = PlayerType::LowReward;
                simulation.all_players_vector[i].position = vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
                simulation.all_players_vector[i].value = LOWVALUE;
            }
        } else {
            simulation.all_players_vector[i].target_id = 0;
            simulation.all_players_vector[i].position = vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
        }
    }

    simulation.iteration += 1;
    simulation.scores_left =  HIGHREWARDS + LOWREWARDS;
    simulation.chaser_scores = Scores {
        close_score: 0.0,
        high_score: 0.0,
        smart_score: 0.0,
    };
}

fn end(_app: &App, _model: Simulation) {
    println!("Exiting application...");
}