// eng-units - engineering unit conversion and calculation library
// Copyright (C) 2023 Frank Pereny

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! J {
    ($value:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.mass_count = 1;
        unit.length_count = 2;
        unit.time_count = -2;
        unit.mass_unit = MassUnit::Kilogram;
        unit.length_unit = LengthUnit::Meter;
        unit.time_unit = TimeUnit::Second;
        unit
    }};
}

#[macro_export]
macro_rules! kJ {
    ($value:literal) => {{
        J! {($value * 1000.0)}
    }};
}
