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
pub mod lengths {
    use crate::units::*;

    #[test]
    fn length_change_m_to_m_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Meter);
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn length_change_m_to_km_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Kilometer);
        assert_eq!(unit.value, 0.001);
    }
    #[test]
    fn length_change_m_to_cm_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Centimeter);
        assert_eq!(unit.value, 100.0);
    }
    #[test]
    fn length_change_m_to_mm_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Millimeter);
        assert_eq!(unit.value, 1000.0);
    }
    #[test]
    fn change_meter_to_feet() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Foot);
        assert_eq!(unit.value, (1.0) / (0.3048));
    }
    #[test]
    fn change_meter_to_inch() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Inch);
        assert_eq!(unit.value, (1.0) / (0.3048) * 12.0);
    }
    #[test]
    fn change_meter_to_yard() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Yard);
        assert_eq!(unit.value, (1.0) / (0.3048) / 3.0);
    }
    #[test]
    fn change_meter_to_mile() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Mile);
        assert_eq!(unit.value, (1.0) / (0.3048) / 5280.0);
    }
}