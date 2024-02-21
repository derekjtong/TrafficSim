use super::RoadItem;

pub mod car;
pub mod traffic_light;
pub mod truck;

pub trait DynamicRoadItem: RoadItem {
    fn update(&mut self, seconds: i32);
}
pub trait Vehicle: DynamicRoadItem {
    fn new(x: f64, y: f64, model: String, speed: f64, direction: f64, desired_speed: f64) -> Self
    where
        Self: Sized;
    fn model(&self) -> &String;
    fn set_desired_speed(&mut self, mps: f64);
    fn get_current_speed(&self) -> f64;
    fn update_speed(&mut self, seconds: i32);
    fn accelerate(&mut self, seconds: i32);
    fn decelerate(&mut self, seconds: i32);
    fn set_speed_limit(&mut self, ms: f64);
}
