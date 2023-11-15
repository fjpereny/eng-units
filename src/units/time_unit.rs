#[macro_export]
macro_rules! time {
    ($value:literal, $unit:expr) => {{
        let mut unit = EngUnit::new();
        unit.value = $value;
        unit.time_count = 1;
        if $unit == TimeUnit::None {
            unit.time_count = 0;
        }
        unit.time_unit = $unit;
        unit
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    None,
}

pub const MINUTE_TO_SECONDS: f64 = 60.0;
pub const HOUR_TO_SECONDS: f64 = 3600.0;
pub const HOUR_TO_MINUTES: f64 = 60.0;

impl TimeUnit {
    pub fn to_string(&self) -> &'static str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "hr",
            TimeUnit::None => "",
        }
    }

    pub fn conversion_factor(from: &TimeUnit, to: &TimeUnit) -> f64 {
        match from {
            TimeUnit::Second => match to {
                TimeUnit::Second => 1.0,
                TimeUnit::Minute => 1.0 / MINUTE_TO_SECONDS,
                TimeUnit::Hour => 1.0 / HOUR_TO_SECONDS,
                TimeUnit::None => 1.0,
            },
            TimeUnit::Minute => match to {
                TimeUnit::Minute => 1.0,
                TimeUnit::Second => MINUTE_TO_SECONDS,
                TimeUnit::Hour => HOUR_TO_MINUTES,
                TimeUnit::None => 1.0,
            },
            TimeUnit::Hour => match to {
                TimeUnit::Minute => HOUR_TO_MINUTES,
                TimeUnit::Second => HOUR_TO_SECONDS,
                TimeUnit::Hour => 1.0,
                TimeUnit::None => 1.0,
            },
            TimeUnit::None => 1.0,
        }
    }
}
