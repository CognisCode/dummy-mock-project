use super::player::{Player, PlayerType};
use nannou::prelude::*;
use super::Simulation;

pub struct Chromosome {
    w_dist: f32,  
    w_value: f32, 
    w_opponent: f32, 
}


pub fn init_chromosones() -> Vec<Chromosome>{

    let mut chromosones: Vec<Chromosome> = Vec::new();

    for _ in 0..5 {
        chromosones.push(
            Chromosome {
                w_dist: random_f32(),
                w_value: random_f32(),
                w_opponent: random_f32()
        });
    };
    chromosones
}

pub fn revaluate(simulation: &mut Simulation) {

}



