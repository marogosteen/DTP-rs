#[derive(Debug,Clone)]
pub struct MoverUnit{
    pub id:   usize,
    pub ride: usize,

    pub start_time:   u64,
    pub arrival_time: u64,
    
    pub route: Route,
    
    pub location: f64,
    pub velocity: f64,
}

impl MoverUnit{
    pub fn new(
        id: usize, route: Route, ride: usize, start_interval: u64) -> MoverUnit{
        let mover_model = MoverUnit{
            id: id,
            route: route,
            ride: ride,
            start_time: start_interval,
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