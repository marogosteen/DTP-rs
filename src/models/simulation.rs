use crate::models::mover_group;
use crate::models::mover_unit;
use crate::models::record;

pub struct SimulationModel{
    pub mover_group_model: mover_group::MoverGroupModel,
    pub time_interval: u64,
    pub car_lane: f64,
    pub car_max_velocity: f64,
    pub train_capacity: usize,
    pub train_velocity: f64,
}

impl SimulationModel{
    pub fn run(mut self, days: usize){
        let mut best_day: usize = 1;
        let mut best_record = record::Record::new();

        for day in 1..=days{ 
            println!("\nday: {}",day);
            let mut record = record::Record::new();

            let (mut car_mover_group, mut train_mover_group) 
                = self.mover_group_model.devide_model();            
            car_mover_group = self.cars_run(car_mover_group);
            train_mover_group = self.trains_run(train_mover_group);
            self.mover_group_model.gather_mover(car_mover_group, train_mover_group);

            let lisning_target_count = 3;
            record = self.mover_group_model.select_route_and_report(lisning_target_count, record);
            record.write_log();

            if day == 1{
                best_record = record;
            }else if record.car_runtime + record.train_runtime < best_record.car_runtime + best_record.train_runtime {
                best_day = day;
                best_record = record;
            }

            self.mover_group_model.initialize_mover();
        }

        println!("\nbest record \nday:{}",best_day);
        best_record.write_log();
    }

    fn cars_run(
        &self, 
        mut car_mover_group: Vec<mover_unit::MoverModel>,
    ) -> Vec<mover_unit::MoverModel>{
        if car_mover_group.len() == 0{
            return car_mover_group
        }

        car_mover_group[0].velocity = self.car_max_velocity;
        let mut time: u64 = 0;
        let route_length = car_mover_group[0].route.get_route_length();
        let mut arrival_id: usize = 0;

        while car_mover_group.last().unwrap().arrival_time == std::u64::MAX {
            time += self.time_interval;

            for car_id in arrival_id..car_mover_group.len(){
                if car_mover_group[car_id].start_time <= time{
                    //locationは前の時刻のvelocityを用い，velocityは現在のlocationを用いる
                    let location: f64 
                        = car_mover_group[car_id].location + car_mover_group[car_id].velocity / 3.6 * self.time_interval as f64;
                    car_mover_group[car_id].location = location;

                    let velocity: f64 = 
                        if car_id == arrival_id{
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
                        arrival_id = car_id + 1;
                    }
                }
            }
        }
        return car_mover_group;
    }

    fn trains_run(
        &self, 
        mut train_mover_group:Vec<mover_unit::MoverModel>,
    ) -> Vec<mover_unit::MoverModel>{
        if train_mover_group.len() == 0{
            return train_mover_group
        }

        let route_length = train_mover_group[0].route.get_route_length();
        let ride_rate = mover_group::MoverGroupModel::RIDE_RATE;

        let mut passengers: usize = 0;
        let mut first_passenger_id: usize = 0;
        let mut current_mover_id = 0;

        loop {
            current_mover_id += 1;
            passengers += (ride_rate * current_mover_id as f64 - ride_rate * (current_mover_id - 1) as f64).round() as usize;

            if passengers >= self.train_capacity{
                current_mover_id = std::cmp::min(current_mover_id, train_mover_group.len() - 1);
                //一車両に乗った客の到着時間は同じになる
                let arrival_time = 
                    train_mover_group[current_mover_id].start_time as f64 + route_length / self.train_velocity * 3.6;

                for id in first_passenger_id ..= current_mover_id{
                    train_mover_group[id].arrival_time = arrival_time as u64;
                    train_mover_group[id].location = route_length;
                    train_mover_group[id].velocity = self.train_velocity;
                }
                passengers = 0;
                first_passenger_id = current_mover_id + 1;

                if current_mover_id >= train_mover_group.len() - 1{
                    break
                }
            }
        }

        return train_mover_group;
    }
}