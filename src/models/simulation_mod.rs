use crate::models::mover_group_mod;

pub struct SimulationModel{
    pub mover_group_model: mover_group_mod::MoverGroupModel,
}

impl SimulationModel{
    const TIME_INTERVAL: i64 = 1;
    const CAR_MAX_VELOCITY: f64 = 51.1;
    const TRAIN_CAPACITY: f64 = 150.0;
    const TRAIN_VELOCITY: f64 = 31.1;

    pub fn run(mut self, days: usize){
        for day in 0..days{ 
            println!("\nday: {}",day);

            let (mut car_mover_group, mut train_mover_group) 
                = self.mover_group_model.devide_model();            
            if car_mover_group.len() != 0{
                car_mover_group = self.cars_run(car_mover_group);
            };
            if train_mover_group.len() != 0{
                train_mover_group = self.trains_run(train_mover_group);
            };
            self.mover_group_model.gather_mover(car_mover_group, train_mover_group);

            self.mover_group_model.check_mover();
            let target_count = 3;
            self.mover_group_model.select_route(target_count);
            self.mover_group_model.initialize_mover();
        }
    }

    fn cars_run(
        &self, 
        mut car_mover_group: Vec<mover_group_mod::MoverModel>,
    ) -> Vec<mover_group_mod::MoverModel>{
        car_mover_group[0].velocity = SimulationModel::CAR_MAX_VELOCITY;
        let mut time: i64 = 0;
        let route_length = car_mover_group[0].route.get_route_length();

        while car_mover_group.last().unwrap().arrival_time == std::i64::MAX {
            time += SimulationModel::TIME_INTERVAL;

            for car_id in 0..car_mover_group.len(){
                if car_mover_group[car_id].start_time <= time{
                    //locationは前の時刻のvelocityを用い，velocityは現在のlocationを用いる
                    let location: f64 
                        = car_mover_group[car_id].location + car_mover_group[car_id].velocity / 3.6 * SimulationModel::TIME_INTERVAL as f64;
                    car_mover_group[car_id].location = location;

                    let velocity: f64 = 
                        if car_id == 0{
                            51.1
                        }else{
                            let traffic_dencity: f64 = 1000.0 / (car_mover_group[car_id - 1].location - car_mover_group[car_id].location);
                            (SimulationModel::CAR_MAX_VELOCITY - 0.58647 * traffic_dencity).max(0.0).min(SimulationModel::CAR_MAX_VELOCITY)
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
        let mut passengers: f64 = 0.0;
        let mut first_passenger_id: usize = 0;

        for current_mover_id in 0..train_mover_group.len(){
            passengers += train_mover_group[current_mover_id].ride_num;

            if passengers >= SimulationModel::TRAIN_CAPACITY{
                //一車両に乗った客の到着時間は同じになる
                let arrival_time = 
                    train_mover_group[current_mover_id].start_time as f64 + route_length / SimulationModel::TRAIN_VELOCITY * 3.6;

                for id in first_passenger_id..=current_mover_id{
                    train_mover_group[id].arrival_time = arrival_time as i64;
                    train_mover_group[id].location = route_length;
                    train_mover_group[id].velocity = SimulationModel::TRAIN_VELOCITY;
                }
                passengers = 0.0;
                first_passenger_id = current_mover_id + 1;
            }
        }
        if passengers > 0.0{
            //一車両に乗った客の到着時間は同じになる
            let arrival_time = train_mover_group.last().unwrap().start_time as f64 + route_length / SimulationModel::TRAIN_VELOCITY * 3.6;
            for id in first_passenger_id..train_mover_group.len(){
                train_mover_group[id].arrival_time = arrival_time as i64;
                train_mover_group[id].location = route_length;
                train_mover_group[id].velocity = SimulationModel::TRAIN_VELOCITY;
            }
        }
        return train_mover_group;
    }
}