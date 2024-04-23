use crate::{
    road_items::{
        dynamic_items::{
            car::Car,
            traffic_light::{LightColor, TrafficLight},
            truck::Truck,
            DynamicRoadItem,
        },
        RoadItem,
    },
    wc_length_to_cc_length, wc_point_to_cc_point, Constants, Heading, Road,
};

pub trait Drawable {
    fn draw_road(&mut self, road: &Road);
    fn draw_car(&mut self, car: &Car);
    fn draw_truck(&mut self, truck: &Truck);
    fn draw_traffic_light(&mut self, traffic_light: &TrafficLight);
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
    fn draw_truck(&mut self, truck: &Truck) {
        println!("Drawing truck on CharMatrix {}", truck.type_name());
    }
    fn draw_traffic_light(&mut self, traffic_light: &TrafficLight) {
        let x = wc_point_to_cc_point(traffic_light.get_x_location());
        let y = wc_point_to_cc_point(-traffic_light.get_y_location());
        if x < Constants::CHAR_MAP_SIZE && y < Constants::CHAR_MAP_SIZE {
            self.map[y][x] = match traffic_light.current_color() {
                LightColor::Red => 'R',
                LightColor::Yellow => 'Y',
                LightColor::Green => 'G',
            };
        }
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
    fn print_truck(&self, v: &Truck, o: &mut dyn Drawable);
    fn print_dynamic_item(&self, item: &dyn DynamicRoadItem, o: &mut dyn Drawable);
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
    fn print_truck(&self, car: &Truck, o: &mut dyn Drawable) {
        o.draw_truck(car);
    }
    fn print_dynamic_item(&self, item: &dyn DynamicRoadItem, o: &mut dyn Drawable) {
        if let Some(car) = item.as_any().downcast_ref::<Car>() {
            o.draw_car(car);
        } else if let Some(truck) = item.as_any().downcast_ref::<Truck>() {
            o.draw_truck(truck);
        } else if let Some(traffic_light) = item.as_any().downcast_ref::<TrafficLight>() {
            o.draw_traffic_light(traffic_light);
        } else {
            eprintln!("Unrecognized dynamic item type");
        }
    }
}
