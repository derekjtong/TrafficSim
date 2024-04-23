use std::any::Any;

use crate::{road_items::RoadItem, utils::Constants};

use super::{DynamicRoadItem, Vehicle};

pub struct Truck {
    x_location: f64,
    y_location: f64,
    model: String,
    speed: f64,
    _direction: f64,
    desired_speed: f64,
    load_weight: f64, // tons
}

impl Truck {
    pub fn new(
        x: f64,
        y: f64,
        model: String,
        speed: f64,
        direction: f64,
        desired_speed: f64,
        load_weight: f64,
    ) -> Self {
        Self {
            x_location: x,
            y_location: y,
            model,
            speed,
            _direction: direction,
            desired_speed,
            load_weight,
        }
    }
}

impl RoadItem for Truck {
    fn set_pos(&mut self, x: f64, y: f64) {
        self.x_location = x;
        self.y_location = y;
    }

    fn get_x_location(&self) -> f64 {
        self.x_location
    }
    fn get_y_location(&self) -> f64 {
        self.y_location
    }

    fn type_name(&self) -> &'static str {
        "Truck"
    }
}

impl DynamicRoadItem for Truck {
    fn update(&mut self, seconds: i32) {
        self.update_speed(seconds)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Vehicle for Truck {
    fn new(x: f64, y: f64, model: String, speed: f64, direction: f64, desired_speed: f64) -> Self
    where
        Self: Sized,
    {
        Self::new(x, y, model, speed, direction, desired_speed, 0.0) // Default load_weight to 0.0
    }

    fn model(&self) -> &String {
        &self.model
    }

    fn set_desired_speed(&mut self, ms: f64) {
        self.desired_speed = ms;
    }

    fn get_current_speed(&self) -> f64 {
        self.speed
    }

    fn update_speed(&mut self, seconds: i32) {
        if self.speed < self.desired_speed {
            self.accelerate(seconds);
        } else if self.speed > self.desired_speed {
            self.decelerate(seconds);
        }
    }

    fn accelerate(&mut self, seconds: i32) {
        let acceleration_rate = if self.load_weight <= 5.0 {
            Constants::ACC_RATE_EMPTY
        } else {
            Constants::ACC_RATE_FULL
        };
        self.speed += acceleration_rate * seconds as f64;
        if self.speed > self.desired_speed {
            self.speed = self.desired_speed;
        }
    }

    fn decelerate(&mut self, seconds: i32) {
        let deceleration_rate = if self.load_weight <= 5.0 {
            Constants::DEC_RATE_EMPTY
        } else {
            Constants::DEC_RATE_FULL
        };
        self.speed -= deceleration_rate * seconds as f64;
        if self.speed < self.desired_speed {
            self.speed = self.desired_speed;
        }
    }

    fn set_speed_limit(&mut self, ms: f64) {
        self.desired_speed = ms;
    }
}

#[cfg(test)]
mod truck_tests {
    use super::*;

    #[test]
    fn truck_creation() {
        let truck = Truck::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 60.0, 10.0);
        assert_eq!(truck.model(), "Test");
        assert_eq!(truck.get_current_speed(), 0.0);
        assert_eq!(truck.get_x_location(), 0.0);
        assert_eq!(truck.get_y_location(), 0.0);
        assert_eq!(truck.load_weight, 10.0);
    }

    #[test]
    fn truck_accelerate_empty() {
        let mut truck = Truck::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 50.0, 4.0);
        truck.accelerate(1); // Empty truck acceleration for 1 second
        let expected_speed = Constants::ACC_RATE_EMPTY * 1.0;
        assert_eq!(truck.get_current_speed(), expected_speed);
    }

    #[test]
    fn truck_accelerate_full() {
        let mut truck = Truck::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 50.0, 6.0);
        truck.accelerate(1); // Full truck acceleration for 1 second
        let expected_speed = Constants::ACC_RATE_FULL * 1.0;
        assert_eq!(truck.get_current_speed(), expected_speed);
    }

    #[test]
    fn truck_decelerate_empty() {
        let mut truck = Truck::new(0.0, 0.0, "Test".to_string(), 100.0, 0.0, 50.0, 4.0);
        truck.decelerate(1); // Empty truck deceleration for 1 second
        let expected_speed = 100.0 - Constants::DEC_RATE_EMPTY * 1.0;
        assert_eq!(truck.get_current_speed(), expected_speed);
    }

    #[test]
    fn truck_decelerate_full() {
        let mut truck = Truck::new(0.0, 0.0, "Test".to_string(), 100.0, 0.0, 50.0, 6.0);
        truck.decelerate(1); // Full truck deceleration for 1 second
        let expected_speed = 100.0 - Constants::DEC_RATE_FULL * 1.0;
        assert_eq!(truck.get_current_speed(), expected_speed);
    }

    #[test]
    fn update_speed_increase_towards_desired_empty() {
        let mut truck = Truck::new(
            0.0,
            0.0,
            "Test".to_string(),
            0.0,
            0.0,
            Constants::ACC_RATE_EMPTY * 2.0,
            4.0,
        );
        truck.update_speed(2); // Assuming the empty truck can reach desired speed in 2 seconds
        assert_eq!(truck.get_current_speed(), Constants::ACC_RATE_EMPTY * 2.0);
    }

    #[test]
    fn update_speed_increase_towards_desired_full() {
        let mut truck = Truck::new(
            0.0,
            0.0,
            "Test".to_string(),
            0.0,
            0.0,
            Constants::ACC_RATE_FULL * 2.0,
            6.0,
        );
        truck.update_speed(2); // Assuming the full truck can reach desired speed in 2 seconds
        assert_eq!(truck.get_current_speed(), Constants::ACC_RATE_FULL * 2.0);
    }

    #[test]
    fn set_speed_limit() {
        let mut truck = Truck::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 60.0, 5.0);
        truck.set_speed_limit(70.0);
        assert_eq!(truck.desired_speed, 70.0);
    }
}
