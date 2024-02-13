use crate::{
    road_item::{DynamicRoadItem, Point, RoadItem},
    road_items_dynamic::Vehicle,
    utils::Constants,
};

// TODO: consolidate into folder

pub struct Car {
    base: DynamicRoadItem,
    model: String,
    speed: f64,
    direction: f64,
    desired_speed: f64,
}

// Car-specific implementations
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
            base: DynamicRoadItem::new(x, y),
            model,
            speed,
            direction,
            desired_speed,
        }
    }
}

impl RoadItem for Car {
    fn set_pos(&mut self, pos: Point) {
        self.base.set_pos(pos);
    }

    fn pos(&self) -> Point {
        self.base.pos()
    }

    fn type_name(&self) -> &'static str {
        "Car"
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

    fn set_desired_speed(&mut self, mph: f64) {
        self.desired_speed = mph;
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
}
