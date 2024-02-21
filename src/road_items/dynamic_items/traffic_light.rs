use crate::road_items::{DynamicRoadItem, Point, RoadItem};

pub struct TrafficLight {
    base: DynamicRoadItem,
    is_green: bool,
}

impl TrafficLight {
    pub fn new(x: f64, y: f64, is_green: bool) -> Self {
        Self {
            base: DynamicRoadItem::new(x, y),
            is_green,
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
        self.base.set_pos(pos);
    }

    fn pos(&self) -> Point {
        self.base.pos()
    }

    fn type_name(&self) -> &'static str {
        "TrafficLight"
    }
}
