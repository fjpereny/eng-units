# eng-units

## Rust library to build, calculate and convert custom engineering units.

[![License](https://img.shields.io/github/license/fjpereny/eng-units)](https://www.gnu.org/licenses/gpl-3.0)
[![Test-Ubuntu](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml)
[![Test-macOS](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml)
[![Test-Windows](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml)

## This is a work in process!
### Only a few units have been implemented for development purposes!
#### Feel free to review and contribute on the repository!

## Example Usage
### Making a EngUnit
#### Example Code
```rust
println!("Let's make our first EngUnit!");
let mut unit = EngUnit::new();
unit.value = 9.81; // <-- Value of the unit
unit.label = "Test Unit".to_string(); // <-- Optional label used for identification       
unit.push_unit(Unit::Meter, 1); // <-- Pushes meter to the numerator  
unit.push_unit(Unit::Second, -2); // <-- Pushes sec^2 to the denominator
println!("{unit}"); // <-- It worked!
```
#### Output
```
Let's make our first EngUnit!
Test Unit: 9.81 (m)/(sec^2)
```
### Using EngUnit
```rust
unit.label;         // <-- The label used for this unit
unit.value;         // <-- The value of the unit as f64
unit.units();       // <-- String representation of the units
unit.has_name();    // <-- Checks if this unit has a known unit type
unit.unit_name();   // <-- String representation of the unit name (ex. acceleration)
```
#### Example Code
```rust
println!("{} has a common name: {}", unit.label, unit.has_name());
println!("{} is a unit of {}", unit.label, unit.unit_name());    
println!("{} value: {}", unit.label, unit.value);
println!("{} units: {}", unit.label, unit.units());
println!();
```
#### Output
```
Test Unit has a common name: true
Test Unit is a unit of acceleration
Test Unit value: 9.81
Test Unit units: (m)/(sec^2)
```
### Converting Units
#### Example Code
```rust
println!("Converting {} to km/hr^2", unit.label);
unit.change_unit(Unit::Kilometer);  // <-- Changes the length unit to km
unit.change_unit(Unit::Hour);       // <-- Changes the time unit to hr
println!("{unit}");                 // <-- Value is calculated automatically!
```
#### Output
```
Converting Test Unit to km/hr^2
Test Unit: 127137.6 (km)/(hr^2)
```
### Adding Units
#### Example Code
```rust
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
```
#### Output
```
Adding and subtracting units is easy!
The units don't need to be the same...
But they do require the same fundamental dimension!

12 cm^3 + 40 mm^3 = Result: 12.04 cm^3

Result has a common name: true
Result is a unit of volume
Result value: 12.04
Result units: cm^3
```
#### AddAssign
```rust
println!("AddAssign operators work too!");
let mut u1 = EngUnit::new();
u1.value = 1.0;
u1.push_unit(Unit::Kilometer, 1);

let mut u2 = EngUnit::new();
u2.value = 553.0;
u2.push_unit(Unit::Meter, 1);

u1 += u2;
println!("{u1}");
```
#### Output
```
AddAssign operators work too!
1.553 km
```
#### Example Error
Adding incompatible units (different fundamental dimmension) will raise a panic!
```rust
println!("This will cause a panic!");
let mut unit_1 = EngUnit::new();
unit_1.value = 12.0;
unit_1.push_unit(Unit::Centimeter, 3); // <-- L^3

let mut unit_2 = EngUnit::new();
unit_2.value = 40.0;
unit_2.push_unit(Unit::Millimeter, 2); // <-- L^2

let mut unit_3 = unit_1.clone() + unit_2.clone(); // <-- L^3 + L^2 = ??
```
#### Output
```
thread 'main' panicked at 'Tried to add incomaptible units (length)'
```

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
