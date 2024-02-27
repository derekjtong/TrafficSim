use crate::output::{ISimInput, ISimOutput};
use crate::road_items::dynamic_items::Vehicle;
use crate::utils::Constants;
use crate::Heading;
use crate::{map::Map, road::Road};

// TODO: decide on gui library
pub trait GUI: ISimOutput + ISimInput {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road;
    fn remove_road_through_gui(&mut self /* add parameters */);
    // Moved to ISimOutput/ISimInput
    // fn get_speed(&self, v: &mut Box<dyn Vehicle>) -> String;
    // fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64);
}

pub struct MetricGUI {}

impl MetricGUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISimOutput for MetricGUI {
    fn get_speed(&self, v: &mut Box<dyn Vehicle>) -> f64 {
        // Convert m/s to kph
        v.get_current_speed() * Constants::MPS_TO_KPH
    }
}

impl ISimInput for MetricGUI {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64) {
        // Convert kph to m/s
        v.set_desired_speed(speed * Constants::KPH_TO_MPS)
    }
}

impl GUI for MetricGUI {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road {
        Road::new(name, length, x_location, y_location, heading)
    }

    fn remove_road_through_gui(&mut self /* add parameters */) {
        println!("Placeholder: GUI remove road called");
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

impl ISimOutput for ImperialGUI {
    fn get_speed(&self, v: &mut Box<dyn Vehicle>) -> f64 {
        // Convert m/s to kph
        v.get_current_speed() * Constants::MPS_TO_MPH
    }
}

impl ISimInput for ImperialGUI {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64) {
        // Convert mph to m/s
        v.set_desired_speed(speed * Constants::MPH_TO_MPS);
    }
}

impl GUI for ImperialGUI {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road {
        Road::new(name, length, x_location, y_location, heading)
    }

    fn remove_road_through_gui(&mut self /* add parameters */) {
        println!("Placeholder: GUI remove road called");
    }
}
