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

use crate::complex_units::ComplexUnit;
use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
use crate::units::electric_current_unit::ElectricCurrentUnit;
use crate::units::length_unit::LengthUnit;
use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
use crate::units::mass_unit::MassUnit;
use crate::units::temperature_unit::TemperatureDeltaUnit;
use crate::units::time_unit::TimeUnit;

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
        push_complex_numerator(&unit, JOULE).unwrap()
    }};
}

#[macro_export]
macro_rules! kJ {
    ($value:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value / KILOJOULE.prefix_multiplier;
        unit.mass_count = 1;
        unit.length_count = 2;
        unit.time_count = -2;
        unit.mass_unit = MassUnit::Kilogram;
        unit.length_unit = LengthUnit::Meter;
        unit.time_unit = TimeUnit::Second;
        push_complex_numerator(&unit, KILOJOULE).unwrap()
    }};
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

#[cfg(test)]

mod tests {
    use super::*;
    use crate::complex_units::*;

    mod kj {
        use crate::temperature;

        use super::*;

        #[test]
        fn creation() {
            let u1 = kJ!(12.345);
            assert_eq!(12.345, u1.value);
            assert_eq!("12.35 kJ", u1.to_string());
            let u2 = u1.to_si_units();
            assert_eq!(12345.0, u2.value);
            assert_eq!("12345.00 kg·m^2/s^2", u2.to_string());
        }

        #[test]
        fn multiply_temp() {
            let u1 = kJ!(12.345);
            let u2 = temperature!(2.0, TemperatureDeltaUnit::K);
            let u3 = u1 * u2;
            assert_eq!(24.690, u3.value);
            assert_eq!("24.69 kJ·K", u3.to_string());
        }

        #[test]
        fn divide_temp() {
            let u1 = kJ!(12.345);
            let u2 = temperature!(10.0, TemperatureDeltaUnit::K);
            let u3 = u1 / u2;
            let expected = 1.2345;
            let delta = (u3.value - expected).abs();
            assert!(delta < 0.00001);
            assert_eq!("1.23 kJ/K", u3.to_string());
        }

        #[test]
        fn multiply_self() {
            let u1 = kJ!(2.0);
            let u2 = kJ!(3.0);
            let u3 = u1 * u2;
            assert_eq!(6.0, u3.value);
            assert_eq!("6.00 kJ^2", u3.to_string());
        }
    }
}
