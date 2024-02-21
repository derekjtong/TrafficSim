pub mod dynamic_items;
pub mod static_items;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait RoadItem {
    fn set_pos(&mut self, pos: Point);
    fn pos(&self) -> Point;
    fn type_name(&self) -> &'static str;
    // &'static to keep the string in the binary
}
