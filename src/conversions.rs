

/// Scalar conversion value of each unit to its base unit
/// Used in converting between units to get correct values
pub mod convs {

    // Length
    pub const KILOMETER_TO_BASE: f64 = 1_000.0;
    pub const METER_TO_BASE: f64 = 1.0;
    pub const CENTIMETER_TO_BASE: f64 = 1.0 / 100.0;
    pub const MILLIMETER_TO_BASE: f64 = 1.0 / 1_000.0;

    // Time
    pub const SECOND_TO_BASE: f64 = 1.0;
    pub const MINUTE_TO_BASE: f64 = 60.0;
    pub const HOUR_TO_BASE: f64 = 3600.0;
}