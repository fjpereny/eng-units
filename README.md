# eng-units

## Rust library to build, calculate and convert custom engineering units.

[![License](https://img.shields.io/github/license/fjpereny/eng-units)](https://www.gnu.org/licenses/gpl-3.0)
[![Test-Ubuntu](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml)
[![Test-macOS](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml)
[![Test-Windows](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml)

## Project Goals
- Easy to use engineering units and calculations
- No panics
- No dependencies
- 100% testing coverage

## Example Usage
### Creating units
```rust
let quarter_pounder = mass!(0.25, MassUnit::Pound);
assert_eq!(0.25, quarter_pounder.value);
assert_eq!("0.25 lb", quarter_pounder.to_string());
```
### Converting units 
```rust
let quarter_pounder = mass!(0.25, MassUnit::Pound);
let royal_with_cheese = quarter_pounder.convert(MassUnit::Kilogram);
assert_eq!("0.11 kg", royal_with_cheese.to_string());
```
### Engineering calculations 
```rust
let temp_1 = temperature!(4.0, TemperatureDeltaUnit::C);
let mass_1 = mass!(5.0, MassUnit::Kilogram);
let t_1 = time!(10.0, TimeUnit::Second);

let unit = temp_1 * mass_1 / t_1;
assert_eq!(2.0, unit.value);
assert_eq!("2.00 kg·°C/s", unit.to_string());

let double = 2.0 * unit;
assert_eq!(4.0, double.value);
assert_eq!("4.00 kg·°C/s", double.to_string());

let flip = double.reciprocal();
assert_eq!(0.25, flip.value);
assert_eq!("0.25 s/kg·°C", flip.to_string());

let mass_2 = mass!(10.0, MassUnit::Kilogram);
let canceled_out = flip * mass_2;
assert_eq!(2.50, canceled_out.value);
assert_eq!("2.50 s/°C", canceled_out.to_string());
```

## Test coverage status


## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
