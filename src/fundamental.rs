//  eng-units (https://crates.io/crates/eng-units)
//  Engineering & scientific conversion for Rust.
//  Copyright (C) 2023 Frank Pereny

//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.

//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.

//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.


use crate::conversions::*;


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
        // Metric
        Unit::Kilometer => Fundamental::Length,
        Unit::Meter => Fundamental::Length,
        Unit::Centimeter => Fundamental::Length,
        Unit::Millimeter => Fundamental::Length,
        // Imperial
        Unit::Inch => Fundamental::Length,
        Unit::Foot => Fundamental::Length,
        Unit::Yard => Fundamental::Length,
        Unit::Mile => Fundamental::Length,
        // Other
        Unit::Lightyear => Fundamental::Length,

        // Time
        Unit::Nanosecond => Fundamental::Time,
        Unit::Microsecond => Fundamental::Time,
        Unit::Millisecond => Fundamental::Time,
        Unit::Second => Fundamental::Time,
        Unit::Minute => Fundamental::Time,
        Unit::Hour => Fundamental::Time,

        _ => {
            println!("WARNING: THIS UNIT FUNDAMENTAL NOT SET");
            println!("Set fundmamental in get_fundamental!");
            Fundamental::Length
        }
    }
}

