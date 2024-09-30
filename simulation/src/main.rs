use std::collections::BTreeMap;

use nannou_osc as osc;
use nannou::prelude::*;

mod player;
use player::{Player, PlayerType};


mod genetics;
use genetics::*;

mod helper;
use helper::*;

fn main() {
    nannou::app(simulation).update(next_step).exit(end).run();
}

const SMARTCHASER: usize = 1;
const CLOSECHASER: usize = 1; 
const HIGHCHASER: usize = 1; 
const GENETICCHASER: usize = 1; 
const HIGHREWARDS: usize = 100; 
const LOWREWARDS: usize = 100; 
const MAXITERATIONS: usize = 30; 
pub const HIGHVALUE: i32 = 200;
pub const LOWVALUE: i32 = 50;

pub struct Simulation {
    all_players_vector: Vec<Player>, 
    progress: Progress,
    scores_left: usize,
    iteration: usize,
    max_iterations: usize,
    sender: osc::Sender<osc::Connected>,
    genetic_tree: BTreeMap<i32, Chromosome>,
}

#[derive(Debug)]
pub struct Progress {
    smart_score: i32,
    high_score: i32,
    close_score: i32,
    genetic_score: i32, 
    smart_start: Vec2,
    high_start: Vec2,
    close_start: Vec2,
    genetic_start: Vec2,
    chromosome: Chromosome,
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

    let progress = Progress {
        close_score: 0,
        high_score: 0,
        smart_score: 0,
        genetic_score: 0,
        smart_start: vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0), 
        high_start: vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0), 
        close_start: vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0),
        genetic_start: vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0),
        chromosome: Chromosome{w_dist: random_f32(),w_value: random_f32(),w_opponent: random_f32()},
    };

    let port = 9007;
    let target_addr = format!("{}:{}", "127.0.0.1", port);

    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");


    create_rewards(1, HIGHREWARDS, PlayerType::HighReward, HIGHVALUE, &mut all_players_vector);
    create_rewards(100, LOWREWARDS, PlayerType::LowReward, LOWVALUE, &mut all_players_vector);

    create_chasers(CLOSECHASER, PlayerType::ChaseClosest, &mut all_players_vector, progress.close_start);
    create_chasers(SMARTCHASER, PlayerType::ChaseSmart, &mut all_players_vector, progress.smart_start);
    create_chasers(HIGHCHASER, PlayerType::ChaseHighest, &mut all_players_vector, progress.high_start);
    create_chasers(GENETICCHASER, PlayerType::Genetic, &mut all_players_vector, progress.genetic_start);

    let genetic_tree = BTreeMap::new();

    Simulation {all_players_vector, progress, scores_left, iteration, max_iterations, sender,genetic_tree}
}

fn next_step(app: &App, simulation: &mut Simulation, _update: Update) {
    // function that determines the progress
    for i in 0..simulation.all_players_vector.len() {
        
        let highrewards: Vec<&Player> = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::HighReward); 
        let lowrewards: Vec<&Player> = simulation.all_players_vector[i].rewards(&simulation.all_players_vector, i, PlayerType::LowReward); 
        let players: Vec<&Player> = simulation.all_players_vector[i].players(&simulation.all_players_vector, i); 
        let other_players: Vec<&Player> = simulation.all_players_vector[i].other_players(&simulation.all_players_vector, i); 

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
            PlayerType::Genetic => {
                let chase_result = simulation.all_players_vector[i].chase_genetic_rewards(&other_players, &highrewards, &lowrewards, &simulation.progress.chromosome);
                simulation.all_players_vector[i].direction += chase_result.direction;
                simulation.all_players_vector[i].target_id = chase_result.target_id;
            }
            PlayerType::HighReward | PlayerType::LowReward => {
                
                if simulation.all_players_vector[i].get_consumed(&players, &mut simulation.progress) {
                    simulation.all_players_vector[i].player_type = PlayerType::Consumed;
                    simulation.scores_left -= 1;
                }
                simulation.all_players_vector[i].direction += vec2(0.0, 0.0);
            }
            _ => {}
        }

        simulation.all_players_vector[i].update(&mut simulation.progress); 

        if simulation.scores_left == 0 {
            reset_all(simulation);
            revaluate(simulation); // insert into binary tree map

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
        osc::Type::Int(simulation.progress.smart_score),
        osc::Type::Int(simulation.progress.high_score),
        osc::Type::Int(simulation.progress.close_score),
        osc::Type::Int(simulation.progress.genetic_score),
        osc::Type::Float(simulation.progress.smart_start[0]),
        osc::Type::Float(simulation.progress.smart_start[1]),
        osc::Type::Float(simulation.progress.high_start[0]),
        osc::Type::Float(simulation.progress.high_start[1]),
        osc::Type::Float(simulation.progress.close_start[0]),
        osc::Type::Float(simulation.progress.close_start[1]),
        osc::Type::Float(simulation.progress.genetic_start[0]),
        osc::Type::Float(simulation.progress.genetic_start[1])
        ]; 

    let packet = (osc_addr, args);
    simulation.sender.send(packet).ok();

    draw.to_frame(app, &frame).unwrap();
}

