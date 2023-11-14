mod units;

#[cfg(test)]
mod tests {
    use crate::units::amount_of_substance::AmountOfSubstanceUnit;
    use crate::units::electric_current::ElectricCurrentUnit;
    use crate::units::length::LengthUnit;
    use crate::units::luminous_intensity::LuminousIntensityUnit;
    use crate::units::mass::MassUnit;
    use crate::units::temperature::TemperatureDeltaUnit;
    use crate::units::time::TimeUnit;
    use crate::units::EngUnit;
    use crate::{mass, temperature, time};

    #[test]
    fn test_1() {
        let temp_1 = temperature!(1.0, TemperatureDeltaUnit::C);
        let mass_1 = mass!(1.0, MassUnit::Kilogram);
        let t_1 = time!(1.0, TimeUnit::Second);

        let unit = temp_1 * mass_1 * t_1;
        assert_eq!(1.0, unit.value);
        assert_eq!("1 kg·°C·s", unit.to_string());
    }

    #[test]
    fn test_2() {
        let temp_1 = temperature!(1.0, TemperatureDeltaUnit::K);
        let mass_1 = mass!(1.0, MassUnit::Pound);
        let t_1 = time!(1.0, TimeUnit::Minute);
        let unit = temp_1 * mass_1 / t_1 * 123.45;
        assert_eq!(123.45, unit.value);
        assert_eq!("123.45 lb·K/min", unit.to_string());
    }
}
