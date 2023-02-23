

#[derive(Debug, Clone)]
pub enum Unit {
    
    // Length
    // (metric)
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    // (Imperial)
    Inch,
    Foot,
    Yard,
    Mile,

    // Time
    Second,
    Minute,
    Hour,

    Temp,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {

            // Length
            // (metric)
            Unit::Kilometer => "km",
            Unit::Meter => "m",
            Unit::Centimeter => "cm",
            Unit::Millimeter => "mm",
            // (Imperial)
            Unit::Inch => "in",
            Unit::Foot => "ft",
            Unit::Yard => "yard",
            Unit::Mile => "mile",

            // Time
            Unit::Second => "sec",
            Unit::Minute => "min",
            Unit::Hour => "hr",

            _ => "Need to finish all the Unit strings",
        };
        write!(f, "{text}")
    }
}
