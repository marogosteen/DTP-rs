mod models;

fn main(){
    const PEOPLE: usize = 2500;
    const TIME_INTERVAL: u64 = 1;

    const CAR_LANE: i32 = 1;
    const CAR_MAX_VELOCITY: f64 = 51.1;
    
    const TRAIN_CAPACITY: usize = 100;
    const TRAIN_VELOCITY: f64 = 31.1;
    const DAYS: usize = 100;

    let mover_group_model = models::mover_group_mod::MoverGroupModel::new(PEOPLE);

    let simuration_model = models::simulation_mod::SimulationModel{
        mover_group_model: mover_group_model,
        time_interval: TIME_INTERVAL,
        car_lane: CAR_LANE as f64,
        car_max_velocity: CAR_MAX_VELOCITY,
        train_capacity: TRAIN_CAPACITY,
        train_velocity: TRAIN_VELOCITY,
    };

    simuration_model.run(DAYS);
}