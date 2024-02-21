pub struct Constants;

impl Constants {
    pub const ACC_RATE: f64 = 3.5; // Acceleration rate for cars in m/s
    pub const ACC_RATE_EMPTY: f64 = 2.5; // Acceleration rate for light trucks in m/s
    pub const ACC_RATE_FULL: f64 = 1.0; // Acceleration rate for heavy trucks in m/s
    pub const DEC_RATE: f64 = 7.0; // Braking rate for cars in m/s
    pub const DEC_RATE_EMPTY: f64 = 5.0; // Braking rate for light trucks in m/s
    pub const DEC_RATE_FULL: f64 = 2.0; // Braking rate for heavy trucks in m/s
    pub const MPS_TO_MPH: f64 = 2.237;
    pub const KPH_TO_MPH: f64 = 0.621371;
}
