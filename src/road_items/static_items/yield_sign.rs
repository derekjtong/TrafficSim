use crate::road_items::{Point, RoadItem};

use super::StaticRoadItem;
pub struct YieldSign {
    pos: Point,
}

impl YieldSign {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Point { x, y },
        }
    }
}

impl RoadItem for YieldSign {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "YieldSign"
    }
}

impl StaticRoadItem for YieldSign {}
