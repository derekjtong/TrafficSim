use crate::{road::Road, Drawable, IPrintDriver};

pub struct Map {
    pub roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Self {
        Self { roads: Vec::new() }
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }

    pub fn remove_road(&mut self, index: usize) {
        if index < self.roads.len() {
            self.roads.remove(index);
        } else {
            eprintln!("Attempted to remove a road at an invalid index: {}", index);
        }
    }

    pub fn get_roads(&self) -> &[Road] {
        &self.roads
    }

    pub fn total_road_items(&self) -> usize {
        self.roads
            .iter()
            .map(|road| road.get_road_items().len())
            .sum()
    }

    pub fn print(&self, pd: &dyn IPrintDriver, o: &mut dyn Drawable) {
        for road in self.roads.iter() {
            pd.print_road(road, o);
            // for item in road.get_road_items().iter() {
            //     pd.print_car(item, o);
            // }
        }
    }
}
