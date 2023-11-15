mod units;

#[cfg(test)]
mod tests {
    use std::ops::Div;

    use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
    use crate::units::electric_current_unit::ElectricCurrentUnit;
    use crate::units::length_unit::LengthUnit;
    use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
    use crate::units::mass_unit::MassUnit;
    use crate::units::temperature_unit::TemperatureDeltaUnit;
    use crate::units::time_unit::TimeUnit;
    use crate::units::EngUnit;
    use crate::{mass, temperature, time};

    #[test]
    fn example_test_1() {
        let quarter_pounder = mass!(0.25, MassUnit::Pound);
        assert_eq!(0.25, quarter_pounder.value);
        assert_eq!("0.25 lb", quarter_pounder.to_string());
    }

    #[test]
    fn example_test_2() {
        let quarter_pounder = mass!(0.25, MassUnit::Pound);
        let royal_with_cheese = quarter_pounder.convert(MassUnit::Kilogram);
        assert_eq!("0.11 kg", royal_with_cheese.to_string());
    }

    #[test]
    fn example_test_3() {
        let temp_1 = temperature!(4.0, TemperatureDeltaUnit::C);
        let mass_1 = mass!(5.0, MassUnit::Kilogram);
        let t_1 = time!(10.0, TimeUnit::Second);

        let unit = temp_1 * mass_1 / t_1;
        assert_eq!(2.0, unit.value);
        assert_eq!("2.00 kg·°C/s", unit.to_string());

        let double = 2.0 * unit;
        assert_eq!(4.0, double.value);
        assert_eq!("4.00 kg·°C/s", double.to_string());
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
