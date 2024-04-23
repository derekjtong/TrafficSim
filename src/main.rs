use std::{cell::RefCell, rc::Rc};

use trafficsim::{
    road_items::dynamic_items::traffic_light::{LightColor, TrafficLight},
    CharMatrix, ConsolePrint, Drawable, IPrintDriver, MetricGUI, Road, Simulation, Timer, GUI,
};

fn main() {
    let cp: Box<dyn IPrintDriver> = Box::new(ConsolePrint::new());
    let cm: Box<dyn Drawable> = Box::new(CharMatrix::new());
    let mut sim_input: Box<dyn GUI> = Box::new(MetricGUI::new());
    let mut map: trafficsim::Map = trafficsim::Map::new();
    let mut simulation = Simulation::new();

    // Create road
    let uptown: Road = sim_input.create_road(
        "Uptown".to_string(),
        0.180,
        -0.03,
        -0.09,
        trafficsim::Heading::North,
    );
    // Add road to map
    map.add_road(uptown);

    // Create traffic light smart pointer (Rc) with mutable interior access (RefCell)
    let traffic_light = Rc::new(RefCell::new(TrafficLight::new(
        -0.01,
        -0.09,
        3,
        1,
        3,
        LightColor::Red,
    )));

    // Cloning smart pointer, not the RefCell traffic light instance
    simulation.add_dynamic_item(traffic_light.clone());
    map.add_dynamic_item(traffic_light.clone());

    // Create the timer
    let mut timer = Timer::new(simulation, map, cp, cm);
    timer.start();
}
