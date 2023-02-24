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
    // println!("Let's make our first EngUnit!");
    // let mut unit = EngUnit::new();
    // unit.value = 9.81; // <-- Value of the unit
    // unit.label = "Test Unit".to_string(); // <-- Optional label used for identification       
    // unit.push_unit(Unit::Meter, 1); // <-- Pushes meter to the numerator  
    // unit.push_unit(Unit::Second, -2); // <-- Pushes sec^2 to the denominator
    // println!("{unit}"); // <-- It worked!
    
    // // unit.label;         // <-- The label used for this unit
    // // unit.value;         // <-- The value of the unit as f64
    // // unit.units();       // <-- String representation of the units
    // // unit.has_name();    // <-- Checks if this unit has a known unit type
    // // unit.unit_name();   // <-- String representation of the unit name (ex. acceleration)

    // println!("{} has a common name: {}", unit.label, unit.has_name());
    // println!("{} is a unit of {}", unit.label, unit.unit_name());    
    // println!("{} value: {}", unit.label, unit.value);
    // println!("{} units: {}", unit.label, unit.units());
    // println!();
        
    // println!("Converting {} to km/hr^2", unit.label);
    // unit.change_unit(Unit::Kilometer);  // <-- Changes the length unit to km
    // unit.change_unit(Unit::Hour);       // <-- Changes the time unit to hr
    // println!("{unit}");                 // <-- Value is calculated automatically!

    println!("Adding and subtracting units is easy!");
    println!("The units don't need to be the same...");
    println!("But they do require the same fundamental dimension!");
    let mut unit_1 = EngUnit::new();
    unit_1.value = 12.0;
    unit_1.push_unit(Unit::Centimeter, 3);

    let mut unit_2 = EngUnit::new();
    unit_2.value = 40.0;
    unit_2.push_unit(Unit::Millimeter, 3);

    let mut unit_3 = unit_1.clone() + unit_2.clone();
    unit_3.label = "Result".to_string();

    println!();    
    println!("{unit_1} + {unit_2} = {unit_3}");
    println!();
    println!("{} has a common name: {}", unit_3.label, unit_3.has_name());
    println!("{} is a unit of {}", unit_3.label, unit_3.unit_name());    
    println!("{} value: {}", unit_3.label, unit_3.value);
    println!("{} units: {}", unit_3.label, unit_3.units());
    println!();

    println!("This will cause a panic!");
    let mut unit_1 = EngUnit::new();
    unit_1.value = 12.0;
    unit_1.push_unit(Unit::Centimeter, 3);

    let mut unit_2 = EngUnit::new();
    unit_2.value = 40.0;
    unit_2.push_unit(Unit::Millimeter, 2);

    let mut unit_3 = unit_1.clone() + unit_2.clone();

}
