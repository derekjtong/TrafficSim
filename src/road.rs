use crate::road_items::RoadItem;

#[derive(Debug, Clone, Copy)]

pub enum Heading {
    North,
    South,
    East,
    West,
}

pub struct Road {
    items: Vec<Box<dyn RoadItem>>,
    name: String,
    length: f64,
    x_location: f64,
    y_location: f64,
    heading: Heading,
}

impl Road {
    pub fn new(
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Self {
        Self {
            items: Vec::new(),
            name,
            length,
            x_location,
            y_location,
            heading,
        }
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_x_location(&self) -> f64 {
        self.x_location
    }

    pub fn get_y_location(&self) -> f64 {
        self.y_location
    }

    pub fn get_heading(&self) -> Heading {
        self.heading
    }

    pub fn get_road_name(&self) -> &str {
        &self.name
    }

    pub fn add_road_item(&mut self, road_item: Box<dyn RoadItem>) {
        self.items.push(road_item);
    }

    pub fn print(&self) {
        // TODO: print interface
    }

    pub fn remove_road_item(&mut self, index: usize) {
        self.items.remove(index);
    }

    // Get a reference to the road items
    pub fn get_road_items(&self) -> &[Box<dyn RoadItem>] {
        &self.items
    }
}
