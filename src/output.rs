pub trait SimOutput {
    fn get_speed(&self, vehicle_speed: f64) -> f64;
    fn is_metric(&self)->bool;
}

pub struct ImperialOutput;

impl SimOutput for ImperialOutput {
    fn get_speed(&self, vehicle_speed: f64) -> f64 {
        vehicle_speed // Assuming speed is already in MPH
    }
    fn is_metric(&self)->bool {
        false
    }
}

pub struct MetricOutput;

impl SimOutput for MetricOutput {
    fn get_speed(&self, vehicle_speed: f64) -> f64 {
        vehicle_speed * 1.6 // Convert MPH to KPH
    }
    fn is_metric(&self)->bool {
        true
    }
}
