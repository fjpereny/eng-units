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

pub mod compound_macros;
pub mod compound_strings;

use crate::AmountOfSubstanceUnit;
use crate::ElectricCurrentUnit;
use crate::EngUnit;
use crate::LengthUnit;
use crate::LuminousIntensityUnit;
use crate::MassUnit;
use crate::TemperatureDeltaUnit;
use crate::TimeUnit;

pub struct CompoundUnit {
    unit: EngUnit,
    str: String,
}

impl CompoundUnit {
    pub fn new(
        str: String,
        amount_of_substance_count: i32,
        amount_of_substance_unit: AmountOfSubstanceUnit,
        electric_current_count: i32,
        electric_current_unit: ElectricCurrentUnit,
        length_count: i32,
        length_unit: LengthUnit,
        luminous_intensity_count: i32,
        luminous_intensity_unit: LuminousIntensityUnit,
        mass_count: i32,
        mass_unit: MassUnit,
        temperature_count: i32,
        temperature_unit: TemperatureDeltaUnit,
        time_count: i32,
        time_unit: TimeUnit,
    ) -> Self {
        let unit = EngUnit {
            value: 1.0,
            amount_of_substance_count,
            amount_of_substance_unit,
            electric_current_count,
            electric_current_unit,
            length_count,
            length_unit,
            luminous_intensity_count,
            luminous_intensity_unit,
            mass_count,
            mass_unit,
            temperature_count,
            temperature_unit,
            time_count,
            time_unit,
        };

        Self { unit, str }
    }
}
#[cfg(test)]
mod tests {
    use crate::AmountOfSubstanceUnit;
    use crate::ElectricCurrentUnit;
    use crate::EngUnit;
    use crate::LengthUnit;
    use crate::LuminousIntensityUnit;
    use crate::MassUnit;
    use crate::TemperatureDeltaUnit;
    use crate::TimeUnit;
    use crate::{enthalpy_J, entropy_J};

    #[test]
    fn test_1() {
        let unit = enthalpy_J!(123.45);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 kg·m^2/s^2", unit.to_string());
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::Meter, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::Kilogram, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::None, unit.temperature_unit);
        assert_eq!(TimeUnit::Second, unit.time_unit);
        assert_eq!(0, unit.temperature_count);
        assert_eq!(-2, unit.time_count);
        assert_eq!(2, unit.length_count);
        assert_eq!(1, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn test_2() {
        let unit = entropy_J!(123.45);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 kg·m^2/s^2·K", unit.to_string());
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::Meter, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::Kilogram, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::None, unit.temperature_unit);
        assert_eq!(TimeUnit::Second, unit.time_unit);
        assert_eq!(0, unit.temperature_count);
        assert_eq!(-2, unit.time_count);
        assert_eq!(2, unit.length_count);
        assert_eq!(1, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }
}
