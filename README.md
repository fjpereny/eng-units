# eng-units

## Rust library to build, calculate and convert custom engineering units.

[![License](https://img.shields.io/github/license/fjpereny/eng-units)](https://www.gnu.org/licenses/gpl-3.0)
[![Test-Ubuntu](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml)
[![Test-macOS](https://github.com/fjpereny/eng-units/actions/workflows/test_macOS.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_ubuntu.yml)
[![Test-Windows](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml/badge.svg)](https://github.com/fjpereny/eng-units/actions/workflows/test_windows.yml)

## Example Usage
### Making a EngUnit
```rust
let temp_1 = temperature!(1.0, TemperatureDeltaUnit::K);
let mass_1 = mass!(1.0, MassUnit::Pound);
let t_1 = time!(1.0, TimeUnit::Minute);
let unit = temp_1 * mass_1 / t_1 * 123.45;
assert_eq!(123.45, unit.value);
assert_eq!("123.45 lb·K/min", unit.to_string());
```

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
