use std::io::{self, Write};
use trafficsim::{
    road_items::dynamic_items::{car::Car, truck::Truck, Vehicle},
    CharMatrix, ConsolePrint, Drawable, IPrintDriver, ImperialGUI, MetricGUI, Road, GUI,
};

fn main() {
    let mut sim_input: Box<dyn GUI> = Box::new(MetricGUI::new());
    let mut map: trafficsim::Map = trafficsim::Map::new();
    let cp: Box<dyn IPrintDriver> = Box::new(ConsolePrint::new());

    let uptown: Road = sim_input.create_road(
        "Uptown".to_string(),
        0.180,
        -0.00,
        -0.07,
        trafficsim::Heading::North,
    );
    map.add_road(uptown);
    let crosstown: Road = sim_input.create_road(
        "Crosstown".to_string(),
        0.180,
        -0.09,
        0.0,
        trafficsim::Heading::East,
    );
    map.add_road(crosstown);

    let mut cm: Box<dyn Drawable> = Box::new(CharMatrix::new());
    map.print(&*cp, &mut *cm);

    cm.print();

    // old();
}

fn _old() {
    let mut input = String::new();

    // Get user input for metric or imperial
    print!("Enter 'M' for metric or 'I' for Imperial: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Need to flush because print! doesn't automatically flush
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut gui: Box<dyn GUI> = if input.trim().to_uppercase() == "M" {
        Box::new(MetricGUI::new())
    } else {
        Box::new(ImperialGUI::new())
    };

    // Get user input for speed limit
    print!("Enter speed limit: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut speed_limit_str = String::new();
    io::stdin()
        .read_line(&mut speed_limit_str)
        .expect("Failed to read line");
    let speed_limit: f64 = speed_limit_str
        .trim()
        .parse()
        .expect("Please enter a valid number for the speed limit");

    // Run example program
    // &mut to make main function maintain ownership of gui
    _example(&mut gui, speed_limit);

    // gui.add_road_through_gui();
    // gui.display_map();

    // gui.remove_road_through_gui(0);
    // gui.display_map();
}

fn _example(gui: &mut Box<dyn GUI>, speed_limit: f64) {
    let mut car: Box<dyn Vehicle> = Box::new(Car::new(
        0.0,
        0.0,
        "Toyota Camry".to_string(),
        0.0,
        0.0,
        0.0,
    ));
    let mut truck1: Box<dyn Vehicle> = Box::new(Truck::new(
        0.0,
        0.0,
        "Ford F-150".to_string(),
        0.0,
        0.0,
        0.0,
        4.0,
    ));
    let mut truck2: Box<dyn Vehicle> = Box::new(Truck::new(
        0.0,
        0.0,
        "Volvo VNL 760".to_string(),
        0.0,
        0.0,
        0.0,
        8.0,
    ));

    gui.set_speed_limit(&mut car, speed_limit);
    gui.set_speed_limit(&mut truck1, speed_limit);
    gui.set_speed_limit(&mut truck2, speed_limit);

    // TODO: Change to dyanmic road items
    let mut vehicles: Vec<Box<dyn Vehicle>> = Vec::new();
    vehicles.push(car);
    vehicles.push(truck1);
    vehicles.push(truck2);

    for _ in 0..11 {
        for vehicle in vehicles.iter_mut() {
            vehicle.update(1);
            println!(
                "{} speed: {:.2}",
                vehicle.type_name(),
                gui.get_speed(vehicle)
            );
        }
    }
}

// Example usage of road, road items, and gui

// let mut road = Road::new();

// // Create road items on heap and add to road
// let dynamic_item = Box::new(DynamicRoadItem::new(1.0, 2.0));
// road.add_road_item(dynamic_item);

// let static_item = Box::new(StaticRoadItem::new(3.0, 4.0));
// road.add_road_item(static_item);

// let mycar = Box::new(Car::new(0.2, 0.0, "Tesla Model S".to_string(), 60.0, 0.0));
// road.add_road_item(mycar); -

// let mytruck = Box::new(Truck::new(0.0, 0.0, "Ford F-150".to_string(), 45.0, 0.0, 1000.0));
// road.add_road_item(mytruck);

// // Show road items
// for item in road.get_road_items() {
//     println!("Road item type: {:?}, position: {:?}", item.type_name(), item.pos());
// }
