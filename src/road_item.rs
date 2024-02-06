#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait RoadItem {
    fn set_pos(&mut self, pos: Point);
    fn pos(&self) -> Point;
    fn type_name(&self) -> &'static str;
}

pub struct StaticRoadItem {
    pos: Point,
}

impl StaticRoadItem {
    // Constructor method for StaticRoadItem
    pub fn new(x: f64, y: f64) -> Self {
        Self { pos: Point { x, y } }
    }
}

impl RoadItem for StaticRoadItem {
    // Implement the RoadItem trait for StaticRoadItem
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "StaticRoadItem"
    }
}

pub struct DynamicRoadItem {
    pos: Point,
}

impl DynamicRoadItem {
    // Constructor method for DynamicRoadItem
    pub fn new(x: f64, y: f64) -> Self {
        Self { pos: Point { x, y } }
    }

    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.pos.x += dx;
        self.pos.y += dy;
    }
}

impl RoadItem for DynamicRoadItem {
    // Implement the RoadItem trait for DynamicRoadItem
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn type_name(&self) -> &'static str {
        "DynamicRoadItem"
    }
}
