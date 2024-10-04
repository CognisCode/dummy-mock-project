// use super::player::{Player, PlayerType};
// use nannou::prelude::*;
// use crate::simulation::{Simulation};
// use crate::*;
// use std::cell::RefCell;


// pub fn create_player(id: i16, player_type: PlayerType, value: i32, pos_x: f32, pos_y: f32, score: RefCell<i32>, color: Color) -> Player {

//     let player = Player::new(
//         pos_x, 
//         pos_y,
//         player_type,
//         id,
//         value,
//         score,
//         color
//     );    
//     player
// }

// pub fn create_rewards(id_start: i16, count: usize, player_type: PlayerType, value: i32, all_players: &mut Vec<Player>, color: Color) {
    
//     let mut id = id_start;
//     for _ in 0..count {
//         let player = create_player(id, player_type.clone(), value, (random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0, RefCell::new(0), color);
//         id += 1;
//         all_players.push(player);
//     }
// }

// pub  fn create_chasers(count: usize, player_type: PlayerType, all_players: &mut Vec<Player>, start: Vec2, color: Color) {

//     for _ in 0..count {
//         let player = create_player(0, player_type.clone(), 0, start[0], start[1], RefCell::new(0) , color) ;
//         all_players.push(player);
//     }
// }




