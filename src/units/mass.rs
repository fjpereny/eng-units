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
