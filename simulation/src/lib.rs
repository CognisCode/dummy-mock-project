use chasers::{Chaser, ChaserType};
use rewards::{Reward, RewardType};

pub mod player;
pub mod genetics;
pub mod simulation;
pub mod helper;
pub mod rewards;
pub mod chasers;
pub mod close_strategy;

pub const SMARTCHASER: usize = 1;
pub const CLOSECHASER: usize = 1; 
pub const HIGHCHASER: usize = 1; 
pub const GENETICCHASER: usize = 1; 
pub const HIGHREWARDS: usize = 100; 
pub const LOWREWARDS: usize = 100; 
pub const MAXITERATIONS: usize = 30; 
pub const HIGHVALUE: i32 = 200;
pub const LOWVALUE: i32 = 50;
pub const PORT: i32 = 9007;
pub const WIDTH: f32 = 2400.0;
pub const HEIGHT: f32 = 1200.0;
pub const SIZE: f32 = 15.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


pub fn create_chaser(count: usize, chaser_type: ChaserType, all_chasers: &mut Vec<Chaser>, color: Color) {
    for _ in 0..count {
        let mut chaser = Chaser::new(chaser_type, color) ;
        all_chasers.push(chaser);
    }
}

pub fn create_reward(id_start: i16, count: usize, reward_type: RewardType, value: i32, all_rewards: &mut Vec<Reward>, color: Color) {
    
    let mut id = id_start;
    for _ in 0..count {
        let mut reward = Reward::new(reward_type, id, value, color);
        id += 1;
        all_rewards.push(reward);
    }
}
