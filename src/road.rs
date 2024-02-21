use crate::road_items::RoadItem;

pub struct Road {
    road_items: Vec<Box<dyn RoadItem>>,
    // TODO: add road parameters
}

impl Road {
    // Constructor for an empty Road
    pub fn new() -> Self {
        Self {
            road_items: Vec::new(),
        }
    }

    // Constructor with initial road items
    pub fn with_items(road_items: Vec<Box<dyn RoadItem>>) -> Self {
        Self { road_items }
    }

    pub fn add_road_item(&mut self, road_item: Box<dyn RoadItem>) {
        self.road_items.push(road_item);
    }

    pub fn remove_road_item(&mut self, index: usize) {
        self.road_items.remove(index);
    }

    // Get a reference to the road items
    pub fn get_road_items(&self) -> &[Box<dyn RoadItem>] {
        &self.road_items
    }
}
