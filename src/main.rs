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
pub mod unit_templates;

use crate::conversions::*;
use crate::units::EngUnit;

fn main() {

    let mut u1 = EngUnit::from_unit(1.0, Unit::Newton, 2);

    let mut u2 = EngUnit::from_unit(0.3, Unit::Newton, -1);
    println!("{u2}");

    u2.inverse();
    println!("{}", u2.unit_name());

    // u1 *= u2;

    // println!("{u1}");

}



