pub struct Record{
    pub count_car_ride:   usize,
    pub count_train_ride: usize,

    pub count_car_mover:   usize,
    pub count_train_mover: usize,

    pub car_runtime:         u64,
    pub train_runtime:       u64,
    pub mover_group_runtime: u64,
}

impl Record{
    pub fn new() -> Record{
        let simulation_record = Record{
            count_car_ride:   0,
            count_train_ride: 0,

            count_car_mover:   0,
            count_train_mover: 0,

            car_runtime:   0,
            train_runtime: 0,
            mover_group_runtime: u64::MAX,
        };

        return simulation_record
    }

    pub fn write_log(&self){
        println!("car ride: {} \ttrain ride: {}", self.count_car_ride, self.count_train_ride);
        println!(
            "car runtime: {} \ttrian runtime: {} \tsum time: {}", 
            self.car_runtime, 
            self.train_runtime, 
            self.car_runtime + self.train_runtime
        );
        println!("mover group runtime: {}",self.mover_group_runtime);
    }
}
