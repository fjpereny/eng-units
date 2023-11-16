use crate::units::AmountOfSubstanceUnit;
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TemperatureDeltaUnit;
use crate::units::TimeUnit;

#[derive(Clone, Debug, PartialEq)]
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
