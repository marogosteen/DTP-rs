use rand::Rng;
use crate::models::mover_unit;
use crate::models::record;

pub struct MoverGroupModel{
    pub model_item: Vec<mover_unit::MoverModel>,
}

impl MoverGroupModel{
    pub const RIDE_RATE: f64 = 1.43;

    pub fn new(people: usize) -> MoverGroupModel{
        let mut mover_group_model = MoverGroupModel{
            model_item: Vec::new()
        };

        let train_people: usize = people / 10 * 9;
        let car_people: usize = people - train_people;

        let many_route: mover_unit::Route;
        let less_route: mover_unit::Route;
        let many_ride_vec: Vec<usize>;
        let less_ride_vec: Vec<usize>;
        let mut many_index = 0;
        let mut less_index = 0;

        if car_people > train_people{
            many_route = mover_unit::Route::Car;
            less_route = mover_unit::Route::Train;
            many_ride_vec = MoverGroupModel::generate_ride_vec(car_people);
            less_ride_vec = MoverGroupModel::generate_ride_vec(train_people);
        }else{
            many_route = mover_unit::Route::Train;
            less_route = mover_unit::Route::Car;
            many_ride_vec = MoverGroupModel::generate_ride_vec(train_people);
            less_ride_vec = MoverGroupModel::generate_ride_vec(car_people);
        }

        let sum_mover = many_ride_vec.len() + less_ride_vec.len();
        let rate = less_ride_vec.len() as f64 / many_ride_vec.len() as f64;
        let mut counter: f64 = rate / 2.0;

        for id in 0..sum_mover{
            let start_interval: u64 = (3600 * id / (sum_mover - 1)) as u64;
            let route: mover_unit::Route;
            let ride: usize;
            if counter < 1.0{
                route = many_route.clone();
                ride = many_ride_vec[many_index];
                many_index += 1;
                counter += rate;
            }
            else {
                route = less_route.clone();
                ride = less_ride_vec[less_index];
                less_index += 1;
                counter -= 1.0;
            }
            mover_group_model.model_item.push(mover_unit::MoverModel::new(id, route, ride, start_interval));
        }
        return mover_group_model;
    }

    fn generate_ride_vec(people: usize) -> Vec<usize>{
        let mut ride_vec: Vec<usize> = Vec::new();
        let mut count:  usize = 0;
        loop{
            count += 1;
            let totalride = (MoverGroupModel::RIDE_RATE * count as f64).round() as usize;
            let ride = totalride - (MoverGroupModel::RIDE_RATE * (count - 1) as f64).round() as usize;
            
            if totalride as usize == people{
                ride_vec.push(ride);
                return ride_vec
            }else if totalride as usize > people{
                ride_vec.push(1);
                return ride_vec
            }else{
                ride_vec.push(ride);
            }
        }

    }
    
    pub fn devide_model(&self) -> (Vec<mover_unit::MoverModel>,Vec<mover_unit::MoverModel>){
        let mut car_mover_group: Vec<mover_unit::MoverModel> = Vec::new();
        let mut train_mover_group: Vec<mover_unit::MoverModel> = Vec::new();
        for mover_unit in self.model_item.iter(){
            match mover_unit.route{
                mover_unit::Route::Car => car_mover_group.push(mover_unit.clone()),
                mover_unit::Route:: Train => train_mover_group.push(mover_unit.clone())
            }
        }
        return (car_mover_group,train_mover_group);
    }

    pub fn gather_mover(
        &mut self,
        mut car_mover_group: Vec<mover_unit::MoverModel>,
        mut train_mover_group: Vec<mover_unit::MoverModel>,
    ){
        let mut new_mover_group: Vec<mover_unit::MoverModel> = Vec::new();
        new_mover_group.append(&mut car_mover_group);
        new_mover_group.append(&mut train_mover_group);
        new_mover_group.sort_by_key(|mover_unit| mover_unit.id);

        self.model_item = new_mover_group;
    }

    fn generate_target_id_list(
        &self, 
        lisner_id: usize, 
        mut lisning_target: usize,
        movers_count: usize,
    ) -> Vec<usize>{
        lisning_target = std::cmp::min(lisning_target, self.model_item.len());
        let half: usize = lisning_target / 2;
            
        let shift = 
            if lisner_id < half {
                0
            // 0originを考慮してmovers_count - 1
            } else if lisner_id > movers_count - 1 - half{
                movers_count - lisning_target
            } else{
                lisner_id - half
            }; 

        // 聞き込み件数が偶数でも，自身を含めた奇数にする．
        let target_id_list: Vec<usize> = (shift .. half * 2 + 1 + shift).collect::<Vec<usize>>();

        return target_id_list
    }

    pub fn select_route_and_report(
        &mut self, lisning_target: usize, mut record: record::Record
    ) -> record::Record{
        let mut next_mover_group_model = self.model_item.clone();
        
        let movers_count: usize = self.model_item.len() as usize;

        for lisner_id in 0..movers_count{
            let mover_unit = &self.model_item[lisner_id];
            let mut first_mover_id: usize = lisner_id;
            let mut first_mover_time: u64 = mover_unit.arrival_time - mover_unit.start_time;

            match mover_unit.route{
                mover_unit::Route::Car => {
                    record.count_car_ride += mover_unit.ride;
                    record.car_runtime += mover_unit.arrival_time - mover_unit.start_time;
                    record.count_car_mover += 1;
                }
                mover_unit::Route::Train => {
                    record.count_train_ride += mover_unit.ride;
                    record.train_runtime += mover_unit.arrival_time - mover_unit.start_time;
                    record.count_train_mover += 1;
                }
            }

            let target_id_list: Vec<usize> = self.generate_target_id_list(
                lisner_id, lisning_target, movers_count
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