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
pub mod conversions;
pub mod fundamental;
pub mod unit_names;

use crate::conversions::Unit;
use crate::units::EngUnit;

fn main() {

    let mut x = EngUnit::from_unit(1.0, Unit::Meter, 1);
    x.push_unit(Unit::Second, -2);
    println!("{x}");
    x *= 9.81;
    x.label = "Gravity".to_string();
    println!("{x} is a unit of {}", x.unit_name());

    let mut y = EngUnit::from_unit(2.73, Unit::Foot, 1);
    y.push_unit(Unit::Second, -2);
    println!("{y}");

    let mut z = x + y;
    z.label = "New Acc.".to_string();
    println!("{z}");

    z.change_unit(Unit::Inch);
    println!("{z}");
}



