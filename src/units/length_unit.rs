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
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LengthUnit {
    Meter,
    Foot,
    None,
}

impl LengthUnit {
    pub fn conversion_factor(from: &LengthUnit, to: &LengthUnit) -> f64 {
        match from {
            Self::Meter => match to {
                Self::Meter => 1.0,
                Self::Foot => 3.28084,
                Self::None => 1.0,
            },
            Self::Foot => match to {
                Self::Meter => 1.0 / 3.28084,
                Self::Foot => 1.0,
                Self::None => 1.0,
            },
            Self::None => 1.0,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LengthUnit::Meter => "m",
            LengthUnit::Foot => "ft",
            LengthUnit::None => "",
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
    > From<&T> for LengthUnit
{
    fn from(value: &T) -> Self {
        if T::is_amount_unit() {
            value.into()
        } else {
            Self::None
        }
    }
}
impl IsEngUnitType for LengthUnit {
    fn is_length_unit() -> bool {
        true
    }
}
impl From<AmountOfSubstanceUnit> for LengthUnit {
    fn from(_: AmountOfSubstanceUnit) -> Self {
        LengthUnit::None
    }
}
impl From<ElectricCurrentUnit> for LengthUnit {
    fn from(_: ElectricCurrentUnit) -> Self {
        LengthUnit::None
    }
}
impl From<LuminousIntensityUnit> for LengthUnit {
    fn from(_: LuminousIntensityUnit) -> Self {
        LengthUnit::None
    }
}
impl From<MassUnit> for LengthUnit {
    fn from(_: MassUnit) -> Self {
        LengthUnit::None
    }
}
impl From<TemperatureDeltaUnit> for LengthUnit {
    fn from(_: TemperatureDeltaUnit) -> Self {
        LengthUnit::None
    }
}
impl From<TimeUnit> for LengthUnit {
    fn from(_: TimeUnit) -> Self {
        LengthUnit::None
    }
}
