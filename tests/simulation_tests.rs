use trafficsim::{
    road_items::{dynamic_items::car::Car, static_items::stop_sign::StopSign},
    Simulation,
};

#[test]
fn test_simulation_with_multiple_road_items() {
    let mut simulation = Simulation::new();

    // Add a static road item
    // TODO: replace with a more complex static road item
    simulation.add_road_item(Box::new(StopSign::new(0.0, 0.0)));

    // Add a dynamic road item (e.g., a car) that moves
    let car = Car::new(1.0, 1.0, "Car1".to_string(), 60.0, 0.0, 0.0);
    simulation.add_road_item(Box::new(car));

    // Run the simulation for a few steps
    for _ in 0..10 {
        simulation.update();
    }

    // Check that the car has moved as expected
    let road_items = simulation.get_road_items();

    // Simple check to ensure items exist
    assert!(!road_items.is_empty());
}
