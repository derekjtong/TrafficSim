use crate::road_items::{DynamicRoadItem, Point, RoadItem};

pub struct TrafficLight {
    is_green: bool,
    pos: Point,
}

impl TrafficLight {
    pub fn new(x: f64, y: f64, is_green: bool) -> Self {
        Self {
            is_green,
            pos: Point { x, y },
        }
    }

    pub fn set_green(&mut self) {
        self.is_green = true;
    }

    pub fn set_red(&mut self) {
        self.is_green = false;
    }

    pub fn is_green(&self) -> bool {
        self.is_green
    }
}

impl RoadItem for TrafficLight {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "TrafficLight"
    }
}

impl DynamicRoadItem for TrafficLight {
    fn update(&mut self, _seconds: i32) {}
}
