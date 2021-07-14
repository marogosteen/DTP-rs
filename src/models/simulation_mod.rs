use crate::models::mover_group_mod;

pub struct SimulationModel{
    pub mover_group_model: mover_group_mod::MoverGroupModel,
    pub time_interval: u64,
    pub car_lane: f64,
    pub car_max_velocity: f64,
    pub train_capacity: usize,
    pub train_velocity: f64,
}

impl SimulationModel{
    pub fn run(mut self, days: usize){
        let mut best_day: usize = 0;
        let mut best_record = SimulationRecord::new();
        //let mut best_ride:Vec<usize> = vec![std::usize::MAX,std::usize::MAX];
        //let mut best_runtime:Vec<u64> = vec![std::u64::MAX,std::u64::MAX];

        for day in 0..days{ 
            println!("\nday: {}",day + 1);
            let mut record = SimulationRecord::new();

            let (mut car_mover_group, mut train_mover_group) 
                = self.mover_group_model.devide_model();            
            if car_mover_group.len() != 0{
                car_mover_group = self.cars_run(car_mover_group);
            };
            if train_mover_group.len() != 0{
                train_mover_group = self.trains_run(train_mover_group);
            };
            self.mover_group_model.gather_mover(car_mover_group, train_mover_group);
            
            let target_count = 3;
            record = self.mover_group_model.select_route(target_count, record);
            self.mover_group_model.initialize_mover();
            
                        println!("car_ride:{} train_ride:{}", record.count_car_ride, record.count_train_ride);
                        println!("car_runtime:{} trian_runtime:{}", record.car_runtime, record.train_runtime);
            
            if day == 0{
                best_record = record;
                //best_ride = vec![record.count_car_ride, record.count_train_ride];
                //best_runtime = vec![record.car_runtime, record.train_runtime];
            }else if record.car_runtime + record.train_runtime < best_record.car_runtime + best_record.train_runtime {
                best_day = day + 1;
                best_record = record;
                //best_ride = vec![record.count_car_ride, record.count_train_ride];
                //best_runtime = vec![record.car_runtime, record.train_runtime];
            }
        }

        println!("\nbest record \nday:{}",best_day);
        println!("car_ride:{} train_ride:{}", best_record.count_car_ride, best_record.count_train_ride);
        println!("car_runtime:{} trian_runtime{}", best_record.car_runtime, best_record.train_runtime);
    }

    fn cars_run(
        &self, 
        mut car_mover_group: Vec<mover_group_mod::MoverModel>,
    ) -> Vec<mover_group_mod::MoverModel>{
        car_mover_group[0].velocity = self.car_max_velocity;
        let mut time: u64 = 0;
        let route_length = car_mover_group[0].route.get_route_length();

        while car_mover_group.last().unwrap().arrival_time == std::u64::MAX {
            time += self.time_interval;

            for car_id in 0..car_mover_group.len(){
                if car_mover_group[car_id].start_time <= time{
                    //locationは前の時刻のvelocityを用い，velocityは現在のlocationを用いる
                    let location: f64 
                        = car_mover_group[car_id].location + car_mover_group[car_id].velocity / 3.6 * self.time_interval as f64;
                    car_mover_group[car_id].location = location;

                    let velocity: f64 = 
                        if car_id == 0{
                            51.1
                        }else{
                            let traffic_dencity: f64 
                                = 1000.0 / (car_mover_group[car_id - 1].location - car_mover_group[car_id].location) / self.car_lane;
                            
                            (self.car_max_velocity - 0.58647 * traffic_dencity).max(0.0).min(self.car_max_velocity)
                        };
                    car_mover_group[car_id].velocity = velocity;

                    if car_mover_group[car_id].location >= route_length{
                        car_mover_group[car_id].arrival_time
                            = std::cmp::min(time,car_mover_group[car_id].arrival_time);
                    }
                }
            }
        }
        return car_mover_group;
    }

    fn trains_run(
        &self, 
        mut train_mover_group:Vec<mover_group_mod::MoverModel>,
    ) -> Vec<mover_group_mod::MoverModel>{
        let route_length = train_mover_group[0].route.get_route_length();
        let mut passengers: usize = 0;
        let mut first_passenger_id: usize = 0;

        for current_mover_id in 0..train_mover_group.len(){
            passengers += train_mover_group[current_mover_id].ride;

            if passengers >= self.train_capacity{
                //一車両に乗った客の到着時間は同じになる
                let arrival_time = 
                    train_mover_group[current_mover_id].start_time as f64 + route_length / self.train_velocity * 3.6;

                for id in first_passenger_id..=current_mover_id{
                    train_mover_group[id].arrival_time = arrival_time as u64;
                    train_mover_group[id].location = route_length;
                    train_mover_group[id].velocity = self.train_velocity;
                }
                passengers = 0;
                first_passenger_id = current_mover_id + 1;
            }
        }
        if passengers > 0{
            //一車両に乗った客の到着時間は同じになる
            let arrival_time = train_mover_group.last().unwrap().start_time as f64 + route_length / self.train_velocity * 3.6;
            for id in first_passenger_id..train_mover_group.len(){
                train_mover_group[id].arrival_time = arrival_time as u64;
                train_mover_group[id].location = route_length;
                train_mover_group[id].velocity = self.train_velocity;
            }
        }
        return train_mover_group;
    }
}

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
}