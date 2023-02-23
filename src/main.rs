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
