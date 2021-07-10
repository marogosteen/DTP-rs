mod models;

fn main(){
    let mover_group_model = models::mover_group_mod::MoverGroupModel::new(300);
    let simuration_model = models::simulation_mod::SimulationModel{
        mover_group_model: mover_group_model,
    };

    let days: usize = 100;
    simuration_model.run(days);
}