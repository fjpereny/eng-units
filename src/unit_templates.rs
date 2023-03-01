


use crate::units::*;
use crate::conversions::*;

pub fn build_unit(value: f64, unit: Unit, power: i8) -> EngUnit {
    let mut new_unit = EngUnit::new();
    new_unit.value = value;

    let (num, den) = get_template(unit);
    if power > 0 {       
        for _ in 0..power {
            for n in &num {
                new_unit.push_unit(*n, 1);
            }
            for d in &den {
                new_unit.push_unit(*d, -1);
            }
        }
    }
    if power < 0 {
        for _ in 0..power.abs() {
            for n in &num {
                new_unit.push_unit(*n, -1);
            }
            for d in &den {
                new_unit.push_unit(*d, 1);
            }
        }
    }
    new_unit
}


pub fn get_template(unit: Unit) -> (Vec<Unit>, Vec<Unit>) {
    let mut num: Vec<Unit> = Vec::new();
    let mut den: Vec<Unit> = Vec::new();
    match unit {
        // Length
        // Metric
        Unit::Kilometer => num.push(Unit::Kilometer),
        Unit::Meter => num.push(Unit::Meter),
        Unit::Centimeter => num.push(Unit::Centimeter),
        Unit::Millimeter => num.push(Unit::Millimeter),
        //  Imperial
        Unit::Inch => num.push(Unit::Inch),
        Unit::Foot => num.push(Unit::Foot),
        Unit::Yard => num.push(Unit::Yard),
        Unit::Mile => num.push(Unit::Mile),
        // Other
        Unit::Lightyear => num.push(Unit::Lightyear),


        // Mass
        // Metric
        Unit::MetricTonne => num.push(Unit::MetricTonne),
        Unit::Kilogram => num.push(Unit::Kilogram),
        Unit::Gram => num.push(Unit::Gram),
        // Imperial
        Unit::PoundMass => num.push(Unit::PoundMass),
        Unit::Slug => num.push(Unit::Slug),
        Unit::ShortTon => num.push(Unit::ShortTon),
        Unit::LongTon => num.push(Unit::LongTon),


        // Time
        Unit::Second => num.push(Unit::Second),
        Unit::Minute => num.push(Unit::Minute),
        Unit::Hour => num.push(Unit::Hour),


        // Temperature
        Unit::Kelvin => num.push(Unit::Kelvin),
        Unit::Rankine => num.push(Unit::Rankine),
        Unit::Celcius => num.push(Unit::Celcius),
        Unit::Fahrenheit => num.push(Unit::Fahrenheit),
        // Change in Temperature
        Unit::KelvinChange => num.push(Unit::KelvinChange),
        Unit::RankineChange => num.push(Unit::RankineChange),
        Unit::CelciusChange => num.push(Unit::CelciusChange),
        Unit::FahrenheitChange => num.push(Unit::FahrenheitChange),


        Unit::Newton => {
            num.push(Unit::Kilogram);
            num.push(Unit::Meter);
            den.push(Unit::Second);
            den.push(Unit::Second);
        },
        _ => {},
    }
    (num, den)
}