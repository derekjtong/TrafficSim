use crate::{
    road_items::{DynamicRoadItem, Point, RoadItem},
    utils::Constants,
};

use super::Vehicle;

pub struct Truck {
    base: DynamicRoadItem,
    model: String,
    speed: f64,
    direction: f64,
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
            base: DynamicRoadItem::new(x, y),
            model,
            speed,
            direction,
            desired_speed,
            load_weight,
        }
    }
}

impl RoadItem for Truck {
    fn set_pos(&mut self, pos: Point) {
        self.base.set_pos(pos);
    }

    fn pos(&self) -> Point {
        self.base.pos()
    }

    fn type_name(&self) -> &'static str {
        "Truck"
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
