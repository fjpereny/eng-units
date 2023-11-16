use crate::units::AmountOfSubstanceUnit;
use crate::units::ElectricCurrentUnit;
use crate::units::IsEngUnitType;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Clone, Debug, PartialEq)]
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
