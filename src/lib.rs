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

pub mod compound_units;
pub mod units;

pub use units::amount_of_substance_unit::AmountOfSubstanceUnit;
pub use units::electric_current_unit::ElectricCurrentUnit;
pub use units::length_unit::LengthUnit;
pub use units::luminous_intensity_unit::LuminousIntensityUnit;
pub use units::mass_unit::MassUnit;
pub use units::temperature_unit::TemperatureDeltaUnit;
pub use units::time_unit::TimeUnit;
pub use units::EngUnit;

#[cfg(test)]
mod tests {
    use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
    use crate::units::electric_current_unit::ElectricCurrentUnit;
    use crate::units::length_unit::LengthUnit;
    use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
    use crate::units::mass_unit::MassUnit;
    use crate::units::temperature_unit::TemperatureDeltaUnit;
    use crate::units::time_unit::TimeUnit;
    use crate::units::EngUnit;
    use crate::{mass, temperature, time};

    #[test]
    fn example_test_1() {
        let quarter_pounder = mass!(0.25, MassUnit::Pound);
        assert_eq!(0.25, quarter_pounder.value);
        assert_eq!("0.25 lb", quarter_pounder.to_string());
    }

    #[test]
    fn example_test_2() {
        let quarter_pounder = mass!(0.25, MassUnit::Pound);
        let royal_with_cheese = quarter_pounder.convert(MassUnit::Kilogram);
        assert_eq!("0.11 kg", royal_with_cheese.to_string());
    }

    #[test]
    fn example_test_3() {
        let temp_1 = temperature!(4.0, TemperatureDeltaUnit::C);
        let mass_1 = mass!(5.0, MassUnit::Kilogram);
        let t_1 = time!(10.0, TimeUnit::Second);

        let unit = temp_1 * mass_1 / t_1;
        assert_eq!(2.0, unit.value);
        assert_eq!("2.00 kg·°C/s", unit.to_string());

        let double = 2.0 * unit;
        assert_eq!(4.0, double.value);
        assert_eq!("4.00 kg·°C/s", double.to_string());
    }

    #[test]
    fn example_test_4() {
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
    }
}
