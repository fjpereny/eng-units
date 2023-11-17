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
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;

#[macro_export]
macro_rules! time {
    ($value:literal, $unit:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.time_count = 1;
        if $unit == TimeUnit::None {
            unit.time_count = 0;
        }
        unit.time_unit = $unit;
        unit
    }};
}

#[macro_export]
macro_rules! s {
    ($value:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.time_count = 1;
        unit.time_unit = TimeUnit::Second;
        unit
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    None,
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
    > From<&T> for TimeUnit
{
    fn from(value: &T) -> Self {
        if T::is_amount_unit() {
            value.into()
        } else {
            Self::None
        }
    }
}
impl IsEngUnitType for TimeUnit {
    fn is_time_unit() -> bool {
        true
    }
}
impl From<AmountOfSubstanceUnit> for TimeUnit {
    fn from(_: AmountOfSubstanceUnit) -> Self {
        TimeUnit::None
    }
}
impl From<ElectricCurrentUnit> for TimeUnit {
    fn from(_: ElectricCurrentUnit) -> Self {
        TimeUnit::None
    }
}
impl From<LengthUnit> for TimeUnit {
    fn from(_: LengthUnit) -> Self {
        TimeUnit::None
    }
}
impl From<LuminousIntensityUnit> for TimeUnit {
    fn from(_: LuminousIntensityUnit) -> Self {
        TimeUnit::None
    }
}
impl From<MassUnit> for TimeUnit {
    fn from(_: MassUnit) -> Self {
        TimeUnit::None
    }
}
impl From<TemperatureDeltaUnit> for TimeUnit {
    fn from(_: TemperatureDeltaUnit) -> Self {
        TimeUnit::None
    }
}

pub const MINUTE_TO_SECONDS: f64 = 60.0;
pub const HOUR_TO_SECONDS: f64 = 3600.0;
pub const HOUR_TO_MINUTES: f64 = 60.0;

impl TimeUnit {
    pub fn to_string(&self) -> &'static str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "hr",
            TimeUnit::None => "",
        }
    }

    pub fn conversion_factor(from: &TimeUnit, to: &TimeUnit) -> f64 {
        match from {
            TimeUnit::Second => match to {
                TimeUnit::Second => 1.0,
                TimeUnit::Minute => 1.0 / MINUTE_TO_SECONDS,
                TimeUnit::Hour => 1.0 / HOUR_TO_SECONDS,
                TimeUnit::None => 1.0,
            },
            TimeUnit::Minute => match to {
                TimeUnit::Minute => 1.0,
                TimeUnit::Second => MINUTE_TO_SECONDS,
                TimeUnit::Hour => HOUR_TO_MINUTES,
                TimeUnit::None => 1.0,
            },
            TimeUnit::Hour => match to {
                TimeUnit::Minute => HOUR_TO_MINUTES,
                TimeUnit::Second => HOUR_TO_SECONDS,
                TimeUnit::Hour => 1.0,
                TimeUnit::None => 1.0,
            },
            TimeUnit::None => 1.0,
        }
    }
}
