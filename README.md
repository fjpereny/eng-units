# eng-units

## Rust library to build, calculate and convert custom engineering units.

[![License](https://img.shields.io/github/license/fjpereny/eng-units)](https://www.gnu.org/licenses/gpl-3.0)
[![Test-Ubuntu](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml)
[![Test-macOS](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml)
[![Test-Windows](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml)

## Example Usage
### Creating an engineering unit
```rust
let unit = EngUnit::from_unit(Unit::Meter, 1.0);
println!({unit});
```
```
1 m
```
### Changing unit type
```rust
let unit = EngUnit::from_unit(Unit::Meter, 1.0);
unit.change_unit(Unit::Millimeter);
println!({unit});
```
```
1000 mm
```
### Printing an engineering unit
```rust
println!({unit});
```
```
1000 mm
```
### Addition
#### Scalar Addition
```rust
let unit = EngUnit::from_unit(Unit::Meter, 1.0);
let unit_2 = unit + 2;
println!({unit_2});
```
```
3 m
```
#### Unit Addition
```rust
let unit_1 = EngUnit::from_unit(Unit::Meter, 100.0);
let unit_2 = EngUnit::from_unit(Unit::Kilometer, 1.0);
let unit_3 = unit_2 + unit_1;
println!({unit_3});
```
```
1.1 km
```

### Multiplication
#### Scalar Multiplication
```rust
let unit_1 = EngUnit::from_unit(Unit::Millimeter, 1000.0);
let unit_2 = 2 * unit;
println!({unit_2});
```
```
2000 mm
```
#### Unit Multiplication
```rust
let unit_1 = EngUnit::from_unit(Unit::Meter, 100.0);
let unit_2 = EngUnit::from_unit(Unit::Centimeter, 500.0);
let unit_3 = unit_1 * unit_2;
println!({unit_3});
```
```
500 m^2
```

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
