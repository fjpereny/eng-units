pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod temperature;
pub mod time;

use crate::units::amount_of_substance::AmountOfSubstanceUnit;
use crate::units::electric_current::ElectricCurrentUnit;
use crate::units::length::LengthUnit;
use crate::units::luminous_intensity::LuminousIntensityUnit;
use crate::units::mass::MassUnit;
use crate::units::temperature::TemperatureDeltaUnit;
use crate::units::time::TimeUnit;

use std::fmt::Display;
use std::ops;

#[derive(Clone, Debug)]
pub struct EngUnit {
    pub value: f64,
    pub length_count: i32,
    pub length_unit: LengthUnit,
    pub time_count: i32,
    pub time_unit: TimeUnit,
    pub mass_count: i32,
    pub mass_unit: MassUnit,
    pub temperature_count: i32,
    pub temperature_unit: TemperatureDeltaUnit,
    pub electric_current_count: i32,
    pub electric_current_unit: ElectricCurrentUnit,
    pub luminous_intensity_count: i32,
    pub luminous_intensity_unit: LuminousIntensityUnit,
    pub amount_of_substance_count: i32,
    pub amount_of_substance_unit: AmountOfSubstanceUnit,
}

impl Default for EngUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EngUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_units() {
            write!(f, "{} {}", self.value, self.unit_to_string())
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl EngUnit {
    pub fn new() -> Self {
        Self {
            value: 1.0,
            length_count: 0,
            length_unit: LengthUnit::None,
            time_count: 0,
            time_unit: TimeUnit::None,
            mass_count: 0,
            mass_unit: MassUnit::None,
            temperature_count: 0,
            temperature_unit: TemperatureDeltaUnit::None,
            electric_current_count: 0,
            electric_current_unit: ElectricCurrentUnit::None,
            luminous_intensity_count: 0,
            luminous_intensity_unit: LuminousIntensityUnit::None,
            amount_of_substance_count: 0,
            amount_of_substance_unit: AmountOfSubstanceUnit::None,
        }
    }

    fn has_units(&self) -> bool {
        if self.length_count != 0 {
            return true;
        }
        if self.time_count != 0 {
            return true;
        }
        if self.mass_count != 0 {
            return true;
        }
        if self.temperature_count != 0 {
            return true;
        }
        if self.electric_current_count != 0 {
            return true;
        }
        if self.luminous_intensity_count != 0 {
            return true;
        }
        if self.amount_of_substance_count != 0 {
            return true;
        }
        return false;
    }

    pub fn unit_to_string(&self) -> String {
        let mut s_numerator: Vec<String> = Vec::new();
        let mut s_denominator: Vec<String> = Vec::new();

        // String Numerator
        if self.mass_count >= 2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_numerator.push(s);
        } else if self.mass_count == 1 {
            let s = self.mass_unit.to_string();
            s_numerator.push(s.to_string());
        }

        if self.temperature_count >= 2 {
            let s = format!(
                "{}^{}",
                self.temperature_unit.to_string(),
                self.temperature_count
            );
            s_numerator.push(s);
        } else if self.temperature_count == 1 {
            let s = self.temperature_unit.to_string();
            s_numerator.push(s.to_string());
        }

        if self.time_count >= 2 {
            let s = format!("{}^{}", self.time_unit.to_string(), self.time_count);
            s_numerator.push(s);
        } else if self.time_count == 1 {
            let s = self.time_unit.to_string();
            s_numerator.push(s.to_string());
        }

        // String Denominator
        if self.mass_count <= -2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_denominator.push(s);
        } else if self.mass_count == -1 {
            let s = self.mass_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.temperature_count <= -2 {
            let s = format!(
                "{}^{}",
                self.temperature_unit.to_string(),
                self.temperature_count
            );
            s_denominator.push(s);
        } else if self.temperature_count == -1 {
            let s = self.temperature_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.time_count <= -2 {
            let s = format!(
                "{}^{}",
                self.time_unit.to_string(),
                i32::abs(self.time_count)
            );
            s_denominator.push(s);
        } else if self.time_count == -1 {
            let s = self.time_unit.to_string();
            s_denominator.push(s.to_string());
        }

        let mut s_output = String::new();
        for s in s_numerator.iter() {
            s_output.push_str(s);
            s_output.push('·');
        }
        if !s_numerator.is_empty() {
            let mut chars = s_output.chars();
            chars.next_back();
            s_output = chars.as_str().to_string();
        }

        if !s_denominator.is_empty() {
            s_output.push('/');
        }
        for s in s_denominator.iter() {
            s_output.push_str(s);
            s_output.push('·');
        }
        if !s_denominator.is_empty() {
            let mut chars = s_output.chars();
            chars.next_back();
            s_output = chars.as_str().to_string();
        }
        s_output
    }

    pub fn to_latex(&self) -> String {
        if !self.has_units() {
            return format!("${}$", self.value);
        }

        let mut s_numerator: Vec<String> = Vec::new();
        let mut s_denominator: Vec<String> = Vec::new();

        // String Numerator
        if self.mass_count >= 2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_numerator.push(s);
        } else if self.mass_count == 1 {
            let s = self.mass_unit.to_string();
            s_numerator.push(s.to_string());
        }

        if self.temperature_count >= 2 {
            let s = format!(
                "{}^{}",
                self.temperature_unit.to_string(),
                self.temperature_count
            );
            s_numerator.push(s);
        } else if self.temperature_count == 1 {
            let s = self.temperature_unit.to_latex();
            s_numerator.push(s.to_string());
        }

        if self.time_count >= 2 {
            let s = format!("{}^{}", self.time_unit.to_string(), self.time_count);
            s_numerator.push(s);
        } else if self.time_count == 1 {
            let s = self.time_unit.to_string();
            s_numerator.push(s.to_string());
        }

        // String Denominator
        if self.mass_count <= -2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_denominator.push(s);
        } else if self.mass_count == -1 {
            let s = self.mass_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.temperature_count <= -2 {
            let s = format!(
                "{}^{}",
                self.temperature_unit.to_string(),
                self.temperature_count
            );
            s_denominator.push(s);
        } else if self.temperature_count == -1 {
            let s = self.temperature_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.time_count <= -2 {
            let s = format!(
                "{}^{}",
                self.time_unit.to_string(),
                i32::abs(self.time_count)
            );
            s_denominator.push(s);
        } else if self.time_count == -1 {
            let s = self.time_unit.to_string();
            s_denominator.push(s.to_string());
        }

        let mut s_output = String::from(format!("${}\\ ", self.value));
        for s in s_numerator.iter() {
            s_output.push_str(s);
        }

        if !s_denominator.is_empty() {
            s_output.push('/');
        }
        for s in s_denominator.iter() {
            s_output.push_str(s);
        }
        s_output.push_str("$");
        s_output
    }

    fn multiply_units(self, other: &EngUnit) -> EngUnit {
        let new_length_count = self.length_count + other.length_count;
        let new_mass_count = self.mass_count + other.mass_count;
        let new_time_count = self.time_count + other.time_count;
        let new_temperature_count = self.temperature_count + other.temperature_count;
        let new_electric_current_count = self.electric_current_count + other.electric_current_count;
        let new_luminous_intensity_count =
            self.luminous_intensity_count + other.luminous_intensity_count;
        let new_amount_of_substance_count =
            self.amount_of_substance_count + other.amount_of_substance_count;

        let temperature_conversion_factor = TemperatureDeltaUnit::conversion_factor(
            &other.temperature_unit,
            &self.temperature_unit,
        );
        let mut other_value_temp = other.value * temperature_conversion_factor;
        if other.temperature_count < 0 {
            other_value_temp = 1.0 / other_value_temp;
        }

        let mut new_unit = EngUnit::new();
        new_unit.value = self.value;
        new_unit.value *= other_value_temp;

        new_unit.length_count = new_length_count;
        new_unit.mass_count = new_mass_count;
        new_unit.time_count = new_time_count;
        new_unit.temperature_count = new_temperature_count;
        new_unit.electric_current_count = new_electric_current_count;
        new_unit.luminous_intensity_count = new_luminous_intensity_count;
        new_unit.amount_of_substance_count = new_amount_of_substance_count;

        if new_length_count != 0 {
            if self.length_count != 0 {
                new_unit.length_unit = self.length_unit;
            } else {
                new_unit.length_unit = other.length_unit.clone();
            }
        }
        if new_mass_count != 0 {
            if self.mass_count != 0 {
                new_unit.mass_unit = self.mass_unit;
            } else {
                new_unit.mass_unit = other.mass_unit.clone();
            }
        }
        if new_time_count != 0 {
            if self.time_count != 0 {
                new_unit.time_unit = self.time_unit;
            } else {
                new_unit.time_unit = other.time_unit.clone();
            }
        }
        if new_temperature_count != 0 {
            if self.temperature_count != 0 {
                new_unit.temperature_unit = self.temperature_unit;
            } else {
                new_unit.temperature_unit = other.temperature_unit.clone();
            }
        }
        if new_electric_current_count != 0 {
            if self.electric_current_count != 0 {
                new_unit.electric_current_unit = self.electric_current_unit;
            } else {
                new_unit.electric_current_unit = other.electric_current_unit.clone();
            }
        }
        if new_luminous_intensity_count != 0 {
            if self.luminous_intensity_count != 0 {
                new_unit.luminous_intensity_unit = self.luminous_intensity_unit;
            } else {
                new_unit.luminous_intensity_unit = other.luminous_intensity_unit.clone();
            }
        }
        if new_amount_of_substance_count != 0 {
            if self.amount_of_substance_count != 0 {
                new_unit.amount_of_substance_unit = self.amount_of_substance_unit;
            } else {
                new_unit.amount_of_substance_unit = other.amount_of_substance_unit.clone();
            }
        }

        new_unit
    }

    fn divide_units(self, other: &EngUnit) -> EngUnit {
        if other.value == 0.0 {
            panic!("Divide by zero error!");
        }
        let mut other_reciprocal = other.clone();
        other_reciprocal.length_count *= -1;
        other_reciprocal.mass_count *= -1;
        other_reciprocal.time_count *= -1;
        other_reciprocal.temperature_count *= -1;
        other_reciprocal.electric_current_count *= -1;
        other_reciprocal.luminous_intensity_count *= -1;
        other_reciprocal.amount_of_substance_count *= -1;
        other_reciprocal.length_count *= -1;
        self.multiply_units(&other_reciprocal)
    }
}

impl ops::Mul for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_units(&rhs)
    }
}

