use crate::units::IsEngUnitType;

#[macro_export]
macro_rules! mass {
    ($value:literal, $unit:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.mass_count = 1;
        if $unit == MassUnit::None {
            unit.mass_count = 0;
        }
        unit.mass_unit = $unit;
        unit
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub enum MassUnit {
    Kilogram,
    Pound,
    None,
}

pub const KILOGRAM_TO_POUND: f64 = 2.204_622_62;

impl MassUnit {
    pub fn to_string(&self) -> &'static str {
        match self {
            MassUnit::Kilogram => "kg",
            MassUnit::Pound => "lb",
            MassUnit::None => "",
        }
    }

    pub fn get_unit<T: Into<MassUnit>>(unit: T) -> MassUnit {
        unit.into()
    }

    pub fn conversion_factor(from: &MassUnit, to: &MassUnit) -> f64 {
        match from {
            MassUnit::Kilogram => match to {
                MassUnit::Kilogram => 1.0,
                MassUnit::Pound => KILOGRAM_TO_POUND,
                MassUnit::None => 1.0,
            },
            MassUnit::Pound => match to {
                MassUnit::Pound => 1.0,
                MassUnit::Kilogram => 1.0 / KILOGRAM_TO_POUND,
                MassUnit::None => 1.0,
            },
            MassUnit::None => 1.0,
        }
    }
}

impl IsEngUnitType for MassUnit {
    fn is_mass_unit() -> bool {
        true
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
    fn test_new() {
        let unit = mass!(123.45, MassUnit::Kilogram);
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 kg", unit.to_string());
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(AmountOfSubstanceUnit::None, unit.amount_of_substance_unit);
        assert_eq!(ElectricCurrentUnit::None, unit.electric_current_unit);
        assert_eq!(LengthUnit::None, unit.length_unit);
        assert_eq!(LuminousIntensityUnit::None, unit.luminous_intensity_unit);
        assert_eq!(MassUnit::Kilogram, unit.mass_unit);
        assert_eq!(TemperatureDeltaUnit::None, unit.temperature_unit);
        assert_eq!(TimeUnit::None, unit.time_unit);
        assert_eq!(0, unit.temperature_count);
        assert_eq!(0, unit.time_count);
        assert_eq!(0, unit.length_count);
        assert_eq!(1, unit.mass_count);
        assert_eq!(0, unit.luminous_intensity_count);
        assert_eq!(0, unit.amount_of_substance_count);
        assert_eq!(0, unit.electric_current_count);
    }

    #[test]
    fn test_value() {
        let m1 = mass!(123.45, MassUnit::Kilogram);
        assert_eq!(123.45, m1.value);
    }

    #[test]
    fn test_kg_to_str() {
        let m1 = mass!(1.0, MassUnit::Kilogram);
        assert_eq!("1.00 kg", m1.to_string());
    }

    #[test]
    fn test_lb_to_str() {
        let m1 = mass!(1.0, MassUnit::Pound);
        assert_eq!("1.00 lb", m1.to_string());
    }

    #[test]
    fn test_none_to_str() {
        let m1 = mass!(1.0, MassUnit::None);
        assert_eq!("1.00", m1.to_string());
    }

    #[test]
    fn test_latex() {
        let m1 = mass!(1.0, MassUnit::Kilogram);
        assert_eq!("$1\\ kg$", m1.to_latex())
    }

    #[test]
    fn test_unit_to_str_kg() {
        let m1 = mass!(1.0, MassUnit::Kilogram);
        assert_eq!("kg", m1.unit_to_string());
    }

    #[test]
    fn test_unit_to_str_lb() {
        let m1 = mass!(1.0, MassUnit::Pound);
        assert_eq!("lb", m1.unit_to_string());
    }

    #[test]
    fn test_unit_to_str_none() {
        let m1 = mass!(1.0, MassUnit::None);
        assert_eq!("", m1.unit_to_string());
        assert_eq!("", MassUnit::to_string(&MassUnit::None));
    }

    #[test]
    fn test_conversion_kg_lb() {
        let m1 = mass!(1.0, MassUnit::Kilogram);
        let m2 = m1.convert(MassUnit::Pound);
        assert_eq!(1.0, m1.value);
        assert_eq!(2.204_622_62, m2.value);
        assert_eq!(MassUnit::Kilogram, m1.mass_unit);
        assert_eq!(MassUnit::Pound, m2.mass_unit);
        assert_eq!("1.00 kg", m1.to_string());
        assert_eq!("2.20 lb", m2.to_string());
    }

    #[test]
    fn test_conversion_lb_kg() {
        let m1 = mass!(1.0, MassUnit::Pound);
        let m2 = m1.convert(MassUnit::Kilogram);
        assert_eq!(1.0, m1.value);
        let expected = 0.45359237;
        assert!(f64::abs(expected - m2.value) < 0.00001);
        assert_eq!(MassUnit::Pound, m1.mass_unit);
        assert_eq!(MassUnit::Kilogram, m2.mass_unit);
        assert_eq!("1.00 lb", m1.to_string());
        assert_eq!("0.45 kg", m2.to_string());
    }
}
