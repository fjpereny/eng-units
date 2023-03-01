


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
        Unit::Meter => num.push(Unit::Meter),
        Unit::Kilometer => num.push(Unit::Kilometer),

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