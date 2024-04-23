use crate::road_items::RoadItem;

use super::StaticRoadItem;
pub struct YieldSign {
    x_location: f64,
    y_location: f64,
}

impl YieldSign {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x_location: x,
            y_location: y,
        }
    }
}

impl RoadItem for YieldSign {
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
        "YieldSign"
    }
}

impl StaticRoadItem for YieldSign {}
