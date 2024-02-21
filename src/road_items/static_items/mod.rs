use super::RoadItem;

pub mod intersection;
pub mod speed_limit_sign;
pub mod stop_sign;
pub mod yield_sign;

pub trait StaticRoadItem: RoadItem {}
