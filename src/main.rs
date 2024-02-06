mod road;
mod road_item;
mod road_items_dynamic;
mod utils;
mod car;
mod truck;

use road::Road;
use road_item::{DynamicRoadItem, StaticRoadItem};
use road_items_dynamic::Vehicle;
use car::Car;
use truck::Truck;
use utils::Constants;


fn main() {
    // Old testing code
    // let mut road = Road::new();

    // // Create road items on heap and add to road
    // let dynamic_item = Box::new(DynamicRoadItem::new(1.0, 2.0));
    // road.add_road_item(dynamic_item);

    // let static_item = Box::new(StaticRoadItem::new(3.0, 4.0));
    // road.add_road_item(static_item);

    // let mycar = Box::new(Car::new(0.2, 0.0, "Tesla Model S".to_string(), 60.0, 0.0));
    // road.add_road_item(mycar);

    // let mytruck = Box::new(Truck::new(0.0, 0.0, "Ford F-150".to_string(), 45.0, 0.0, 1000.0));
    // road.add_road_item(mytruck);

    // // Show road items
    // for item in road.get_road_items() {
    //     println!("Road item type: {:?}, position: {:?}", item.type_name(), item.pos());
    // }

    let mut car = Car::new(0.0, 0.0, "Tesla Model S".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0);
    car.set_desired_speed(65.0 / Constants::MPS_TO_MPH); // Convert mph to m/s

    let mut truck1 = Truck::new(0.0, 0.0, "Ford F-150".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0, 4.0);
    truck1.set_desired_speed(55.0 / Constants::MPS_TO_MPH); // Convert mph to m/s

    let mut truck2 = Truck::new(0.0, 0.0, "Ford F-150".to_string(), 0.0 / Constants::MPS_TO_MPH, 0.0, 0.0, 8.0);
    truck2.set_desired_speed(50.0 / Constants::MPS_TO_MPH); // Convert mph to m/s

    let mut vehicles: Vec<Box<dyn Vehicle>> = Vec::new();
    vehicles.push(Box::new(car));
    vehicles.push(Box::new(truck1));
    vehicles.push(Box::new(truck2));

    for _ in 0..11 {
        for vehicle in vehicles.iter_mut() {
            vehicle.update_speed(1);
            println!("{} speed: {:.2} mph", vehicle.type_name(), vehicle.get_current_speed() * Constants::MPS_TO_MPH);
        }
    }

}
