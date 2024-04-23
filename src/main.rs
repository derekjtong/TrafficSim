use trafficsim::{
    CharMatrix, ConsolePrint, Drawable, IPrintDriver, MetricGUI, Road, Simulation, Timer, GUI,
};

fn main() {
    let mut sim_input: Box<dyn GUI> = Box::new(MetricGUI::new());
    let mut map: trafficsim::Map = trafficsim::Map::new();
    let cp: Box<dyn IPrintDriver> = Box::new(ConsolePrint::new());
    let mut cm: Box<dyn Drawable> = Box::new(CharMatrix::new());

    let uptown: Road = sim_input.create_road(
        "Uptown".to_string(),
        0.180,
        -0.01,
        -0.09,
        trafficsim::Heading::North,
    );
    map.add_road(uptown);

    let mut simulation = Simulation::new();

    // Create the timer
    let mut timer = Timer::new(simulation, map, cp, cm);

    // Start the timer
    timer.start();
}
