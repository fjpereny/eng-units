

#[cfg(test)]
mod tests {
    use crate::units::Unit;
    use crate::eng_units::EngUnit;

    #[test]
    fn length_change_m_to_km_val() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Kilometer);
        assert_eq!(unit.value, 0.001);
    }

    #[test]
    fn length_change_m_to_km_to_str() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Kilometer);
        assert_eq!(unit.to_string(), "0.001 km");
    }

    #[test]
    fn length_change_m_to_cm_val() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Centimeter);
        assert_eq!(unit.value, 100.0);
    }

    #[test]
    fn length_change_m_to_cm_to_str() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Centimeter);
        assert_eq!(unit.to_string(), "100 cm");
    }

    #[test]
    fn length_change_m_to_mm_val() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Millimeter);
        assert_eq!(unit.value, 1000.0);
    }

    #[test]
    fn length_change_m_to_mm_to_str() {
        let mut unit = EngUnit::from_unit(Unit::Meter, 1.0);
        unit.change_unit(Unit::Millimeter);
        assert_eq!(unit.to_string(), "1000 mm");
    }

}