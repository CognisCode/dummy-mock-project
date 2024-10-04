
use nannou_osc as osc;
use nannou::prelude::*;

use crate::genetics::*;
use crate::*;

pub struct Simulation {
    pub chasers: Vec<Chaser>,
    pub rewards: Vec<Reward> ,
    pub rewards_left: usize,
    pub iteration: usize,
    pub max_iterations: usize,
    pub sender: osc::Sender<osc::Connected>,
    // pub genetic_tree: BTreeMap<i32, Chromosome>,
}


pub fn app(app: &App) -> Simulation {

    // Initialize the window
    app.new_window()
        .size(2400, 1200)
        .view(view)
        .build()
        .unwrap();

    let iteration = 0;
    let max_iterations = MAXITERATIONS;
    let rewards_left = HIGHREWARDS + LOWREWARDS;

    let target_addr = format!("{}:{}", "127.0.0.1", PORT);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");


    let mut rewards: Vec<Reward> = Vec::with_capacity(HIGHREWARDS+LOWREWARDS);
    create_reward(1, HIGHREWARDS, RewardType::HighReward, HIGHVALUE, &mut rewards ,Color{red: 0.0, green: 255.0, blue: 0.0});
    create_reward(HIGHREWARDS as i16 + 1, LOWREWARDS, RewardType::LowReward, LOWVALUE, &mut rewards, Color{red: 0.0, green: 0.0, blue: 255.0});


    let mut chasers: Vec<Chaser> = Vec::with_capacity(CLOSECHASER + SMARTCHASER + HIGHCHASER);
    create_chaser(CLOSECHASER, ChaserType::Closest, &mut chasers, Color{red: 139.0, green: 0.0, blue: 139.0});
    create_chaser(SMARTCHASER, ChaserType::Expected, &mut chasers,  Color{red: 0.0, green: 0.0, blue: 0.0});
    create_chaser(HIGHCHASER, ChaserType::Highest, &mut chasers,  Color{red: 255.0, green: 0.0, blue: 0.0});
    // create_chasers(GENETICCHASER, PlayerType::Genetic, &mut all_players_vector, progress.genetic_start, Color{red: 0.0, green: 0.0, blue: 139.0});

    Simulation {rewards_left, iteration, max_iterations, sender, chasers, rewards}
}

pub fn next_step(app: &App, simulation: &mut Simulation, _update: Update) {
    

    for chaser in &mut simulation.chasers {
        chaser.strategy(&simulation.rewards);
    }

    for reward in  &mut simulation.rewards {
        if reward.assign_score(&simulation.chasers) {
            simulation.rewards_left -= 1;
        }
    }    

    for chaser in  &mut simulation.chasers {
        chaser.update();
    }

    if simulation.rewards_left == 0 {
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

fn view(app: &App, simulation: &Simulation, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BEIGE);

    let smart_score: i32 = 0;
    let smart_x: f32 = 0.0;
    let smart_y: f32 = 0.0;

    let high_score: i32 = 0;
    let close_score: i32 = 0;
    let genetic_score: i32 = 0;


    for chaser in simulation.chasers {
        chaser.show(&draw); 

        match chaser.chaser_type{
            ChaserType::Closest =>{},
            _ => () 
        }
    }

    for reward in simulation.rewards {
        reward.show(&draw); 
    }

    let osc_addr = "/simulation/scores".to_string();

    let args: Vec<nannou_osc::Type> = vec![
        osc::Type::Int(smart_score),
        osc::Type::Int(high_score),
        osc::Type::Int(close_score),
        osc::Type::Int(genetic_score),
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
                simulation.all_players_vector[i].color = Color{red: 0.0, green: 255.0, blue: 0.0};
                high_reward_count += 1;
            } else {
                simulation.all_players_vector[i].player_type = PlayerType::LowReward;
                simulation.all_players_vector[i].position = vec2((random_f32() - 0.5) * 2400.0, (random_f32() - 0.5) * 1200.0);
                simulation.all_players_vector[i].value = LOWVALUE;
                simulation.all_players_vector[i].color = Color{red: 0.0, green: 0.0, blue: 255.0}
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