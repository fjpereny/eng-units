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

use crate::s;
use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
use crate::units::electric_current_unit::ElectricCurrentUnit;
use crate::units::length_unit::LengthUnit;
use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
use crate::units::mass_unit::MassUnit;
use crate::units::temperature_unit::TemperatureDeltaUnit;
use crate::units::time_unit::TimeUnit;
use crate::EngUnit;
use crate::J;

#[macro_export]
macro_rules! W {
    ($value:expr) => {{
        let j = J!($value);
        let s = s!(1.0);
        j / s
    }};
}

#[macro_export]
macro_rules! kW {
    ($value:expr) => {{
        let j = J!($value * 1000.0);
        let s = s!(1.0);
        j / s
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = W!(1.0);
        assert_eq!(1.0, x.value);
        assert!(x.has_units());
        assert_eq!("1.00 kg·m^2/s^3", x.to_string());
    }

    #[test]
    fn test_2() {
        let x = kW!(1.0);
        assert_eq!(1000.0, x.value);
        assert!(x.has_units());
        assert_eq!("1000.00 kg·m^2/s^3", x.to_string());
    }

    #[test]
    fn test_3() {
        let x = kW!(1.0);
        let y = W!(1.0);
        let z = x / y;
        assert_eq!(1000.0, z.value);
        assert!(!z.has_units());
        assert_eq!("1000.00", z.to_string());
    }
}
