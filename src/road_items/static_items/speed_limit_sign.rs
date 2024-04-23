use crate::road_items::RoadItem;

use super::StaticRoadItem;

pub struct SpeedLimitSign {
    x_location: f64,
    y_location: f64,
    speed_limit: i8,
}
impl SpeedLimitSign {
    pub fn new(x: f64, y: f64, speed_limit: i8) -> Self {
        Self {
            x_location: x,
            y_location: y,
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
        "SpeedLimitSign"
    }
}

impl StaticRoadItem for SpeedLimitSign {}
