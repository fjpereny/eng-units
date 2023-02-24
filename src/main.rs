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

use crate::units::Unit;
use crate::units::EngUnit;

fn main() {

    let mut u1 = EngUnit::new();
    u1.value = 5.0;
    u1.push_unit(Unit::Kilometer, 2);

    let mut u2 = EngUnit::new();
    u2.value = 6.25;
    u2.push_unit(Unit::Second, -2);

    u1 *= u2;
    println!("{u1}");

}
