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

use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
use crate::units::electric_current_unit::ElectricCurrentUnit;
use crate::units::length_unit::LengthUnit;
use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
use crate::units::mass_unit::MassUnit;
use crate::units::temperature_unit::TemperatureDeltaUnit;
use crate::units::time_unit::TimeUnit;
use crate::EngUnit;

pub struct ComplexUnit {
    pub prefix_multiplier: f64,
    pub amount_of_substance_count: i32,
    pub amount_of_substance_unit: AmountOfSubstanceUnit,
    pub electric_current_count: i32,
    pub electric_current_unit: ElectricCurrentUnit,
    pub length_count: i32,
    pub length_unit: LengthUnit,
    pub luminous_intensity_count: i32,
    pub luminous_intensity_unit: LuminousIntensityUnit,
    pub mass_count: i32,
    pub mass_unit: MassUnit,
    pub temperature_count: i32,
    pub temperature_unit: TemperatureDeltaUnit,
    pub time_count: i32,
    pub time_unit: TimeUnit,
    pub unit_string: &'static str,
}

pub const JOULE: ComplexUnit = ComplexUnit {
    prefix_multiplier: 1.0,
    amount_of_substance_count: 0,
    amount_of_substance_unit: AmountOfSubstanceUnit::None,
    electric_current_count: 0,
    electric_current_unit: ElectricCurrentUnit::None,
    length_count: 2,
    length_unit: LengthUnit::Meter,
    luminous_intensity_count: 0,
    luminous_intensity_unit: LuminousIntensityUnit::None,
    mass_count: 1,
    mass_unit: MassUnit::Kilogram,
    temperature_count: 0,
    temperature_unit: TemperatureDeltaUnit::None,
    time_count: -2,
    time_unit: TimeUnit::Second,
    unit_string: "J",
};

pub const KILOJOULE: ComplexUnit = ComplexUnit {
    prefix_multiplier: 1.0 / 1000.0,
    amount_of_substance_count: 0,
    amount_of_substance_unit: AmountOfSubstanceUnit::None,
    electric_current_count: 0,
    electric_current_unit: ElectricCurrentUnit::None,
    length_count: 2,
    length_unit: LengthUnit::Meter,
    luminous_intensity_count: 0,
    luminous_intensity_unit: LuminousIntensityUnit::None,
    mass_count: 1,
    mass_unit: MassUnit::Kilogram,
    temperature_count: 0,
    temperature_unit: TemperatureDeltaUnit::None,
    time_count: -2,
    time_unit: TimeUnit::Second,
    unit_string: "kJ",
};

pub fn can_extract_normal(unit: &EngUnit, complex: &ComplexUnit) -> bool {
    if complex.amount_of_substance_count > 0 {
        if unit.amount_of_substance_count < complex.amount_of_substance_count {
            return false;
        }
    } else if complex.amount_of_substance_count < 0 {
        if unit.amount_of_substance_count > complex.amount_of_substance_count {
            return false;
        }
    }

    if complex.electric_current_count > 0 {
        if unit.electric_current_count < complex.electric_current_count {
            return false;
        }
    } else if complex.electric_current_count < 0 {
        if unit.electric_current_count > complex.electric_current_count {
            return false;
        }
    }

    if complex.length_count > 0 {
        if unit.length_count < complex.length_count {
            return false;
        }
    } else if complex.length_count < 0 {
        if unit.length_count > complex.length_count {
            return false;
        }
    }

    if complex.luminous_intensity_count > 0 {
        if unit.luminous_intensity_count < complex.luminous_intensity_count {
            return false;
        }
    } else if complex.luminous_intensity_count < 0 {
        if unit.luminous_intensity_count > complex.luminous_intensity_count {
            return false;
        }
    }

    if complex.mass_count > 0 {
        if unit.mass_count < complex.mass_count {
            return false;
        }
    } else if complex.mass_count < 0 {
        if unit.mass_count > complex.mass_count {
            return false;
        }
    }

    if complex.temperature_count > 0 {
        if unit.temperature_count < complex.temperature_count {
            return false;
        }
    } else if complex.temperature_count < 0 {
        if unit.temperature_count > complex.temperature_count {
            return false;
        }
    }

    if complex.time_count > 0 {
        if unit.time_count < complex.time_count {
            return false;
        }
    } else if complex.time_count < 0 {
        if unit.time_count > complex.time_count {
            return false;
        }
    }
    true
}

pub fn extract_normal(unit: &EngUnit, complex: ComplexUnit) -> Option<EngUnit> {
    if !can_extract_normal(&unit, &complex) {
        return None;
    }
    let new_unit = unit.convert(complex.amount_of_substance_unit);
    let new_unit = new_unit.convert(complex.electric_current_unit);
    let new_unit = new_unit.convert(complex.length_unit);
    let new_unit = new_unit.convert(complex.luminous_intensity_unit);
    let new_unit = new_unit.convert(complex.mass_unit);
    let new_unit = new_unit.convert(complex.temperature_unit);
    let new_unit = new_unit.convert(complex.time_unit);

    let mut new_unit = new_unit;
    new_unit.value *= complex.prefix_multiplier;
    new_unit.amount_of_substance_count -= complex.amount_of_substance_count;
    new_unit.electric_current_count -= complex.electric_current_count;
    new_unit.length_count -= complex.length_count;
    new_unit.luminous_intensity_count -= complex.luminous_intensity_count;
    new_unit.mass_count -= complex.mass_count;
    new_unit.temperature_count -= complex.temperature_count;
    new_unit.time_count -= complex.time_count;

    new_unit
        .unit_string_numerator
        .push(complex.unit_string.to_string());
    Some(new_unit)
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_1() {
        let u1 = kJ!(1.0);
        let u2 = extract_normal(&u1, JOULE);
        assert!(u2.is_some());
        let u2 = u2.unwrap();
        assert_eq!(1000.0, u2.value);
        assert_eq!("1000.00 J", u2.to_string());
    }
}
