#[macro_export]
macro_rules! enthalpy_J {
    ($value:literal) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.mass_count = 1;
        unit.length_count = 2;
        unit.time_count = -2;
        unit.mass_unit = MassUnit::Kilogram;
        unit.length_unit = LengthUnit::Meter;
        unit.time_unit = TimeUnit::Second;
        unit
    }};
}

#[macro_export]
macro_rules! entropy_J {
    ($value:literal) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.mass_count = 1;
        unit.length_count = 2;
        unit.time_count = -2;
        unit.temperature_count = -1;
        unit.mass_unit = MassUnit::Kilogram;
        unit.length_unit = LengthUnit::Meter;
        unit.temperature_unit = TemperatureDeltaUnit::K;
        unit.time_unit = TimeUnit::Second;
        unit
    }};
}
