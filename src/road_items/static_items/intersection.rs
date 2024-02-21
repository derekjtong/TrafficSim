use crate::road_items::{Point, RoadItem};

pub struct Intersection {
    pos: Point,
}

impl Intersection {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Point { x, y },
        }
    }
}

impl RoadItem for Intersection {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }
    fn pos(&self) -> Point {
        self.pos
    }
    fn type_name(&self) -> &'static str {
        "Intersection"
    }
}
