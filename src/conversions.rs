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



use crate::fundamental::*;


/// Scalar conversion value of each unit to its base unit
/// Used in converting between units to get correct values
pub mod convs {

    // Length
    // Metric
    pub const KILOMETER_TO_BASE: f64 = 1_000.0;
    pub const METER_TO_BASE: f64 = 1.0;
    pub const CENTIMETER_TO_BASE: f64 = 1.0 / 100.0;
    pub const MILLIMETER_TO_BASE: f64 = 1.0 / 1_000.0;
    // Imperial
    pub const FOOT_TO_BASE: f64 = 0.3048;
    pub const INCH_TO_BASE: f64 = FOOT_TO_BASE / 12.0;
    pub const YARD_TO_BASE: f64 = FOOT_TO_BASE * 3.0;
    pub const MILE_TO_BASE: f64 = FOOT_TO_BASE * 5_280.0;
    // Common
    pub const LIGHTYEAR_TO_BASE: f64 = 9_460_730_472_580_800.0;


    // Mass
    // Metric
    pub const KILOGRAM_TO_BASE: f64 = 1.0;


    // Time
    pub const NANOSECOND_TO_BASE: f64 = 1.0 / 1_000_000_000.0;
    pub const MICROSECOND_TO_BASE: f64 = 1.0 / 1_000_000.0;
    pub const MILLISECOND_TO_BASE: f64 = 1.0 / 1_000.0;
    pub const SECOND_TO_BASE: f64 = 1.0;
    pub const MINUTE_TO_BASE: f64 = 60.0;
    pub const HOUR_TO_BASE: f64 = 3600.0;

    // Temperature Change
    pub const KELVIN_TO_BASE: f64 = 1.0;
    pub const CELCIUS_TO_BASE: f64 = 1.0;
    pub const FAHRENHEIT_TO_BASE: f64 = 5.0 / 9.0;
    pub const RANKINE_TO_BASE: f64 = 5.0 / 9.0;
    
}


pub fn temp_to_kelvin(value: f64, temp: &Unit) -> f64 {
    match temp {
        Unit::Kelvin => value,
        Unit::Rankine => 5.0 / 9.0 * value,
        Unit::Celcius => -273.15 + value,
        Unit::Fahrenheit => {
            (value + 32.0) / 1.80 - 273.15
        },
        _ => {
            println!("Invalid temperature unit!");
            0.0
        }
    }
}

pub fn kelvin_to_temp(value: f64, temp: &Unit) -> f64 {
    match temp {
        Unit::Kelvin => value,
        Unit::Rankine => value * 9.0 / 5.0,
        Unit::Celcius => value + 273.15,
        Unit::Fahrenheit => {
            (value + 273.15) * 9.0 / 5.0 - 32.0
        },
        _ => {
            println!("Invalid temperature unit!");
            0.0
        }
    }
}


/// Converts a unit's value to the base unit value.
/// Returns the base unit value and base unit type.
/// Example: Unit::KM => 1,000.0, Unit::M
pub fn unit_to_base_val(unit: &Unit) -> f64 {
    match unit {
        
        // Length
        // Metric
        Unit::Kilometer => convs::KILOMETER_TO_BASE,
        Unit::Meter => convs::METER_TO_BASE,
        Unit::Centimeter => convs::CENTIMETER_TO_BASE,
        Unit::Millimeter => convs::MILLIMETER_TO_BASE,
        // Imperial
        Unit::Foot => convs::FOOT_TO_BASE,
        Unit::Inch => convs::INCH_TO_BASE,
        Unit::Yard => convs::YARD_TO_BASE,
        Unit::Mile => convs::MILE_TO_BASE,
        // Other
        Unit::Lightyear => convs::LIGHTYEAR_TO_BASE,


        // Mass
        // Metric
        Unit::Kilogram => convs::KILOGRAM_TO_BASE,


        // Time
        Unit::Nanosecond => convs::NANOSECOND_TO_BASE,
        Unit::Microsecond => convs::MICROSECOND_TO_BASE,
        Unit::Millisecond => convs::MILLISECOND_TO_BASE,
        Unit::Second => convs::SECOND_TO_BASE,
        Unit::Minute => convs::MINUTE_TO_BASE,
        Unit::Hour => convs::HOUR_TO_BASE,

        // Temperature
        Unit::CelciusChange => convs::CELCIUS_TO_BASE,
        Unit::KelvinChange => convs::KELVIN_TO_BASE,
        Unit::FahrenheitChange => convs::FAHRENHEIT_TO_BASE,
        Unit::RankineChange => convs::RANKINE_TO_BASE,

        _ => {
            println!("UNKNOWN UNIT");
            println!("UNIT CONVERSION CONSTANT NOT SET!");
            0.0
        },
    }
}

pub fn base_to_unit_val(unit: &Unit) -> f64 {
    1.0 / unit_to_base_val(unit)
}

pub fn get_base_unit(unit: &Unit) -> Unit {
    let fundamental = get_fundamental(unit);
    match fundamental {
        Fundamental::Length => Unit::Meter,
        Fundamental::Mass => Unit::Kilogram,
        Fundamental::Time => Unit::Second,
        Fundamental::Temperature => Unit::CelciusChange,
        _ => {
            println!("NOT YET IMPLEMENTED");
            println!("get_base_unit function is not complete!");
            Unit::Meter
        },
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Unit {
    
    // Length
    // Metric
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    // Imperial
    Inch,
    Foot,
    Yard,
    Mile,
    // Common
    Lightyear,

    // Mass
    // Metric
    Kilogram,

    // Time
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,

    // Temperature
    Kelvin,
    Rankine,
    Celcius,
    Fahrenheit,
    // Change in Temperature
    KelvinChange,
    RankineChange,
    CelciusChange,
    FahrenheitChange,

    Temp,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {

            // Length
            // Metric
            Unit::Kilometer => "km",
            Unit::Meter => "m",
            Unit::Centimeter => "cm",
            Unit::Millimeter => "mm",
            // Imperial
            Unit::Inch => "in",
            Unit::Foot => "ft",
            Unit::Yard => "yard",
            Unit::Mile => "mile",
            // Other
            Unit::Lightyear => "lightyear",


            // Mass
            // Metric
            Unit::Kilogram => "kg",


            // Time
            Unit::Millisecond => "ms",
            Unit::Second => "sec",
            Unit::Minute => "min",
            Unit::Hour => "hr",

            // Temperature
            Unit::Kelvin => "K",
            Unit::Rankine => "°R",
            Unit::Celcius => "°C",
            Unit::Fahrenheit => "°F",
            // Change in Temperature
            Unit::KelvinChange => "ΔK",
            Unit::RankineChange => "Δ°R",
            Unit::CelciusChange => "Δ°C",
            Unit::FahrenheitChange => "Δ°F",

            _ => "Need to finish all the Unit strings",
        };
        write!(f, "{text}")
    }
}