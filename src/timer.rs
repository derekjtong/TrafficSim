use std::thread;
use std::time::Duration;

use crate::{Drawable, IPrintDriver, Map, Simulation};

pub struct Timer {
    simulation: Simulation,
    map: Map,
    print_driver: Box<dyn IPrintDriver>,
    drawable: Box<dyn Drawable>,
}

impl Timer {
    pub fn new(
        simulation: Simulation,
        map: Map,
        print_driver: Box<dyn IPrintDriver>,
        drawable: Box<dyn Drawable>,
    ) -> Self {
        Self {
            simulation,
            map,
            print_driver,
            drawable,
        }
    }

    pub fn start(&mut self) {
        let spinner = vec![".", "..", "...", ""];
        let mut spinner_index = 0;
        loop {
            // Clear the screen
            print!("{}[2J", 27 as char);

            // Update simulation
            self.simulation.update(1);

            // Print the map and dynamic road items
            self.drawable.clear();
            self.map.print(&*self.print_driver, &mut *self.drawable);
            self.drawable.print();

            // Update progress spinner
            println!("{}", spinner[spinner_index]);
            spinner_index = (spinner_index + 1) % spinner.len();

            // Sleep for one second
            thread::sleep(Duration::from_secs(1));
        }
    }
}