impl ops::Mul for &EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: Self) -> Self::Output {
        let new_unit = self.clone();
        new_unit.multiply_units(rhs)
    }
}

impl ops::Mul<&EngUnit> for EngUnit {
    type Output = EngUnit;
    fn mul(self, lhs: &Self) -> Self::Output {
        self.multiply_units(lhs)
    }
}

impl ops::Div for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: Self) -> Self::Output {
        self.divide_units(&rhs)
    }
}

impl ops::Div for &EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: Self) -> Self::Output {
        let new_unit = self.clone();
        new_unit.divide_units(rhs)
    }
}

impl ops::Mul<f64> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs;
        new_unit
    }
}

impl ops::Mul<f64> for &EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs;
        new_unit
    }
}

impl ops::Mul<EngUnit> for f64 {
    type Output = EngUnit;
    fn mul(self, lhs: EngUnit) -> Self::Output {
        let mut new_unit = lhs.clone();
        new_unit.value *= self;
        new_unit
    }
}

impl ops::Mul<&EngUnit> for f64 {
    type Output = EngUnit;
    fn mul(self, lhs: &EngUnit) -> Self::Output {
        let mut new_unit = lhs.clone();
        new_unit.value *= self;
        new_unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_eng_unt() {
        let new_unit = EngUnit::new();
        assert_eq!(0, new_unit.length_count);
        assert_eq!(0, new_unit.time_count);
        assert_eq!(0, new_unit.mass_count);
        assert_eq!(0, new_unit.temperature_count);
        assert_eq!(0, new_unit.electric_current_count);
        assert_eq!(0, new_unit.luminous_intensity_count);
        assert_eq!(0, new_unit.amount_of_substance_count);
        assert_eq!(false, new_unit.has_units());
    }

    #[test]
    fn multiply_numerator_check() {
        let unit_1 = EngUnit::new();
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = 1;
        unit_2.mass_count = 1;
        unit_2.time_count = 1;
        unit_2.temperature_count = 1;
        unit_2.electric_current_count = 1;
        unit_2.luminous_intensity_count = 1;
        unit_2.amount_of_substance_count = 1;

        let unit3 = unit_1.multiply_units(&unit_2);
        assert_eq!(1, unit3.length_count);
        assert_eq!(1, unit3.mass_count);
        assert_eq!(1, unit3.time_count);
        assert_eq!(1, unit3.temperature_count);
        assert_eq!(1, unit3.electric_current_count);
        assert_eq!(1, unit3.luminous_intensity_count);
        assert_eq!(1, unit3.amount_of_substance_count);
        assert_eq!(true, unit3.has_units());
    }

    #[test]
    fn multiply_operator_numerator_check() {
        let unit_1 = EngUnit::new();
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = 1;
        unit_2.mass_count = 1;
        unit_2.time_count = 1;
        unit_2.temperature_count = 1;
        unit_2.electric_current_count = 1;
        unit_2.luminous_intensity_count = 1;
        unit_2.amount_of_substance_count = 1;

        let unit3 = unit_1 * unit_2;
        assert_eq!(1, unit3.length_count);
        assert_eq!(1, unit3.mass_count);
        assert_eq!(1, unit3.time_count);
        assert_eq!(1, unit3.temperature_count);
        assert_eq!(1, unit3.electric_current_count);
        assert_eq!(1, unit3.luminous_intensity_count);
        assert_eq!(1, unit3.amount_of_substance_count);
        assert_eq!(true, unit3.has_units());
    }

    #[test]
    fn multiply_denominator_check() {
        let unit_1 = EngUnit::new();
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = -1;
        unit_2.mass_count = -1;
        unit_2.time_count = -1;
        unit_2.temperature_count = -1;
        unit_2.electric_current_count = -1;
        unit_2.luminous_intensity_count = -1;
        unit_2.amount_of_substance_count = -1;

        let unit3 = unit_1.multiply_units(&unit_2);
        assert_eq!(-1, unit3.length_count);
        assert_eq!(-1, unit3.mass_count);
        assert_eq!(-1, unit3.time_count);
        assert_eq!(-1, unit3.temperature_count);
        assert_eq!(-1, unit3.electric_current_count);
        assert_eq!(-1, unit3.luminous_intensity_count);
        assert_eq!(-1, unit3.amount_of_substance_count);
        assert_eq!(true, unit3.has_units());
    }

    #[test]
    fn multiply_operator_denominator_check() {
        let unit_1 = EngUnit::new();
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = -1;
        unit_2.mass_count = -1;
        unit_2.time_count = -1;
        unit_2.temperature_count = -1;
        unit_2.electric_current_count = -1;
        unit_2.luminous_intensity_count = -1;
        unit_2.amount_of_substance_count = -1;

        let unit3 = unit_1 * unit_2;
        assert_eq!(-1, unit3.length_count);
        assert_eq!(-1, unit3.mass_count);
        assert_eq!(-1, unit3.time_count);
        assert_eq!(-1, unit3.temperature_count);
        assert_eq!(-1, unit3.electric_current_count);
        assert_eq!(-1, unit3.luminous_intensity_count);
        assert_eq!(-1, unit3.amount_of_substance_count);
        assert_eq!(true, unit3.has_units());
    }

    #[test]
    fn multiply_cancel_out_check() {
        let mut unit_1 = EngUnit::new();
        unit_1.length_count = -1;
        unit_1.mass_count = -1;
        unit_1.time_count = -1;
        unit_1.temperature_count = -1;
        unit_1.electric_current_count = -1;
        unit_1.luminous_intensity_count = -1;
        unit_1.amount_of_substance_count = -1;
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = 1;
        unit_2.mass_count = 1;
        unit_2.time_count = 1;
        unit_2.temperature_count = 1;
        unit_2.electric_current_count = 1;
        unit_2.luminous_intensity_count = 1;
        unit_2.amount_of_substance_count = 1;

        let unit3 = unit_1.multiply_units(&unit_2);
        assert_eq!(0, unit3.length_count);
        assert_eq!(0, unit3.mass_count);
        assert_eq!(0, unit3.time_count);
        assert_eq!(0, unit3.temperature_count);
        assert_eq!(0, unit3.electric_current_count);
        assert_eq!(0, unit3.luminous_intensity_count);
        assert_eq!(0, unit3.amount_of_substance_count);
    }

    #[test]
    fn multiply_operator_cancel_out_check() {
        let mut unit_1 = EngUnit::new();
        unit_1.length_count = -1;
        unit_1.mass_count = -1;
        unit_1.time_count = -1;
        unit_1.temperature_count = -1;
        unit_1.electric_current_count = -1;
        unit_1.luminous_intensity_count = -1;
        unit_1.amount_of_substance_count = -1;
        let mut unit_2 = EngUnit::new();
        unit_2.length_count = 1;
        unit_2.mass_count = 1;
        unit_2.time_count = 1;
        unit_2.temperature_count = 1;
        unit_2.electric_current_count = 1;
        unit_2.luminous_intensity_count = 1;
        unit_2.amount_of_substance_count = 1;

        let unit3 = unit_1 * unit_2;
        assert_eq!(0, unit3.length_count);
        assert_eq!(0, unit3.mass_count);
        assert_eq!(0, unit3.time_count);
        assert_eq!(0, unit3.temperature_count);
        assert_eq!(0, unit3.electric_current_count);
        assert_eq!(0, unit3.luminous_intensity_count);
        assert_eq!(0, unit3.amount_of_substance_count);
    }

    #[test]
    fn temperature_unit_new() {
        let mut unit_1 = EngUnit::new();
        unit_1.value = 100.0;
        unit_1.temperature_count = 1;
        unit_1.temperature_unit = TemperatureDeltaUnit::C;
        let unit_2 = EngUnit::new();
        let unit_3 = unit_1 * unit_2;
        assert_eq!(100.0, unit_3.value);
        assert_eq!(1, unit_3.temperature_count);
        assert_eq!(TemperatureDeltaUnit::C, unit_3.temperature_unit);
        assert_eq!(true, unit_3.has_units())
    }

    #[test]
    fn temperature_c_div_c() {
        let mut unit_1 = EngUnit::new();
        unit_1.value = 100.0;
        unit_1.temperature_count = 1;
        unit_1.temperature_unit = TemperatureDeltaUnit::C;
        let mut unit_2 = EngUnit::new();
        unit_2.value = 100.0;
        unit_2.temperature_count = -1;
        unit_2.temperature_unit = TemperatureDeltaUnit::C;
        let unit_3 = unit_1 * unit_2;
        assert_eq!(1.0, unit_3.value);
        assert_eq!(0, unit_3.temperature_count);
        assert_eq!(TemperatureDeltaUnit::None, unit_3.temperature_unit);
        assert_eq!(false, unit_3.has_units())
    }

    #[test]
    fn temperature_c_div_f() {
        let mut unit_1 = EngUnit::new();
        unit_1.value = 100.0;
        unit_1.temperature_count = 1;
        unit_1.temperature_unit = TemperatureDeltaUnit::C;
        let mut unit_2 = EngUnit::new();
        unit_2.value = 100.0;
        unit_2.temperature_count = -1;
        unit_2.temperature_unit = TemperatureDeltaUnit::F;
        let expected = 100.0 / (100.0 * 5.0 / 9.0);
        let unit_3 = unit_1 * unit_2;
        let diff = f64::abs(unit_3.value - expected);
        assert!(diff < 0.00001);
        assert_eq!(0, unit_3.temperature_count);
        assert_eq!(TemperatureDeltaUnit::None, unit_3.temperature_unit);
        assert_eq!(false, unit_3.has_units())
    }
}
