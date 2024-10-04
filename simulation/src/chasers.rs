use nannou::prelude::*;
use crate::*;
use crate::rewards::Reward;
use crate::close_strategy::chase_closest;

#[derive(Debug, PartialEq, Clone)]
pub enum ChaserType {
    Closest,
    Highest,
    Expected,
    Genetic,
}

#[derive(Debug, Clone)]
pub struct Chaser {
    pub position: Vec2,
    pub angle_vec: Vec2,
    pub chaser_type: ChaserType,
    pub direction: Vec2,
    size: f32, 
    max_step_size: f32,
    pub target_id: i16,
    pub score: i32,
    pub color: Color,
}

impl Chaser {
    pub fn new(chaser_type: ChaserType, color: Color) -> Chaser {
     
        Chaser {
                position: vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT),
                angle_vec: vec2(0.0, 0.0),
                size: SIZE,
                direction: vec2(0.0, 0.0),
                max_step_size: 30.0,
                chaser_type,
                target_id: 0,
                score: 0,
                color
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .w_h(self.size, self.size)
            .x_y(self.position.x, self.position.y)
            .rotate(self.angle_vec.angle()) // angle of the triangle
            .rgba(self.color.red, self.color.green, self.color.blue, 0.85);
    }

    pub fn update(&mut self){
        self.position += self.direction.normalize(); 

        // angle vector is previous direction + new direction and then normalized to keep te step size equal to max step size
        self.angle_vec += self.direction;
        self.angle_vec = self.angle_vec.clamp_length(self.max_step_size,self.max_step_size);
        self.direction = vec2(0.0, 0.0); // reset accelartion
    }

    pub fn strategy(&mut self, rewards: &Vec<Reward>) {

        match self.chaser_type { 
            ChaserType::Closest => chase_closest(self, rewards),
            _ => ()
            // ChaserType::Expected => chase_closest(),
            // ChaserType::Highest  => chase_closest(),
            // ChaserType::Genetic  => chase_closest(),
        }
    }


}