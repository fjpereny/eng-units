

use crate::units::Unit;


#[derive(Debug, Clone)]
/// # Fundamental engineering units
/// All units of measurement are derived from one or more fundamental units.
pub enum Fundamental {
    /// Length - meter (m)
    Length,    
    /// Mass - kilogram (kg)
    Mass,
    /// Time - seconds (s)
    Time,
    /// Electrical Current - ampere (A)
    Current,
    /// Temperature - degree Kelvin (K)
    Temperature,
    /// Luminous Intensity - candela (cd)
    LuminousIntensity,
    /// Amount of Substance - mole (mol)
    AmountOfSubstance,

}

pub fn get_fundamental(unit: &Unit) -> Fundamental {
    match unit {

        // Length
        // (metric)
        Unit::Kilometer => Fundamental::Length,
        Unit::Meter => Fundamental::Length,
        Unit::Centimeter => Fundamental::Length,
        Unit::Millimeter => Fundamental::Length,
        // (Imperial)
        Unit::Inch => Fundamental::Length,
        Unit::Foot => Fundamental::Length,
        Unit::Yard => Fundamental::Length,
        Unit::Mile => Fundamental::Length,

        // Time
        Unit::Second => Fundamental::Time,
        Unit::Minute => Fundamental::Time,
        Unit::Hour => Fundamental::Time,

        _ => Fundamental::Length,
    }
}

