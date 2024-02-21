use crate::road_items::{Point, RoadItem};

use super::StaticRoadItem;

pub struct SpeedLimitSign {
    pos: Point,
    speed_limit: i8,
}
impl SpeedLimitSign {
    pub fn new(x: f64, y: f64, speed_limit: i8) -> Self {
        Self {
            pos: Point { x, y },
            speed_limit,
        }
    }

    pub fn set_speed_limit(&mut self, speed_limit: i8) {
        self.speed_limit = speed_limit;
    }

    pub fn speed_limit(&self) -> i8 {
        self.speed_limit
    }
}

impl RoadItem for SpeedLimitSign {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "SpeedLimitSign"
    }
}

impl StaticRoadItem for SpeedLimitSign {}
