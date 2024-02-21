use crate::road_items_dynamic::Vehicle;
use crate::utils::Constants;
use crate::{map::Map, road::Road};

// TODO: decide on gui library
pub trait GUI {
    fn add_road_through_gui(&mut self);
    fn remove_road_through_gui(&mut self, index: usize);
    fn display_map(&self);
    fn display_speed(&self, v: &mut Box<dyn Vehicle>) -> String; // New method for displaying speed in the appropriate units
}

pub struct MetricGUI {
    map: Map,
}

impl MetricGUI {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GUI for MetricGUI {
    fn add_road_through_gui(&mut self /*, road_parameters: ... */) {
        self.map.add_road(Road::new());
        // TODO: implement road parameters in road.rs
        println!("Placeholder: A road has been added.");
    }

    fn remove_road_through_gui(&mut self, index: usize) {
        self.map.remove_road(index);
        println!(
            "Placeholder: A road at index {} has been removed, if it existed.",
            index
        );
    }

    fn display_map(&self) {
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

    fn display_speed(&self, v: &mut Box<dyn Vehicle>) -> String {
        format!("{:.2} km/h", v.get_current_speed() * Constants::MPS_TO_MPH) // Convert MPH to KPH
    }
}

pub struct ImperialGUI {
    map: Map,
}

impl ImperialGUI {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GUI for ImperialGUI {
    fn add_road_through_gui(&mut self /*, road_parameters: ... */) {
        self.map.add_road(Road::new());
        // TODO: implement road parameters in road.rs
        println!("Placeholder: A road has been added.");
    }

    fn remove_road_through_gui(&mut self, index: usize) {
        self.map.remove_road(index);
        println!(
            "Placeholder: A road at index {} has been removed, if it existed.",
            index
        );
    }

    fn display_map(&self) {
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

    fn display_speed(&self, v: &mut Box<dyn Vehicle>) -> String {
        format!("{:.2} mph", v.get_current_speed()) // Speed is already in MPH
    }
}
