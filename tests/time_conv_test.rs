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


use eng_units::*;

#[cfg(test)]
pub mod times {
    use crate::units::*;

    #[test]
    fn time_change_second_to_nanosecond() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Nanosecond);
        assert_eq!(unit.value, 1_000_000_000.0);
    }
    #[test]
    fn time_change_second_to_microseconds() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Microsecond);
        assert_eq!(unit.value, 1_000_000.0);
    }
    #[test]
    fn time_change_second_to_milliseconds() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Millisecond);
        assert_eq!(unit.value, 1_000.0);
    }
    #[test]
    fn time_change_second_to_seconds() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Second);
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn time_change_second_to_minutes() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Minute);
        assert_eq!(unit.value, 1.0 / 60.0);
    }
    #[test]
    fn time_change_second_to_hours() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Second, 1);
        unit.change_unit(Unit::Hour);
        assert_eq!(unit.value, 1.0 / 3600.0);
    }
}