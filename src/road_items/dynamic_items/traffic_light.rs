use crate::road_items::dynamic_items::DynamicRoadItem;
use crate::road_items::{Point, RoadItem};

#[derive(Debug, Clone, Copy)]
pub enum LightColor {
    Red,
    Yellow,
    Green,
}

pub struct TrafficLight {
    pos: Point,
    red_duration: i32,
    yellow_duration: i32,
    green_duration: i32,
    current_color: LightColor,
    time_since_last_update: i32,
}

impl TrafficLight {
    pub fn new(x: f64, y: f64, red: i32, yellow: i32, green: i32, start_color: LightColor) -> Self {
        Self {
            pos: Point { x, y },
            red_duration: red,
            yellow_duration: yellow,
            green_duration: green,
            current_color: start_color,
            time_since_last_update: 0,
        }
    }
}

impl RoadItem for TrafficLight {
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "TrafficLight"
    }
}

impl DynamicRoadItem for TrafficLight {
    fn update(&mut self, seconds: i32) {
        self.time_since_last_update += seconds;

        match self.current_color {
            LightColor::Red => {
                if self.time_since_last_update >= self.red_duration {
                    self.current_color = LightColor::Green;
                    self.time_since_last_update = 0;
                }
            }
            LightColor::Yellow => {
                if self.time_since_last_update >= self.yellow_duration {
                    self.current_color = LightColor::Red;
                    self.time_since_last_update = 0;
                }
            }
            LightColor::Green => {
                if self.time_since_last_update >= self.green_duration {
                    self.current_color = LightColor::Yellow;
                    self.time_since_last_update = 0;
                }
            }
        }
    }
}
