pub mod units;
pub mod conversions;
pub mod fundamental;
pub mod unit_names;
pub mod unit_templates;

use crate::units::*;
use crate::conversions::*;

fn main() {
    let mut u1 = EngUnit::from_unit(1.0, Unit::Meter, 1);
    u1.set_label("Hello, world!");
    println!("{u1}")
}