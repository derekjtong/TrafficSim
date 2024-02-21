use crate::road_items::{Point, RoadItem};
pub struct StopSign {
    pos: Point,
}

impl StopSign {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Point { x, y },
        }
    }
}

impl RoadItem for StopSign {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "StopSign"
    }
}
