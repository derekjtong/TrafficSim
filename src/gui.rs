use crate::{map::Map, road::Road};

// TODO: decide on gui library
pub struct Gui {
    map: Map,
}

impl Gui {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }

    pub fn add_road_through_gui(&mut self /*, road_parameters: ... */) {
        self.map.add_road(Road::new());
        // TODO: imeplement road parameters in road.rs
        println!("Placeholder: A road has been added.");
    }

    pub fn remove_road_through_gui(&mut self, index: usize) {
        self.map.remove_road(index);
        println!(
            "Placeholder: A road at index {} has been removed, if it existed.",
            index
        );
    }

    pub fn display_map(&self) {
        // TODO: placeholder for actual GUI display logic.
        println!(
            "Placeholder: Displaying map with {} roads.",
            self.map.get_roads().len()
        );
        println!(
            "             Total road items on the map: {}",
            self.map.total_road_items()
        );
    }
}
