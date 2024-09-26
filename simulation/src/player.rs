use std::ops::Sub;
use nannou::prelude::*;
use crate::{Progress, HIGHVALUE, LOWVALUE};

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerType {
    ChaseClosest,
    ChaseHighest,
    ChaseSmart,
    Consumed,
    HighReward,
    LowReward
}

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Vec2,
    pub angle_vec: Vec2,
    pub player_type: PlayerType,
    pub direction: Vec2,
    size: f32, 
    max_step_size: f32,
    pub id: i16,
    pub value: f32,
    pub target_id: i16
}

pub struct TargetGoal {
    pub direction: Vec2,
    pub target_id: i16,
}

impl Player {
    pub fn new(pos_x: f32, pos_y: f32, size: f32, player_type: PlayerType, id: i16, value: f32) -> Player {
        
        let max_step_size = match player_type {
            PlayerType::ChaseHighest => {3.0},
            PlayerType::ChaseClosest => {3.0},
            PlayerType::ChaseSmart => {3.0},
            PlayerType::Consumed => {0.0},
            PlayerType::HighReward => {0.0},
            PlayerType::LowReward => {0.0},
        };
     
        Player {
                position: vec2(pos_x, pos_y),
                angle_vec: vec2(0.0, 0.0),
                size,
                direction: vec2(0.0, 0.0),
                max_step_size,
                player_type,
                id,
                value,
                target_id: 0
        }
    }

    pub fn show(&self, draw: &Draw) {

        let red;
        let green;
        let blue;
        
        match self.player_type {
            PlayerType::ChaseHighest => {
                red = 255.0;
                green = 0.0;
                blue = 0.0;
            },
            PlayerType::ChaseClosest => {
                red = 139.0;
                green = 0.0;
                blue = 139.0;
            },
            PlayerType::HighReward => {
                red = 0.0;
                green = 255.0;
                blue = 0.0;
            },
            PlayerType::LowReward => {
                red = 0.0;
                green = 0.0;
                blue = 255.0;
            },
            PlayerType::ChaseSmart => {
                red = 0.0;
                green = 0.0;
                blue = 0.0;
            },
            PlayerType::Consumed => {
                red = 255.0;
                green = 255.0;
                blue = 0.0;
            }
        };
        
        if self.player_type != PlayerType::HighReward && self.player_type != PlayerType::LowReward && self.player_type != PlayerType::Consumed {
            draw.tri()
                .w_h(self.size, self.size)
                .x_y(self.position.x, self.position.y)
                .rotate(self.angle_vec.angle()) // angle of the triangle
                .rgba(red, green, blue, 0.85);
        } else {
            draw.ellipse()
                .w_h(self.size, self.size)
                .x_y(self.position.x, self.position.y)
                .rgba(red, green, blue, 0.85);
        }
    }

    pub fn update(&mut self, chaser_scores: &mut Progress) {

        if self.player_type != PlayerType::HighReward && self.player_type != PlayerType::LowReward && self.player_type != PlayerType::Consumed {
            
            self.position += self.direction.normalize() * 10.0;  

            // angle vector is previous direction + new direction and then normalized to keep te step size equal to max step size
            self.angle_vec += self.direction;
            self.angle_vec = self.angle_vec.clamp_length(self.max_step_size,self.max_step_size);
        
        
            match self.player_type{ 
                PlayerType::ChaseClosest => {chaser_scores.close_start = self.position},
                PlayerType::ChaseHighest => {chaser_scores.high_start = self.position},
                PlayerType::ChaseSmart => {chaser_scores.smart_start = self.position},
                _  => {}
            }        
        }
        self.direction = vec2(0.0, 0.0); // reset accelartion
    }

    // pub fn edge(&mut self, top: f32, right: f32) {
    //     if self.position.x > right {
    //         self.position.x = -right;
    //     } else if self.position.x < -right {
    //         self.position.x = right
    //     }
    //     if self.position.y > top {
    //         self.position.y = -top;
    //     } else if self.position.y < -top {
    //         self.position.y = top
    //     }
    // }

