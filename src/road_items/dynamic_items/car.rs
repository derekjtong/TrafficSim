use std::any::Any;

use crate::{road_items::RoadItem, Constants};

use super::{DynamicRoadItem, Vehicle};

pub struct Car {
    x_location: f64,
    y_location: f64,
    model: String,
    speed: f64,
    _direction: f64,
    desired_speed: f64,
}

impl Car {
    pub fn new(
        x: f64,
        y: f64,
        model: String,
        speed: f64,
        direction: f64,
        desired_speed: f64,
    ) -> Self {
        Self {
            x_location: x,
            y_location: y,
            model,
            speed,
            _direction: direction,
            desired_speed,
        }
    }
}

impl RoadItem for Car {
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
        "Car"
    }
}

impl DynamicRoadItem for Car {
    fn update(&mut self, seconds: i32) {
        self.update_speed(seconds)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Vehicle for Car {
    fn new(x: f64, y: f64, model: String, speed: f64, direction: f64, desired_speed: f64) -> Self
    where
        Self: Sized,
    {
        Self::new(x, y, model, speed, direction, desired_speed)
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
        let speed_increase = Constants::ACC_RATE * seconds as f64;
        self.speed += speed_increase;
        if self.speed > self.desired_speed {
            self.speed = self.desired_speed;
        }
    }

    fn decelerate(&mut self, seconds: i32) {
        let speed_decrease = Constants::DEC_RATE * seconds as f64;
        self.speed -= speed_decrease;
        if self.speed < self.desired_speed {
            self.speed = self.desired_speed;
        }
    }

    fn set_speed_limit(&mut self, ms: f64) {
        self.desired_speed = ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_creation() {
        let car = Car::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 60.0);
        assert_eq!(car.model(), "Test");
        assert_eq!(car.get_current_speed(), 0.0);
        assert_eq!(car.get_x_location(), 0.0);
        assert_eq!(car.get_y_location(), 0.0);
    }

    #[test]
    fn car_accelerate() {
        let mut car = Car::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 50.0);
        // Simulate acceleration for 1 second
        car.accelerate(1);
        // Expected speed increase = ACC_RATE * time (in seconds)
        let expected_speed = Constants::ACC_RATE * 1.0;
        assert_eq!(car.get_current_speed(), expected_speed);
    }

    #[test]
    fn car_decelerate() {
        let mut car = Car::new(0.0, 0.0, "Test".to_string(), 100.0, 0.0, 50.0);
        // Simulate deceleration for 1 second
        car.decelerate(1);
        // Expected speed decrease = DEC_RATE * time (in seconds)
        let expected_speed = 100.0 - Constants::DEC_RATE * 1.0;
        assert_eq!(car.get_current_speed(), expected_speed);
    }

    #[test]
    fn update_speed_increase_towards_desired() {
        let mut car = Car::new(
            0.0,
            0.0,
            "Test".to_string(),
            0.0,
            0.0,
            Constants::ACC_RATE * 2.0,
        );
        // Simulate updating speed for 2 seconds to reach desired speed
        car.update_speed(2);
        assert_eq!(car.get_current_speed(), Constants::ACC_RATE * 2.0);
    }

    #[test]
    fn set_speed_limit() {
        let mut car = Car::new(0.0, 0.0, "Test".to_string(), 0.0, 0.0, 60.0);
        car.set_speed_limit(70.0);
        assert_eq!(car.desired_speed, 70.0);
    }
}
