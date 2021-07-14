use std;
use rand::Rng;
use crate::models::simulation_mod;

pub struct MoverGroupModel{
    pub model_item: Vec<MoverModel>,
}

impl MoverGroupModel{
    const RIDE_RATE: f64 = 1.43;

    pub fn new(people: usize) -> MoverGroupModel{
        let mut mover_group_model = MoverGroupModel{
            model_item: Vec::new()
        };

        let ride_vec: Vec<usize> = MoverGroupModel::generate_ride_vec(people);
        for (id,ride) in ride_vec.into_iter().enumerate(){
            mover_group_model.model_item.push(MoverModel::new(id,ride));
        }

        return mover_group_model;
    }

    fn generate_ride_vec(people: usize) -> Vec<usize>{
        let mut mover: Vec<usize> = Vec::new();
        let mut count:  usize = 0;
        loop{
            count += 1;
            let totalride = (MoverGroupModel::RIDE_RATE * count as f64).round() as usize;
            let ride = totalride - (MoverGroupModel::RIDE_RATE * (count - 1) as f64).round() as usize;
            
            if totalride as usize == people{
                mover.push(ride);
                return mover
            }else if totalride as usize > people{
                mover.push(1);
                return mover
            }else{
                mover.push(ride);
            }
        }

    }
    
    pub fn devide_model(&self) -> (Vec<MoverModel>,Vec<MoverModel>){
        let mut car_mover_group: Vec<MoverModel> = Vec::new();
        let mut train_mover_group: Vec<MoverModel> = Vec::new();
        for mover in self.model_item.iter(){
            match mover.route{
                Route::Car => car_mover_group.push(mover.clone()),
                Route:: Train => train_mover_group.push(mover.clone())
            }
        }
        return (car_mover_group,train_mover_group);
    }

    pub fn gather_mover(
        &mut self,
        mut car_mover_group: Vec<MoverModel>,
        mut train_mover_group: Vec<MoverModel>,
    ){
        let mut new_mover_group: Vec<MoverModel> = Vec::new();
        new_mover_group.append(&mut car_mover_group);
        new_mover_group.append(&mut train_mover_group);
        new_mover_group.sort_by_key(|mover| mover.id);

        self.model_item = new_mover_group;
    }

    fn generate_target_id_list(
        &self, 
        lisner_id: usize, 
        mut lisning_target_count: usize,
        movers_count: usize,
    ) -> Vec<usize>{
        lisning_target_count = std::cmp::min(lisning_target_count, self.model_item.len());
        let half: usize = lisning_target_count / 2;
            
        let shift = 
            if lisner_id < half {
                0
            // 0originを考慮してmovers_count - 1
            } else if lisner_id > movers_count - 1 - half{
                movers_count - lisning_target_count
            } else{
                lisner_id - half
            }; 

        // 聞き込み件数が偶数でも，自身を含めた奇数にする．
        let target_id_list: Vec<usize> = (shift .. half * 2 + 1 + shift).collect::<Vec<usize>>();

        return target_id_list
    }

    pub fn select_route(
        &mut self, lisning_target_count: usize, mut record: simulation_mod::SimulationRecord
    ) -> simulation_mod::SimulationRecord{
        let movers_count: usize = self.model_item.len() as usize;
        let mut next_mover_group_model = self.model_item.clone();

        for lisner_id in 0..movers_count{
            let mover = &self.model_item[lisner_id];
            let mut first_mover_id: usize = lisner_id;
            let mut first_mover_time: u64 = mover.arrival_time - mover.start_time;

            match mover.route{
                Route::Car => {
                    record.count_car_ride += mover.ride;
                    record.car_runtime += mover.arrival_time - mover.start_time;
                    record.count_car_mover += 1;
                }
                Route::Train => {
                    record.count_train_ride += mover.ride;
                    record.train_runtime += mover.arrival_time - mover.start_time;
                    record.count_train_mover += 1;
                }
            }

            let target_id_list: Vec<usize> = self.generate_target_id_list(
                lisner_id, lisning_target_count, movers_count
            );

            for target_id in target_id_list{
                let run_time = self.model_item[target_id].arrival_time - self.model_item[target_id].start_time;
                if first_mover_time > run_time{
                    first_mover_id = target_id;
                    first_mover_time = run_time;
                }
            }

            let mut rng = rand::thread_rng();
            let probability: f32 = rng.gen(); 
            if probability >= 0.95{
                next_mover_group_model[lisner_id].route = self.model_item[first_mover_id].route.clone();
            }
        }
        
        record.car_runtime = (record.car_runtime as f64 / record.count_car_mover as f64) as u64;
        record.train_runtime = (record.train_runtime as f64 / record.count_train_mover as f64) as u64;

        self.model_item = next_mover_group_model;
        return record
    }

    pub fn initialize_mover(&mut self){
        for id in 0..self.model_item.len(){
            self.model_item[id].arrival_time = std::u64::MAX;
            self.model_item[id].location = 0.0;
            self.model_item[id].velocity = 0.0;
        }
    }
}

#[derive(Debug,Clone)]
pub struct MoverModel{
    pub id:   usize,
    pub ride: usize,

    pub start_time:   u64,
    pub arrival_time: u64,
    
    pub route: Route,
    
    pub location: f64,
    pub velocity: f64,
}

impl MoverModel{
    pub fn new(id: usize, ride: usize) -> MoverModel{
        let mover_model = MoverModel{
            id: id,
            route: match std::cmp::min(id%2,1){
                0 => Route::Car,
                _ => Route::Train,
            },
            ride: ride,
            start_time: id as u64,
            arrival_time: std::u64::MAX,
            location: 0.0,
            velocity: 0.0,
        };

        return mover_model;
    }
}

#[derive(Debug, Clone)]
pub enum Route{
    Car,
    Train,
}

impl Route{
    pub fn get_route_length(&self) -> f64{
        let route_length: f64 = match self {
            Self::Car => 11400.0,
            Self::Train => 11400.0,
        };
        return route_length;
    }
}