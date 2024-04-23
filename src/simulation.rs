use crate::road_items::dynamic_items::DynamicRoadItem;
use std::{cell::RefCell, rc::Rc, vec::Vec};

pub struct Simulation {
    dynamic_items: Vec<Rc<RefCell<dyn DynamicRoadItem>>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            dynamic_items: Vec::new(),
        }
    }

    pub fn add_dynamic_item(&mut self, item: Rc<RefCell<dyn DynamicRoadItem>>) {
        self.dynamic_items.push(item);
    }

    pub fn update(&mut self, seconds: i32) {
        for item in &self.dynamic_items {
            item.borrow_mut().update(seconds);
        }
    }
}
