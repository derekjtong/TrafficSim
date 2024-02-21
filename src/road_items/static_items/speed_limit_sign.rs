use crate::road_items::StaticRoadItem;

pub struct SpeedLimitSign {
    base: StaticRoadItem,
    speed_limit: i8,
}

impl SpeedLimitSign {
    pub fn new(x: f64, y: f64, speed_limit: i8) -> Self {
        Self {
            base: StaticRoadItem::new(x, y),
            speed_limit,
        }
    }

    pub fn get_speed_limit(&self) -> i8 {
        self.speed_limit
    }

    pub fn set_speed_limit(&mut self, speed_limit: i8) {
        self.speed_limit = speed_limit;
    }
}
