pub mod dynamic_items;
pub mod static_items;

pub trait RoadItem {
    fn set_pos(&mut self, x_location: f64, y_location: f64);
    fn get_x_location(&self) -> f64;
    fn get_y_location(&self) -> f64;
    fn type_name(&self) -> &'static str;
    // &'static to keep the string in the binary
}
