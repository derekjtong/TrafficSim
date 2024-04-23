use crate::road_items::dynamic_items::DynamicRoadItem;
use std::vec::Vec;

pub struct Simulation {
    dynamic_items: Vec<Box<dyn DynamicRoadItem>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            dynamic_items: Vec::new(),
        }
    }

    // Add dynamic road items to the simulation
    pub fn add_dynamic_item(&mut self, item: Box<dyn DynamicRoadItem>) {
        self.dynamic_items.push(item);
    }

    // Update method that will call update method for each dynamic road item
    pub fn update(&mut self, seconds: i32) {
        for item in self.dynamic_items.iter_mut() {
            item.update(seconds);
        }
    }
}
