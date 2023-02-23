

pub mod units;
pub mod eng_units;
pub mod conversions;
pub mod fundamental;

use crate::eng_units::EngUnit;
use crate::units::Unit;

fn main() {

    let mut unit_1 = EngUnit::from_str("1 m").unwrap();
    
    println!("{unit_1}");
    println!("{} is a unit of {}", &unit_1, &unit_1.unit_name());
    unit_1.change_unit(Unit::Kilometer);
    println!("{} is a unit of {}", &unit_1, &unit_1.unit_name());
    unit_1.change_unit(Unit::Millimeter);
    println!("{} is a unit of {}", &unit_1, &unit_1.unit_name());
    unit_1.change_unit(Unit::Centimeter);
    println!("{} is a unit of {}", &unit_1, &unit_1.unit_name());

}
