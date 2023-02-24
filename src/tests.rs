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


#[cfg(test)]
mod tests {
    use crate::units::Unit;
    use crate::units::EngUnit;

    #[test]
    fn length_change_m_to_km_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Kilometer);
        assert_eq!(unit.value, 0.001);
    }

    #[test]
    fn length_change_m_to_km_to_str() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Kilometer);
        assert_eq!(unit.to_string(), "0.001 km");
    }

    #[test]
    fn length_change_m_to_cm_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Centimeter);
        assert_eq!(unit.value, 100.0);
    }

    #[test]
    fn length_change_m_to_cm_to_str() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Centimeter);
        assert_eq!(unit.to_string(), "100 cm");
    }

    #[test]
    fn length_change_m_to_mm_val() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Millimeter);
        assert_eq!(unit.value, 1000.0);
    }

    #[test]
    fn length_change_m_to_mm_to_str() {
        let mut unit = EngUnit::from_unit(1.0, Unit::Meter, 1);
        unit.change_unit(Unit::Millimeter);
        assert_eq!(unit.to_string(), "1000 mm");
    }

}