use std::io::{self, Write};
use trafficsim::Car;
use trafficsim::Constants;
use trafficsim::Truck;
use trafficsim::Vehicle;
use trafficsim::{ImperialGUI, MetricGUI, GUI};

fn main() {
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
    input.clear();

    // Get user input for speed limit
    print!("Enter speed limit: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Need to flush because print! doesn't automatically flush
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Run example program
    // &mut to make main function maintain ownership of gui
    example(&mut gui);

    gui.add_road_through_gui();
    gui.display_map();

    gui.remove_road_through_gui(0);
    gui.display_map();
}

fn example(gui: &mut Box<dyn GUI>) {
    // Update Speed
    // TODO: move to testing
    let mut car = Car::new(
        0.0,
        0.0,
        "Toyota Camry".to_string(),
        0.0 / Constants::MPS_TO_MPH,
        0.0,
        0.0,
    );
    car.set_desired_speed(65.0 / Constants::MPS_TO_MPH);

    let mut truck1 = Truck::new(
        0.0,
        0.0,
        "Ford F-150".to_string(),
        0.0 / Constants::MPS_TO_MPH,
        0.0,
        0.0,
        4.0,
    );
    truck1.set_desired_speed(55.0 / Constants::MPS_TO_MPH);

    let mut truck2 = Truck::new(
        0.0,
        0.0,
        "Volvo VNL 760".to_string(),
        0.0 / Constants::MPS_TO_MPH,
        0.0,
        0.0,
        8.0,
    );
    truck2.set_desired_speed(50.0 / Constants::MPS_TO_MPH);

    let mut vehicles: Vec<Box<dyn Vehicle>> = Vec::new();
    vehicles.push(Box::new(car));
    vehicles.push(Box::new(truck1));
    vehicles.push(Box::new(truck2));

    for _ in 0..11 {
        for vehicle in vehicles.iter_mut() {
            vehicle.update_speed(1);
            println!(
                "{} speed: {}",
                vehicle.type_name(),
                gui.display_speed(vehicle)
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
