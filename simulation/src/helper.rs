use super::player::{Player, PlayerType};
use nannou::prelude::*;
use super::Simulation;
use crate::{Progress, HIGHVALUE, LOWVALUE, HIGHREWARDS, LOWREWARDS};

use super::genetics::*;

pub fn create_player(id: i16, player_type: PlayerType, value: i32, pos_x: f32, pos_y: f32) -> Player {

    let player = Player::new(
        pos_x, 
        pos_y,
        15.0,
        player_type,
        id,
        value,
    );    
    player
}

pub fn create_rewards(id_start: i16, count: usize, player_type: PlayerType, value: i32, all_players: &mut Vec<Player>) {
    
    let mut id = id_start;
    for _ in 0..count {
        let player = create_player(id, player_type.clone(), value, (random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
        id += 1;
        all_players.push(player);
    }
}

pub  fn create_chasers(count: usize, player_type: PlayerType, all_players: &mut Vec<Player>, start: Vec2) {

    for _ in 0..count {
        let player = create_player(0, player_type.clone(), 0, start[0], start[1]) ;
        all_players.push(player);
    }
}

pub fn reset_all(simulation: &mut Simulation) {
    
    let mut high_reward_count = 0; 
    simulation.iteration += 1;
    simulation.scores_left =  HIGHREWARDS + LOWREWARDS;
    
    simulation.progress = Progress {
        smart_score: 0,
        high_score: 0,
        close_score: 0,
        genetic_score: 0,
        smart_start: vec2(0.0, 0.0),
        high_start: vec2(0.0, 0.0),
        close_start: vec2(0.0, 0.0),
        genetic_start: vec2(0.0, 0.0),
        chromosome: simulation.progress.chromosome, // leave unchanged until reevaluate
    };

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
            let start_x: f32 = (random_f32() - 0.5) * 2400.0;
            let start_y: f32 = (random_f32() - 0.5) * 1200.0;

            simulation.all_players_vector[i].target_id = 0;
            simulation.all_players_vector[i].position = vec2(start_x, start_y);

            match simulation.all_players_vector[i].player_type {
                PlayerType::ChaseClosest => simulation.progress.close_start = vec2(start_x, start_y),
                PlayerType::ChaseHighest => simulation.progress.high_start = vec2(start_x, start_y),
                PlayerType::ChaseSmart => simulation.progress.smart_start = vec2(start_x, start_y),
                PlayerType::Genetic => simulation.progress.genetic_start = vec2(start_x, start_y),

                _ => {},
            }

        }
    }
}

pub fn end(_app: &App, _model: Simulation) {
    println!("Exiting application...");
}

