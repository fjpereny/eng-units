use crate::units::ElectricCurrentUnit;
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Clone, Debug, PartialEq)]
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
