mod models;

use crate::models::mover_group::MoverGroup;
use crate::models::simulation::SimulationModel;
use crate::models::record::Record;

fn main(){
    const PEOPLE: usize = 2500;
    const TIME_INTERVAL: u64 = 1;
    const DAYS: usize = 100;

    const CAR_LANE: i32 = 1;
    const CAR_MAX_VELOCITY: f64 = 51.1;
    
    const TRAIN_CAPACITY: usize = 100;
    const TRAIN_VELOCITY: f64 = 31.1 ;

    let mover_group = MoverGroup::new(PEOPLE);
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