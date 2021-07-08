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
        
        for i in 0..generate{
            let mover = MoverModel{
                id: i,
                route: 
                    if std::cmp::min(i%3,1) == 0{
                        Route::Car
                    }else{
                        Route::Train
                    },
                ride_num: 1.43,
                start_time: 1 + i as i64,
                arrivaltime: std::i64::MAX,
                location: 0.0,
                velocity: if i == 0{51.1}else{0.0},
            };
            mover_group_model.model_item.push(mover);
        }
        
        return mover_group_model;
    }
    
    pub fn devide_route(
        &self, 
    ) -> (Vec<MoverModel>,Vec<MoverModel>){
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

    pub fn select_route(&mut self, lisning_target_count: usize){
        let half: usize = lisning_target_count / 2;
        let movers_count: usize = self.model_item.len() as usize;

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
            let target_id_list: Vec<usize> = (0 + shift .. half * 2 + 1 + shift).collect::<Vec<usize>>();
            let mut first_mover_id: usize = lisner_id;
            let mut first_mover_time: i64 = self.model_item[lisner_id].arrivaltime - self.model_item[lisner_id].start_time;

            for target_id in target_id_list{
                let run_time = self.model_item[target_id].arrivaltime - self.model_item[target_id].start_time;
                if first_mover_time > run_time{
                    first_mover_id = target_id;
                    first_mover_time = run_time;
                };
            }

            let mut rng = rand::thread_rng();
            let probability: f32 = rng.gen(); 
            if probability >= 0.95{
                self.model_item[lisner_id].route = self.model_item[first_mover_id].route.clone();
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct MoverModel{
    pub id:          i32,
    pub start_time:  i64,
    pub arrivaltime: i64,
    
    pub route: Route,
    
    pub ride_num:    f64,
    pub location:    f64,
    pub velocity:    f64,
}

#[derive(Debug, Clone)]
pub enum Route{
    Car,
    Train,
}

impl Route{
    pub fn get_route_length(&self) -> f64{
        let route_length: f64 = match self {
            Self::Car => 1000.0,
            Self::Train => 1000.0,
        };
        return route_length;
    }
}