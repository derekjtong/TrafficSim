use std::cell::RefCell;
use std::rc::Rc;

use crate::output::{ISimInput, ISimOutput};
use crate::road::Road;
use crate::road_items::dynamic_items::car::Car;
use crate::road_items::dynamic_items::traffic_light::{LightColor, TrafficLight};
use crate::road_items::dynamic_items::Vehicle;
use crate::utils::Constants;
use crate::Heading;

pub trait GUI: ISimOutput + ISimInput {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road;
    fn remove_road(&mut self /* add parameters */);
    fn create_traffic_light(
        &self,
        x_location: f64,
        y_location: f64,
        red_duration: i32,
        yellow_duration: i32,
        green_duration: i32,
        initial_color: LightColor,
    ) -> Rc<RefCell<TrafficLight>>;
    fn create_car(
        &self,
        x_location: f64,
        y_location: f64,
        model: String,
        speed: f64,
        direction: Heading,
        desired_speed: f64,
    ) -> Rc<RefCell<Car>>;
}

pub struct MetricGUI {}

impl MetricGUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISimOutput for MetricGUI {
    fn get_speed(&self, v: &mut Box<dyn Vehicle>) -> f64 {
        // Convert m/s to kph
        v.get_current_speed() * Constants::MPS_TO_KPH
    }
}

impl ISimInput for MetricGUI {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64) {
        // Convert kph to m/s
        v.set_desired_speed(speed * Constants::KPH_TO_MPS)
    }
}

impl GUI for MetricGUI {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road {
        Road::new(
            name,
            length * Constants::KM_TO_M,
            x_location * Constants::KM_TO_M,
            y_location * Constants::KM_TO_M,
            heading,
        )
    }

    fn remove_road(&mut self /* add parameters */) {
        println!("Placeholder: GUI remove road called");
    }

    fn create_traffic_light(
        &self,
        x_location: f64,
        y_location: f64,
        red_duration: i32,
        yellow_duration: i32,
        green_duration: i32,
        initial_color: LightColor,
    ) -> Rc<RefCell<TrafficLight>> {
        Rc::new(RefCell::new(TrafficLight::new(
            x_location * Constants::KM_TO_M,
            y_location * Constants::KM_TO_M,
            red_duration,
            yellow_duration,
            green_duration,
            initial_color,
        )))
    }

    fn create_car(
        &self,
        x_location: f64,
        y_location: f64,
        model: String,
        speed: f64,
        direction: Heading,
        desired_speed: f64,
    ) -> Rc<RefCell<Car>> {
        Rc::new(RefCell::new(Car::new(
            x_location * Constants::KM_TO_M,
            y_location * Constants::KM_TO_M,
            model,
            speed,
            direction,
            desired_speed,
        )))
    }
}

pub struct ImperialGUI {}

impl ImperialGUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISimOutput for ImperialGUI {
    fn get_speed(&self, v: &mut Box<dyn Vehicle>) -> f64 {
        // Convert m/s to kph
        v.get_current_speed() * Constants::MPS_TO_MPH
    }
}

impl ISimInput for ImperialGUI {
    fn set_speed_limit(&mut self, v: &mut Box<dyn Vehicle>, speed: f64) {
        // Convert mph to m/s
        v.set_desired_speed(speed * Constants::MPH_TO_MPS);
    }
}

impl GUI for ImperialGUI {
    fn create_road(
        &mut self,
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road {
        Road::new(
            name,
            length * Constants::MI_TO_M,
            x_location * Constants::MI_TO_M,
            y_location * Constants::MI_TO_M,
            heading,
        )
    }

    fn remove_road(&mut self /* add parameters */) {
        println!("Placeholder: GUI remove road called");
    }

    fn create_traffic_light(
        &self,
        x_location: f64,
        y_location: f64,
        red_duration: i32,
        yellow_duration: i32,
        green_duration: i32,
        initial_color: LightColor,
    ) -> Rc<RefCell<TrafficLight>> {
        Rc::new(RefCell::new(TrafficLight::new(
            x_location * Constants::MI_TO_M,
            y_location * Constants::MI_TO_M,
            red_duration,
            yellow_duration,
            green_duration,
            initial_color,
        )))
    }

    fn create_car(
        &self,
        x_location: f64,
        y_location: f64,
        model: String,
        speed: f64,
        direction: Heading,
        desired_speed: f64,
    ) -> Rc<RefCell<Car>> {
        Rc::new(RefCell::new(Car::new(
            x_location * Constants::MI_TO_M,
            y_location * Constants::MI_TO_M,
            model,
            speed,
            direction,
            desired_speed,
        )))
    }
}
