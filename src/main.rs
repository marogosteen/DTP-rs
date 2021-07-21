mod models;

use crate::models::mover_group::MoverGroup;
use crate::models::simulation::SimulationModel;
use crate::models::record::Record;

fn main(){
    const PEOPLE: usize = 2000;
    const TIME_INTERVAL: u64 = 1;
    const DAYS: usize = 100;

    const CAR_LANE: i32 = 1;
    const CAR_MAX_VELOCITY: f64 = 51.1;
    
    const TRAIN_CAPACITY: usize = 452; //452
    const TRAIN_VELOCITY: f64 = 31.1 ;

    println!(
        "
        parameters
        \n\tPEOPLE: {}
        \n\tCAR_LANE: {}
        \n\tCAR_MAX_VELOCITY: {}
        \n\tTRAIN_CAPACITY: {}
        \n\tTRAIN_VELOCITY: {}
        ",
        PEOPLE,
        CAR_LANE,
        CAR_MAX_VELOCITY,
        TRAIN_CAPACITY,
        TRAIN_VELOCITY,
    );

    let mover_group = MoverGroup::new(PEOPLE, 0.85);
    let record = Record::new();
    let simuration_model = SimulationModel{
        mover_group: mover_group,
        time_interval: TIME_INTERVAL,
        car_lane: CAR_LANE as f64,
        car_max_velocity: CAR_MAX_VELOCITY,
        train_capacity: TRAIN_CAPACITY,
        train_velocity: TRAIN_VELOCITY,
        record: record,
    };

    simuration_model.run(DAYS);
}