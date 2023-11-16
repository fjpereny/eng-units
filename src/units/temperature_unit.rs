use crate::units::AmountOfSubstanceUnit;
use crate::units::ElectricCurrentUnit;
use crate::units::IsEngUnitType;
use crate::units::LengthUnit;
use crate::units::LuminousIntensityUnit;
use crate::units::MassUnit;
use crate::units::TimeUnit;

#[macro_export]
macro_rules! temperature {
    ($value:literal, $unit:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.temperature_count = 1;
        if $unit == TemperatureDeltaUnit::None {
            unit.temperature_count = 0;
        }
        unit.temperature_unit = $unit;
        unit
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub enum TemperatureDeltaUnit {
    C,
    R,
    F,
    K,
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
    > From<&T> for TemperatureDeltaUnit
{
    fn from(value: &T) -> Self {
        if T::is_amount_unit() {
            value.into()
        } else {
            Self::None
        }
    }
}
impl IsEngUnitType for TemperatureDeltaUnit {
    fn is_temperature_unit() -> bool {
        true
    }
}
impl From<AmountOfSubstanceUnit> for TemperatureDeltaUnit {
    fn from(_: AmountOfSubstanceUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}
impl From<ElectricCurrentUnit> for TemperatureDeltaUnit {
    fn from(_: ElectricCurrentUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}
impl From<LengthUnit> for TemperatureDeltaUnit {
    fn from(_: LengthUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}
impl From<LuminousIntensityUnit> for TemperatureDeltaUnit {
    fn from(_: LuminousIntensityUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}
impl From<MassUnit> for TemperatureDeltaUnit {
    fn from(_: MassUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}
impl From<TimeUnit> for TemperatureDeltaUnit {
    fn from(_: TimeUnit) -> Self {
        TemperatureDeltaUnit::None
    }
}

impl TemperatureDeltaUnit {
    pub fn to_string(&self) -> &'static str {
        match self {
            TemperatureDeltaUnit::R => "R",
            TemperatureDeltaUnit::K => "K",
            TemperatureDeltaUnit::C => "°C",
            TemperatureDeltaUnit::F => "°F",
            TemperatureDeltaUnit::None => "",
        }
    }

    pub fn to_latex(&self) -> &'static str {
        match self {
            TemperatureDeltaUnit::R => "R",
            TemperatureDeltaUnit::K => "K",
            TemperatureDeltaUnit::C => "^\\circ C",
            TemperatureDeltaUnit::F => "^\\circ F",
            TemperatureDeltaUnit::None => "",
        }
    }

    pub fn conversion_factor(from: &TemperatureDeltaUnit, to: &TemperatureDeltaUnit) -> f64 {
        match from {
            TemperatureDeltaUnit::R => match to {
                TemperatureDeltaUnit::R => 1.0,
                TemperatureDeltaUnit::K => 5.0 / 9.0,
                TemperatureDeltaUnit::F => 1.0,
                TemperatureDeltaUnit::C => 5.0 / 9.0,
                TemperatureDeltaUnit::None => 1.0,
            },
            TemperatureDeltaUnit::F => match to {
                TemperatureDeltaUnit::R => 1.0,
                TemperatureDeltaUnit::K => 5.0 / 9.0,
                TemperatureDeltaUnit::F => 1.0,
                TemperatureDeltaUnit::C => 5.0 / 9.0,
                TemperatureDeltaUnit::None => 1.0,
            },
            TemperatureDeltaUnit::K => match to {
                TemperatureDeltaUnit::R => 9.0 / 5.0,
                TemperatureDeltaUnit::K => 1.0,
                TemperatureDeltaUnit::F => 9.0 / 5.0,
                TemperatureDeltaUnit::C => 1.0,
                TemperatureDeltaUnit::None => 1.0,
            },
            TemperatureDeltaUnit::C => match to {
                TemperatureDeltaUnit::R => 9.0 / 5.0,
                TemperatureDeltaUnit::K => 1.0,
                TemperatureDeltaUnit::F => 9.0 / 5.0,
                TemperatureDeltaUnit::C => 1.0,
                TemperatureDeltaUnit::None => 1.0,
            },
            TemperatureDeltaUnit::None => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
    use crate::units::electric_current_unit::ElectricCurrentUnit;
    use crate::units::length_unit::LengthUnit;
    use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
    use crate::units::mass_unit::MassUnit;
    use crate::units::temperature_unit::TemperatureDeltaUnit;
    use crate::units::time_unit::TimeUnit;
    use crate::units::EngUnit;

    #[test]
    fn conversion_r_to_r() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::R,
            &TemperatureDeltaUnit::R,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_f_to_f() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::F,
            &TemperatureDeltaUnit::F,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_k_to_k() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::K,
            &TemperatureDeltaUnit::K,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_k_to_f() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::K,
            &TemperatureDeltaUnit::F,
        );
        assert_eq!(9.0 / 5.0, val);
    }

    #[test]
    fn conversion_r_to_c() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::R,
            &TemperatureDeltaUnit::C,
        );
        assert_eq!(5.0 / 9.0, val);
    }

    #[test]
    fn conversion_r_to_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::R,
            &TemperatureDeltaUnit::None,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_f_to_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::F,
            &TemperatureDeltaUnit::None,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_c_to_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::C,
            &TemperatureDeltaUnit::None,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_c_to_f() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::C,
            &TemperatureDeltaUnit::F,
        );
        assert_eq!(9.0 / 5.0, val);
    }

    #[test]
    fn conversion_f_to_c() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::F,
            &TemperatureDeltaUnit::C,
        );
        assert_eq!(5.0 / 9.0, val);
    }

    #[test]
    fn conversion_c_to_k() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::C,
            &TemperatureDeltaUnit::K,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_k_to_c() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::K,
            &TemperatureDeltaUnit::C,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_k_to_r() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::K,
            &TemperatureDeltaUnit::R,
        );
        assert_eq!(9.0 / 5.0, val);
    }

    #[test]
    fn conversion_r_to_k() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::R,
            &TemperatureDeltaUnit::K,
        );
        assert_eq!(5.0 / 9.0, val);
    }

    #[test]
    fn conversion_c_to_r() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::C,
            &TemperatureDeltaUnit::R,
        );
        assert_eq!(9.0 / 5.0, val);
    }

    #[test]
    fn conversion_f_to_k() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::F,
            &TemperatureDeltaUnit::K,
        );
        assert_eq!(5.0 / 9.0, val);
    }

    #[test]
    fn conversion_f_to_r() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::F,
            &TemperatureDeltaUnit::R,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn conversion_r_to_f() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::R,
            &TemperatureDeltaUnit::F,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn from_is_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::None,
            &TemperatureDeltaUnit::K,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn to_is_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::K,
            &TemperatureDeltaUnit::None,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn both_are_none() {
        let val = TemperatureDeltaUnit::conversion_factor(
            &TemperatureDeltaUnit::None,
            &TemperatureDeltaUnit::None,
        );
        assert_eq!(1.0, val);
    }

    #[test]
    fn macro_create_temperature_1() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::C);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 °C", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::C, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(1, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(0, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn macro_create_temperature_2() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::K);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 K", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::K, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(1, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(0, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn macro_create_temperature_3() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::F);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 °F", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::F, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(1, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(0, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn macro_create_temperature_4() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::R);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 R", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::R, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(1, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(0, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn macro_create_temperature_5() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::None);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::None, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(0, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(0, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn test_unit_strings() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::C);
        assert_eq!("°C", unit.unit_to_string());
        let unit = temperature!(123.45, TemperatureDeltaUnit::K);
        assert_eq!("K", unit.unit_to_string());
        let unit = temperature!(123.45, TemperatureDeltaUnit::R);
        assert_eq!("R", unit.unit_to_string());
        let unit = temperature!(123.45, TemperatureDeltaUnit::F);
        assert_eq!("°F", unit.unit_to_string());
        let unit = temperature!(123.45, TemperatureDeltaUnit::None);
        assert_eq!("", unit.unit_to_string());
    }

    #[test]
    fn test_unit_strings_2() {
        assert_eq!("°C", TemperatureDeltaUnit::C.to_string());
        assert_eq!("°F", TemperatureDeltaUnit::F.to_string());
        assert_eq!("R", TemperatureDeltaUnit::R.to_string());
        assert_eq!("K", TemperatureDeltaUnit::K.to_string());
        assert_eq!("", TemperatureDeltaUnit::None.to_string());
    }

    #[test]
    fn unit_clone() {
        let unit = temperature!(123.45, TemperatureDeltaUnit::R);
        let unit_2 = unit.clone();
        assert_eq!(123.45, unit_2.value);
        assert_eq!("123.45 R", unit_2.to_string());
        assert_eq!(TimeUnit::None, unit_2.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit_2.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit_2.electric_current_unit);
        assert_eq!(LengthUnit::None, unit_2.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit_2.luminous_intensity_unit);
        assert_eq!(MassUnit::None, unit_2.mass_unit);
        assert_eq!(TemperatureDeltaUnit::R, unit_2.temperature_unit);
        assert_eq!(TimeUnit::None, unit_2.time_unit);
        assert_eq!(1, unit_2.temperature_count);
        assert_eq!(0, unit_2.time_count);
        assert_eq!(0, unit_2.length_count);
        assert_eq!(0, unit_2.mass_count);
        assert_eq!(0, unit_2.luminous_intensity_count);
        assert_eq!(0, unit_2.amount_of_substance_count);
        assert_eq!(0, unit_2.electric_current_count);
    }

    #[test]
    fn test_equality() {
        let a = temperature!(123.45, TemperatureDeltaUnit::K);
        let b = temperature!(123.45, TemperatureDeltaUnit::K);
        assert_eq!(a.temperature_unit, b.temperature_unit);

        let a = temperature!(123.45, TemperatureDeltaUnit::C);
        let b = temperature!(123.45, TemperatureDeltaUnit::C);
        assert_eq!(a.temperature_unit, b.temperature_unit);

        let a = temperature!(123.45, TemperatureDeltaUnit::R);
        let b = temperature!(123.45, TemperatureDeltaUnit::R);
        assert_eq!(a.temperature_unit, b.temperature_unit);

        let a = temperature!(123.45, TemperatureDeltaUnit::F);
        let b = temperature!(123.45, TemperatureDeltaUnit::F);
        assert_eq!(a.temperature_unit, b.temperature_unit);
    }

    #[test]
    fn test_debug() {
        let a = temperature!(123.45, TemperatureDeltaUnit::K);
        let s = format!("{a:?}");
        assert!(!s.is_empty())
    }

    #[test]
    fn test_latex_1() {
        let a = temperature!(123.45, TemperatureDeltaUnit::K);
        assert_eq!("$123.45\\ K$", a.to_latex());
    }

    #[test]
    fn test_latex_2() {
        let a = temperature!(123.45, TemperatureDeltaUnit::R);
        assert_eq!("$123.45\\ R$", a.to_latex());
    }

    #[test]
    fn test_latex_3() {
        let a = temperature!(123.45, TemperatureDeltaUnit::C);
        assert_eq!("$123.45\\ ^\\circ C$", a.to_latex());
    }

    #[test]
    fn test_latex_4() {
        let a = temperature!(123.45, TemperatureDeltaUnit::F);
        assert_eq!("$123.45\\ ^\\circ F$", a.to_latex());
    }

    #[test]
    fn test_latex_5() {
        let a = temperature!(123.45, TemperatureDeltaUnit::None);
        assert_eq!("$123.45$", a.to_latex());
    }
}
