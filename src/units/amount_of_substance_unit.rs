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

use crate::units::ElectricCurrentUnit;
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AmountOfSubstanceUnit {
    Mol,
    None,
}

impl AmountOfSubstanceUnit {
    pub fn conversion_factor(from: &AmountOfSubstanceUnit, to: &AmountOfSubstanceUnit) -> f64 {
        match from {
            Self::Mol => match to {
                Self::Mol => 1.0,
                Self::None => 1.0,
            },
            Self::None => 1.0,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            AmountOfSubstanceUnit::Mol => "mol",
            AmountOfSubstanceUnit::None => "",
        }
    }
}

impl IsEngUnitType for AmountOfSubstanceUnit {
    fn is_amount_unit() -> bool {
        true
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
    > From<&T> for AmountOfSubstanceUnit
{
    fn from(value: &T) -> Self {
        if T::is_amount_unit() {
            value.into()
        } else {
            Self::None
        }
    }
}
impl From<ElectricCurrentUnit> for AmountOfSubstanceUnit {
    fn from(_: ElectricCurrentUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
impl From<LengthUnit> for AmountOfSubstanceUnit {
    fn from(_: LengthUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
impl From<LuminousIntensityUnit> for AmountOfSubstanceUnit {
    fn from(_: LuminousIntensityUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
impl From<MassUnit> for AmountOfSubstanceUnit {
    fn from(_: MassUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
impl From<TemperatureDeltaUnit> for AmountOfSubstanceUnit {
    fn from(_: TemperatureDeltaUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
impl From<TimeUnit> for AmountOfSubstanceUnit {
    fn from(_: TimeUnit) -> Self {
        AmountOfSubstanceUnit::None
    }
}
