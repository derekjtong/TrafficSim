use crate::road_item::StaticRoadItem;

// TODO: add RoadItem traits

pub struct StopSign {
    base: StaticRoadItem,
}

pub struct Intersection {
    base: StaticRoadItem,
}

pub struct SpeedLimitSign {
    base: StaticRoadItem,
    speed_limit: i32,
}

impl SpeedLimitSign {
    pub fn new(speed_limit: i32) -> Self {
        Self {
            base: StaticRoadItem { pos: Point { x: 0.0, y: 0.0 } },
            speed_limit,
        }
    }

    pub fn get_speed_limit(&self) -> i32 {
        self.speed_limit
    }

    pub fn set_speed_limit(&mut self, speed_limit: i32) {
        self.speed_limit = speed_limit;
    }
}

pub struct YieldSign {
    base: StaticRoadItem,
}
