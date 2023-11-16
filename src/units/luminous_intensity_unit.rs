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

use crate::units::AmountOfSubstanceUnit;
use crate::units::ElectricCurrentUnit;
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Clone, Debug, PartialEq)]
pub enum LuminousIntensityUnit {
    Candela,
    None,
}

impl LuminousIntensityUnit {
    pub fn conversion_factor(from: &LuminousIntensityUnit, to: &LuminousIntensityUnit) -> f64 {
        match from {
            Self::Candela => match to {
                Self::Candela => 1.0,
                Self::None => 1.0,
            },
            Self::None => 1.0,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LuminousIntensityUnit::Candela => "cd",
            LuminousIntensityUnit::None => "",
        }
    }
}

impl<
        T: IsEngUnitType
            + Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    > From<&T> for LuminousIntensityUnit
{
    fn from(value: &T) -> Self {
        if T::is_amount_unit() {
            value.into()
        } else {
            Self::None
        }
    }
}
impl IsEngUnitType for LuminousIntensityUnit {
    fn is_luminous_unit() -> bool {
        true
    }
}
impl From<AmountOfSubstanceUnit> for LuminousIntensityUnit {
    fn from(_: AmountOfSubstanceUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
impl From<ElectricCurrentUnit> for LuminousIntensityUnit {
    fn from(_: ElectricCurrentUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
impl From<LengthUnit> for LuminousIntensityUnit {
    fn from(_: LengthUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
impl From<MassUnit> for LuminousIntensityUnit {
    fn from(_: MassUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
impl From<TemperatureDeltaUnit> for LuminousIntensityUnit {
    fn from(_: TemperatureDeltaUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
impl From<TimeUnit> for LuminousIntensityUnit {
    fn from(_: TimeUnit) -> Self {
        LuminousIntensityUnit::None
    }
}
