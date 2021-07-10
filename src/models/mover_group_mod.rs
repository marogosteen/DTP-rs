use std;
use rand::Rng;

pub struct MoverGroupModel{
    pub model_item: Vec<MoverModel>,
}

impl MoverGroupModel{
    pub fn new(generate:i32,) -> MoverGroupModel{
        let mut mover_group_model = MoverGroupModel{
            model_item: Vec::new()
        };

        for id in 0..generate{
            mover_group_model.model_item.push(MoverModel::new(id));
        }
        
        return mover_group_model;
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

    pub fn select_route(&mut self, mut lisning_target_count: usize){
        lisning_target_count = std::cmp::min(lisning_target_count, self.model_item.len());
        let half: usize = lisning_target_count / 2;
        let movers_count: usize = self.model_item.len() as usize;
        let mut next_mover_group_model = self.model_item.clone();

        for lisner_id in 0..movers_count{
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
            let mut first_mover_id: usize = lisner_id;
            let mut first_mover_time: i64 = self.model_item[lisner_id].arrival_time - self.model_item[lisner_id].start_time;

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

        self.model_item = next_mover_group_model;
    }

    pub fn initialize_mover(&mut self){
        for id in 0..self.model_item.len(){
            self.model_item[id].arrival_time = std::i64::MAX;
            self.model_item[id].location = 0.0;
            self.model_item[id].velocity = 0.0;
        }
    }

    pub fn check_mover(&self){
        let mut count_route_car = 0;
        let mut count_route_train = 0;
        for id in 0..self.model_item.len(){
            let mover = &self.model_item[id];
            println!(
                "id:{} \trun time:{} \tstart:{} \tarrival:{} \troute:{:?}",
                mover.id, mover.arrival_time - mover.start_time, mover.start_time, mover.arrival_time, mover.route
            );

            match mover.route{
                Route::Car => count_route_car+=1,
                Route::Train => count_route_train+=1,
            }
        }
        println!("car:{} train:{}",count_route_car,count_route_train);
    }
}

#[derive(Debug,Clone)]
pub struct MoverModel{
    pub id:          i32,
    pub start_time:  i64,
    pub arrival_time: i64,
    
    pub route: Route,
    
    pub ride_num:    f64,
    pub location:    f64,
    pub velocity:    f64,
}

impl MoverModel{
    pub fn new(id: i32) -> MoverModel{
        let mover_model = MoverModel{
            id: id,
            route:
                if std::cmp::min(id%3,1) == 0{
                    Route::Car
                }else{
                    Route::Train
                },
            ride_num: 1.43,
            start_time: id as i64,
            arrival_time: std::i64::MAX,
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
            Self::Car => 10000.0,
            Self::Train => 10000.0,
        };
        return route_length;
    }
}