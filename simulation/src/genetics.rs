use nannou::prelude::*;
use crate::simulation::Simulation;

#[derive(Debug, Clone, Copy)]
pub struct Chromosome {
    pub w_dist: f32,  
    pub w_value: f32, 
    pub w_opponent: f32, 
}


// pub fn init_chromosones() -> Vec<Chromosome>{

//     let mut chromosones: Vec<Chromosome> = Vec::new();

//     for _ in 0..5 {
//         chromosones.push(
//             Chromosome {
//                 w_dist: random_f32(),
//                 w_value: random_f32(),
//                 w_opponent: random_f32()
//         });
//     };
//     chromosones
// }

pub fn revaluate(simulation: &mut Simulation) {
    
    // simple genetic algorthim after 5 iterations take the highsest scoring weights and and small mutations

    simulation.genetic_tree.entry(simulation.progress.genetic_score).or_insert( simulation.progress.chromosome);

    if simulation.iteration > 5  {

        if let Some((_, largest_value)) =  simulation.genetic_tree.iter().rev().next() {

            simulation.progress.chromosome = Chromosome{
                w_dist: largest_value.w_dist + (random_f32() - 0.5) / 10.0 ,
                w_value: largest_value.w_value + (random_f32() - 0.5) / 10.0 ,
                w_opponent: largest_value.w_opponent + (random_f32() - 0.5) / 10.0 
            };

        }
    } else {
        simulation.progress.chromosome = Chromosome{w_dist: random_f32(),w_value: random_f32(),w_opponent: random_f32()};
    }
}


