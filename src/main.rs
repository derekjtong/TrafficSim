mod road;
mod road_item;
mod road_items_dynamic;
mod utils;
mod car;
mod truck;
mod gui;
mod map;
// if these are removed, change imports in definition files to use trafficsim::road, etc.

use road_items_dynamic::Vehicle;
use car::Car;
use truck::Truck;
use gui::Gui;
use utils::Constants;
use trafficsim::{SimOutput, ImperialOutput, MetricOutput};
use std::io::{self, Write}; // Write is for flush()


fn main() {
    let mut input = String::new();

    // Get user input for metric or imperial
    print!("Enter 'M' for metric or 'I' for Imperial: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Need to flush because print! doesn't automatically flush
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let sim_output: Box<dyn SimOutput> = if input.trim().to_uppercase() == "M" {
        Box::new(MetricOutput)
    } else {
        Box::new(ImperialOutput)
    };
    input.clear();

    // Get user input for speed limit
    print!("Enter speed limit: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Need to flush because print! doesn't automatically flush
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Run example program
    example(sim_output);

    // Run GUI example
    // let mut gui = Gui::new();

    // gui.add_road_through_gui();
    // gui.display_map();

    // gui.remove_road_through_gui(0);
    // gui.display_map();
}

fn example(sim_output: Box<dyn SimOutput>) {
    // Update Speed
    // TODO: move to testing
    let mut car = Car::new(0.0, 0.0, "Toyota Camry".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0);
    car.set_desired_speed(65.0 / Constants::MPS_TO_MPH);

    let mut truck1 = Truck::new(0.0, 0.0, "Ford F-150".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0, 4.0);
    truck1.set_desired_speed(55.0 / Constants::MPS_TO_MPH);

    let mut truck2 = Truck::new(0.0, 0.0, "Volvo VNL 760".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0, 8.0);
    truck2.set_desired_speed(50.0 / Constants::MPS_TO_MPH);

    let mut vehicles: Vec<Box<dyn Vehicle>> = Vec::new();
    vehicles.push(Box::new(car));
    vehicles.push(Box::new(truck1));
    vehicles.push(Box::new(truck2));


    for _ in 0..11 {
        for vehicle in vehicles.iter_mut() {
            vehicle.update_speed(1);
            println!(
                "{} speed: {:.2} {}",
                vehicle.type_name(),
                sim_output.get_speed(vehicle.get_current_speed()),
                if sim_output.is_metric() { "km/h" } else { "mph" }
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
