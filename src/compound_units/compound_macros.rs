#[macro_export]
macro_rules! Joule {
    ($value:expr) => {{
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
macro_rules! energy_J {
    ($value:literal) => {{
        Joule! {$value}
    }};
}

#[macro_export]
macro_rules! energy_kJ {
    ($value:literal) => {{
        Joule! {($value * 1000.0)}
    }};
}

#[macro_export]
macro_rules! entropy_J_per_K {
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
