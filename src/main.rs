mod models;

fn main(){
    const MOVER_NUM: i32 = 300;
    const TIME_INTERVAL: i64 = 1;
    const CAR_MAX_VELOCITY: f64 = 51.1;
    const TRAIN_CAPACITY: f64 = 150.0;
    const TRAIN_VELOCITY: f64 = 31.1;
    const DAYS: usize = 50;
    
    let mover_group_model = models::mover_group_mod::MoverGroupModel::new(MOVER_NUM);

    let simuration_model = models::simulation_mod::SimulationModel{
        mover_group_model: mover_group_model,
        time_interval: TIME_INTERVAL,
        car_max_velocity: CAR_MAX_VELOCITY,
        train_capacity: TRAIN_CAPACITY,
        train_velocity: TRAIN_VELOCITY,
    };

    simuration_model.run(DAYS);
}