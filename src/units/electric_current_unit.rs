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
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ElectricCurrentUnit {
    Ampere,
    None,
}

impl ElectricCurrentUnit {
    pub fn conversion_factor(from: &ElectricCurrentUnit, to: &ElectricCurrentUnit) -> f64 {
        match from {
            Self::Ampere => match to {
                Self::Ampere => 1.0,
                Self::None => 1.0,
            },
            Self::None => 1.0,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ElectricCurrentUnit::Ampere => "A",
            ElectricCurrentUnit::None => "",
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
    > From<&T> for ElectricCurrentUnit
{
    fn from(value: &T) -> ElectricCurrentUnit {
        if T::is_amount_unit() {
            value.into()
        } else {
            ElectricCurrentUnit::None
        }
    }
}
impl IsEngUnitType for ElectricCurrentUnit {
    fn is_electric_current_unit() -> bool {
        true
    }
}
impl From<AmountOfSubstanceUnit> for ElectricCurrentUnit {
    fn from(_: AmountOfSubstanceUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
impl From<LengthUnit> for ElectricCurrentUnit {
    fn from(_: LengthUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
impl From<LuminousIntensityUnit> for ElectricCurrentUnit {
    fn from(_: LuminousIntensityUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
impl From<MassUnit> for ElectricCurrentUnit {
    fn from(_: MassUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
impl From<TemperatureDeltaUnit> for ElectricCurrentUnit {
    fn from(_: TemperatureDeltaUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
impl From<TimeUnit> for ElectricCurrentUnit {
    fn from(_: TimeUnit) -> Self {
        ElectricCurrentUnit::None
    }
}