    pub fn rewards<'a>(&self, all_players: &'a Vec<Player>, player_index: usize, player_type: PlayerType) -> Vec<&'a Player> {
        
        let mut rewards = Vec::new();

        for i in 0..all_players.len() {

            if i != player_index  && all_players[i].player_type == player_type {
                rewards.push(&all_players[i]);
            }
        }
        rewards
    }

    pub fn players<'a>(&self, all_players: &'a Vec<Player>, player_index: usize) -> Vec<&'a Player> {
        
        let mut players = Vec::new();

        for i in 0..all_players.len() {

            if i != player_index  && all_players[i].player_type != PlayerType::HighReward && all_players[i].player_type != PlayerType::Consumed && all_players[i].player_type != PlayerType::LowReward {
                players.push(&all_players[i]);
            }
        }
        players
    }

    pub fn get_consumed(&self, chasers: &Vec<&Player>, chaser_scores: &mut Progress) -> bool {

        let len = chasers.len();

        if len == 0 {
            return false;
        }
        
        for chaser in chasers {

            if self.position.distance(chaser.position) < 5.0 && self.id == chaser.target_id {
                
                let mut reward: f32 = 0.0;

                match self.player_type {
                    PlayerType::HighReward => {reward = HIGHVALUE},
                    PlayerType::LowReward => {reward = LOWVALUE},
                    _ => {println!("something went wrong at the consuming part")}
                }
                match chaser.player_type {
                    PlayerType::ChaseSmart => {chaser_scores.smart_score += reward;},
                    PlayerType::ChaseHighest => {chaser_scores.high_score += reward;},
                    PlayerType::ChaseClosest => {chaser_scores.close_score += reward;},
                    _ => {}
                } 
                return true;
            }
        }
        false
    }

    pub fn chase_closest_rewards(&self, highrewards: &Vec<&Player>, lowrewards: &Vec<&Player>) -> TargetGoal {

        if highrewards.len() + lowrewards.len() == 0 { 
           return TargetGoal{direction: vec2(0.0, 0.0), target_id: 0} 
        }
    
        // find nearest reward
        let mut shortest_distance = 100000000.0;
        let mut direction = Vec2::new(0.0, 0.0);

        let mut target_id = 0;
        // find closest target
        for reward in highrewards {
            if reward.position.distance(self.position) < shortest_distance {    
                direction = reward.position.sub(self.position).normalize();
                shortest_distance = reward.position.distance(self.position);
                target_id = reward.id; 
            }   
        }

        for reward in lowrewards {
            if reward.position.distance(self.position) < shortest_distance {    
                direction = reward.position.sub(self.position).normalize();
                shortest_distance = reward.position.distance(self.position);
                target_id = reward.id; 
            }   
        }

        return TargetGoal{direction: Vec2::new(direction.x , direction.y ), target_id}         
    }

    pub fn chase_highest_rewards(&self, highrewards: &Vec<&Player>, lowrewards: &Vec<&Player>) -> TargetGoal {
        // return closest reward coordinates
        let rewards: &Vec<&Player>;

        if highrewards.len() != 0 { 
            rewards = highrewards;
        } else if lowrewards.len() != 0{
            rewards = lowrewards;
        } else {
            return TargetGoal{direction: vec2(0.0, 0.0), target_id: 0} 
        }
        
        let mut shortest_distance = 100000000.0;
        let mut direction = Vec2::new(0.0, 0.0);
        let mut target_id = 0;

        for reward in rewards {

            if reward.position.distance(self.position) < shortest_distance {    
                direction = reward.position.sub(self.position).normalize();
                shortest_distance = reward.position.distance(self.position);
                target_id = reward.id; 
            }   
        }
        return TargetGoal{direction: Vec2::new(direction.x , direction.y ), target_id}         
    }

pub fn chase_smart_rewards(&self, highrewards: &Vec<&Player>, lowrewards: &Vec<&Player>) -> TargetGoal {

    if highrewards.len() + lowrewards.len() == 0 { 
        return TargetGoal{direction: vec2(0.0, 0.0), target_id: 0} 
     }
             
     let mut most_valuable = 0.0;
     let mut direction = Vec2::new(0.0, 0.0);
     let mut target_id = 0;
     
     for reward in highrewards {

         if HIGHVALUE / reward.position.distance(self.position) > most_valuable {    
             direction = reward.position.sub(self.position).normalize();
             most_valuable = HIGHVALUE / reward.position.distance(self.position);
             target_id = reward.id; 
         }
     }

     for reward in lowrewards {
        if LOWVALUE / reward.position.distance(self.position) > most_valuable {    
            direction = reward.position.sub(self.position).normalize();
            most_valuable =  LOWVALUE / reward.position.distance(self.position);
            target_id = reward.id; 
        }   
     }

     return TargetGoal{direction: Vec2::new(direction.x , direction.y ), target_id}               
    }

}
