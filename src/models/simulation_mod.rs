use crate::models::mover_group_mod;

pub struct SimulationModel{
    pub mover_group_model: mover_group_mod::MoverGroupModel
}

impl SimulationModel{
    const TIME_INTERVAL: i64 = 1;
    
    pub fn run(mut self, days: usize){
        let target_count = 3;

        for day in 0..days{ 
            let (mut car_mover_group, mut train_mover_group) 
                = self.mover_group_model.devide_route();
            car_mover_group = self.cars_run(car_mover_group);
            train_mover_group = self.trains_run(train_mover_group);
            self.mover_group_model.gather_mover(car_mover_group, train_mover_group);
            self.mover_group_model.select_route(target_count);
            
            println!("\nday: {}",day);
            for id in 0..10{
                println!("{:?}",self.mover_group_model.model_item[id])
            }
            
        }
    }

    fn cars_run(
        &self, 
        mut car_mover_group: Vec<mover_group_mod::MoverModel>,
    ) -> Vec<mover_group_mod::MoverModel>{
        let mut time: i64 = 0;
        let route_length = car_mover_group[0].route.get_route_length();
        
        while car_mover_group.last().unwrap().arrivaltime == std::i64::MAX {
            time += SimulationModel::TIME_INTERVAL;
            let mut traffic_density:f64 = 0.0;
            let mut lead_runner = 0;

            for car_id in lead_runner..car_mover_group.len(){
                traffic_density = 
                    if car_id == 0{
                        0.0
                    }else{
                        1000.0 / car_mover_group[car_id - 1].location - car_mover_group[car_id].location
                    };

                if car_mover_group[car_id].start_time <= time{
                    let location: f64 
                        = car_mover_group[car_id].location + car_mover_group[car_id].velocity / 3.6 * SimulationModel::TIME_INTERVAL as f64;
                    let velocity: f64 
                        = (51.1 - 0.58647 * traffic_density).max(0.0).min(51.1);
                        //= [0.0, 51.1 - 0.58647 * traffic_density].iter().fold(0.0/0.0, |x,y| y.max(x));

                    car_mover_group[car_id].location = location;
                    car_mover_group[car_id].velocity = velocity;

                    if car_mover_group[car_id].location >= route_length{
                        lead_runner = car_id;
                        car_mover_group[car_id].arrivaltime
                            = std::cmp::min(time,car_mover_group[car_id].arrivaltime);
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
        let capacity: f64 = 150.0;
        let velocity: f64 = 31.1;
        let route_length = train_mover_group[0].route.get_route_length();
        
        let total_movers: usize = train_mover_group.len();
        let mut passengers: f64 = 0.0;
        let mut first_passenger_id: usize = 0;

        for mover_id in 0..total_movers{
            println!("index:{} moverID:{}",mover_id,train_mover_group[mover_id].id);
            passengers += train_mover_group[mover_id].ride_num;
            if passengers >= capacity{
                //一車両に乗った客の到着時間は同じになる
                let arrival_time = train_mover_group[mover_id].start_time as f64 + route_length / velocity * 3.6;
                for id in first_passenger_id..=mover_id{
                    train_mover_group[id].arrivaltime = arrival_time as i64
                }
                passengers = 0.0;
                first_passenger_id = mover_id + 1;
            }
        }
        if passengers > 0.0{
            println!("hoge");
            let last_mover_id: usize = total_movers - 1;
            //一車両に乗った客の到着時間は同じになる
            let arrival_time = train_mover_group[last_mover_id].start_time as f64 + route_length / velocity * 3.6;
            for id in first_passenger_id..last_mover_id{
                train_mover_group[id].arrivaltime = arrival_time as i64
            }
        }
        return train_mover_group;
    }
}