pub struct SimulationRecord{
    pub count_car_ride:   usize,
    pub count_train_ride: usize,

    pub count_car_mover:   usize,
    pub count_train_mover: usize,

    pub car_runtime:   u64,
    pub train_runtime: u64,
}

impl SimulationRecord{
    pub fn new() -> SimulationRecord{
        let simulation_record = SimulationRecord{
            count_car_ride: 0,
            car_runtime: 0,
            count_car_mover: 0,
            count_train_ride: 0,
            train_runtime: 0,
            count_train_mover: 0,
        };

        return simulation_record
    }

    //pub fn write_log(record: &SimulationRecord){
    pub fn write_log(&self){
        println!("car_ride:{} train_ride:{}", self.count_car_ride, self.count_train_ride);
        println!(
            "car_runtime:{} trian_runtime:{} \nsum_time:{}", 
            self.car_runtime, 
            self.train_runtime, 
            self.car_runtime + self.train_runtime
        );
    }
}
