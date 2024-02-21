use crate::road_items_dynamic::Vehicle;

pub trait ISimOutput {
    fn get_speed(&self, vehicle: &mut Box<dyn Vehicle>) -> String;
}

pub trait ISimInput {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64);
}
