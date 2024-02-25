pub struct Constants;

impl Constants {
    pub const ACC_RATE: f64 = 3.5; // Acceleration rate for cars in m/s
    pub const ACC_RATE_EMPTY: f64 = 2.5; // Acceleration rate for light trucks in m/s
    pub const ACC_RATE_FULL: f64 = 1.0; // Acceleration rate for heavy trucks in m/s
    pub const DEC_RATE: f64 = 7.0; // Braking rate for cars in m/s
    pub const DEC_RATE_EMPTY: f64 = 5.0; // Braking rate for light trucks in m/s
    pub const DEC_RATE_FULL: f64 = 2.0; // Braking rate for heavy trucks in m/s

    pub const MPS_TO_MPH: f64 = 2.237;
    pub const MPH_TO_MPS: f64 = 0.44704;

    pub const MPS_TO_KPH: f64 = 3.6;
    pub const KPH_TO_MPS: f64 = 0.277778;

    pub const CHAR_MAP_SIZE: f64 = 100.0;
    pub const WORLD_SIZE: f64 = 50.0;
}

pub fn wc_point_to_cc_point(val: f64) -> i32 {
    ((val * (Constants::CHAR_MAP_SIZE / Constants::WORLD_SIZE)) + (Constants::CHAR_MAP_SIZE / 2.0))
        .round() as i32
}

pub fn wc_length_to_cc_length(val: f64) -> i32 {
    (val * (Constants::CHAR_MAP_SIZE / Constants::WORLD_SIZE)).round() as i32
}
