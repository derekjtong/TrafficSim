use crate::road_items::RoadItem;

pub struct Simulation {
    // TODO: one array for everything or a separate array for each type of road item?
    road_items: Vec<Box<dyn RoadItem>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            road_items: Vec::new(),
        }
    }

    pub fn add_road_item(&mut self, item: Box<dyn RoadItem>) {
        self.road_items.push(item);
    }

    pub fn update(&mut self) {
        for _item in self.road_items.iter_mut() {
            // TODO: update items, requires implementing the RoadItem update trait (road_item.rs)
        }
    }

    // TODO: visualization
    pub fn get_road_items(&self) -> &Vec<Box<dyn RoadItem>> {
        &self.road_items
    }
}
