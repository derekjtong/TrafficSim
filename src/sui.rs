use crate::{
    road_items::{dynamic_items::car::Car, RoadItem},
    wc_length_to_cc_length, wc_point_to_cc_point, Constants, Heading, Road,
};

pub trait Drawable {
    fn draw_road(&mut self, road: &Road);
    fn draw_car(&mut self, car: &Car);
    fn print(&self);
}

pub struct CharMatrix {
    map: Vec<Vec<char>>,
}

impl CharMatrix {
    pub fn new() -> Self {
        let size = Constants::CHAR_MAP_SIZE;
        let output = vec![vec![' '; size]; size];

        Self { map: output }
    }
}

impl Default for CharMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for CharMatrix {
    fn draw_road(&mut self, road: &Road) {
        let mut x: usize;
        let mut y: usize;
        let mut distance: usize = 0;
        let ccx = wc_point_to_cc_point(road.get_x_location());
        let ccy = wc_point_to_cc_point(-road.get_y_location());
        let cc_road_length = wc_length_to_cc_length(road.get_length());

        match road.get_heading() {
            crate::Heading::North => {
                x = ccx;
                if x < Constants::CHAR_MAP_SIZE {
                    while distance < cc_road_length {
                        if distance > ccy {
                            break;
                        }
                        y = ccy - distance;
                        if y < Constants::CHAR_MAP_SIZE {
                            self.map[y][x] = '|';
                            self.map[y][x + 2] = '|';
                            self.map[y][x + 4] = '|';
                        }
                        distance += 1;
                    }
                }
            }
            Heading::South => {}
            Heading::East => {
                y = ccy;
                if y < Constants::CHAR_MAP_SIZE {
                    while distance < cc_road_length {
                        x = ccx + distance;
                        if x < Constants::CHAR_MAP_SIZE {
                            self.map[y][x] = '-';
                            self.map[y + 2][x] = '-';
                            self.map[y + 4][x] = '-';
                        }
                        distance += 1;
                    }
                }
            }
            Heading::West => {}
        }
    }
    fn draw_car(&mut self, car: &Car) {
        println!("Drawing car on CharMatrix {}", car.type_name());
    }
    fn print(&self) {
        for row in &self.map {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }
}

pub trait IPrintDriver {
    fn print_road(&self, r: &Road, o: &mut dyn Drawable);
    fn print_car(&self, v: &Car, o: &mut dyn Drawable);
}

pub struct ConsolePrint {}

impl ConsolePrint {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConsolePrint {
    fn default() -> Self {
        Self::new()
    }
}

impl IPrintDriver for ConsolePrint {
    fn print_road(&self, road: &Road, o: &mut dyn Drawable) {
        o.draw_road(road);
    }
    fn print_car(&self, car: &Car, o: &mut dyn Drawable) {
        o.draw_car(car);
    }
}
