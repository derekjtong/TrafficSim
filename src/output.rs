use crate::road_items::dynamic_items::Vehicle;

pub trait ISimOutput {
    fn get_speed(&self, vehicle: &mut Box<dyn Vehicle>) -> f64;
}

pub trait ISimInput {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64);
}
