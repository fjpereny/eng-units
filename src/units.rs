// eng-units - engineering unit conversion and calculation library
// Copyright (C) 2023 Frank Pereny

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod amount_of_substance_unit;
pub mod electric_current_unit;
pub mod energy_unit;
pub mod length_unit;
pub mod luminous_intensity_unit;
pub mod mass_unit;
pub mod power_unit;
pub mod temperature_unit;
pub mod time_unit;

use crate::complex_units::ComplexUnit;
use crate::units::amount_of_substance_unit::AmountOfSubstanceUnit;
use crate::units::electric_current_unit::ElectricCurrentUnit;
use crate::units::length_unit::LengthUnit;
use crate::units::luminous_intensity_unit::LuminousIntensityUnit;
use crate::units::mass_unit::MassUnit;
use crate::units::temperature_unit::TemperatureDeltaUnit;
use crate::units::time_unit::TimeUnit;

use std::cmp::Ordering;
use std::fmt::Display;
use std::ops;

#[derive(Clone, Debug)]
pub struct EngUnit {
    pub value: f64,
    pub amount_of_substance_count: i32,
    pub amount_of_substance_unit: AmountOfSubstanceUnit,
    pub electric_current_count: i32,
    pub electric_current_unit: ElectricCurrentUnit,
    pub length_count: i32,
    pub length_unit: LengthUnit,
    pub luminous_intensity_count: i32,
    pub luminous_intensity_unit: LuminousIntensityUnit,
    pub mass_count: i32,
    pub mass_unit: MassUnit,
    pub temperature_count: i32,
    pub temperature_unit: TemperatureDeltaUnit,
    pub time_count: i32,
    pub time_unit: TimeUnit,
    pub unit_numerator: Vec<ComplexUnit>,
    pub unit_denominator: Vec<ComplexUnit>,
}

impl Default for EngUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EngUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_units() || self.has_custom_untits() {
            write!(f, "{:.2} {}", self.value, self.unit_to_string())
        } else {
            write!(f, "{:.2}", self.value)
        }
    }
}

pub trait Convert<T> {
    fn conversion_factor(from_unit: &T, to_unit: &T) -> f64;
}

pub trait IsEngUnitType {
    fn is_amount_unit() -> bool {
        false
    }
    fn is_electric_current_unit() -> bool {
        false
    }
    fn is_length_unit() -> bool {
        false
    }
    fn is_luminous_unit() -> bool {
        false
    }
    fn is_mass_unit() -> bool {
        false
    }
    fn is_temperature_unit() -> bool {
        false
    }
    fn is_time_unit() -> bool {
        false
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
            unit_numerator: Vec::new(),
            unit_denominator: Vec::new(),
        }
    }

    fn has_custom_untits(&self) -> bool {
        self.unit_numerator.len() > 0 || self.unit_denominator.len() > 0
    }

    fn to_amount_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> AmountOfSubstanceUnit {
        unit.into()
    }
    fn to_electric_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> ElectricCurrentUnit {
        unit.into()
    }
    fn to_length_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> LengthUnit {
        unit.into()
    }
    fn to_luminous_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> LuminousIntensityUnit {
        unit.into()
    }
    fn to_mass_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> MassUnit {
        unit.into()
    }
    fn to_temperature_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> TemperatureDeltaUnit {
        unit.into()
    }

    fn to_time_unit<
        T: Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        unit: T,
    ) -> TimeUnit {
        unit.into()
    }

    pub fn convert<
        T: IsEngUnitType
            + Into<AmountOfSubstanceUnit>
            + Into<ElectricCurrentUnit>
            + Into<LengthUnit>
            + Into<LuminousIntensityUnit>
            + Into<MassUnit>
            + Into<TemperatureDeltaUnit>
            + Into<TimeUnit>,
    >(
        &self,
        to_unit: T,
    ) -> Self {
        let mut new_unit = self.clone();
        if T::is_amount_unit() {
            let from_unit = &self.amount_of_substance_unit;
            let to_unit = EngUnit::to_amount_unit(to_unit);
            let mut conversion_factor =
                AmountOfSubstanceUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.amount_of_substance_count as f64);
            match self.amount_of_substance_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.amount_of_substance_unit = to_unit;
        } else if T::is_electric_current_unit() {
            let from_unit = &self.electric_current_unit;
            let to_unit = EngUnit::to_electric_unit(to_unit);
            let mut conversion_factor = ElectricCurrentUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.electric_current_count as f64);
            match self.electric_current_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.electric_current_unit = to_unit;
        } else if T::is_length_unit() {
            let from_unit = &self.length_unit;
            let to_unit = EngUnit::to_length_unit(to_unit);
            let mut conversion_factor = LengthUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.length_count as f64);
            match self.length_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.length_unit = to_unit;
        } else if T::is_luminous_unit() {
            let from_unit = &self.luminous_intensity_unit;
            let to_unit = EngUnit::to_luminous_unit(to_unit);
            let mut conversion_factor =
                LuminousIntensityUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.luminous_intensity_count as f64);
            match self.luminous_intensity_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.luminous_intensity_unit = to_unit;
        } else if T::is_mass_unit() {
            let from_unit = &self.mass_unit;
            let to_unit = EngUnit::to_mass_unit(to_unit);
            let mut conversion_factor = MassUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.mass_count as f64);
            match self.mass_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.mass_unit = to_unit;
        } else if T::is_temperature_unit() {
            let from_unit = &self.temperature_unit;
            let to_unit = EngUnit::to_temperature_unit(to_unit);
            let mut conversion_factor =
                TemperatureDeltaUnit::conversion_factor(from_unit, &to_unit);
            conversion_factor = f64::powf(conversion_factor, self.temperature_count as f64);
            match self.temperature_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.temperature_unit = to_unit;
        } else if T::is_time_unit() {
            let from_unit = &self.time_unit;
            let to_unit = EngUnit::to_time_unit(to_unit);
            let conversion_factor = TimeUnit::conversion_factor(from_unit, &to_unit);
            new_unit.value *= f64::powf(conversion_factor, self.time_count as f64);
            match self.time_count.cmp(&0) {
                Ordering::Greater => {
                    new_unit.value *= conversion_factor;
                }
                Ordering::Less => {
                    new_unit.value /= conversion_factor;
                }
                Ordering::Equal => {}
            }
            new_unit.time_unit = to_unit;
        }
        new_unit
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
        false
    }

    pub fn unit_to_string(&self) -> String {
        let mut s_numerator: Vec<String> = Vec::new();
        let mut s_denominator: Vec<String> = Vec::new();

        for u in &self.unit_numerator {
            s_numerator.push(u.unit_to_string())
        }

        if self.amount_of_substance_count >= 2 {
            let s = format!(
                "{}^{}",
                self.amount_of_substance_unit.to_string(),
                self.amount_of_substance_count
            );
            s_numerator.push(s);
        } else if self.amount_of_substance_count == 1 {
            let s = self.amount_of_substance_unit.to_string();
            s_numerator.push(s.to_string());
        }
        if self.electric_current_count >= 2 {
            let s = format!(
                "{}^{}",
                self.electric_current_unit.to_string(),
                self.electric_current_count
            );
            s_numerator.push(s);
        } else if self.electric_current_count == 1 {
            let s = self.electric_current_unit.to_string();
            s_numerator.push(s.to_string());
        }
        if self.mass_count >= 2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_numerator.push(s);
        } else if self.mass_count == 1 {
            let s = self.mass_unit.to_string();
            s_numerator.push(s.to_string());
        }
        if self.length_count >= 2 {
            let s = format!("{}^{}", self.length_unit.to_string(), self.length_count);
            s_numerator.push(s);
        } else if self.length_count == 1 {
            let s = self.length_unit.to_string();
            s_numerator.push(s.to_string());
        }
        if self.luminous_intensity_count >= 2 {
            let s = format!(
                "{}^{}",
                self.luminous_intensity_unit.to_string(),
                self.luminous_intensity_count
            );
            s_numerator.push(s);
        } else if self.luminous_intensity_count == 1 {
            let s = self.luminous_intensity_unit.to_string();
            s_numerator.push(s.to_string());
        }
        if self.time_count >= 2 {
            let s = format!("{}^{}", self.time_unit.to_string(), self.time_count);
            s_numerator.push(s);
        } else if self.time_count == 1 {
            let s = self.time_unit.to_string();
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

        // String Denominator
        for u in &self.unit_denominator {
            s_denominator.push(u.unit_to_string())
        }
        if self.amount_of_substance_count <= -2 {
            let s = format!(
                "{}^{}",
                self.amount_of_substance_unit.to_string(),
                self.amount_of_substance_count
            );
            s_denominator.push(s);
        } else if self.amount_of_substance_count == -1 {
            let s = self.amount_of_substance_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.electric_current_count <= -2 {
            let s = format!(
                "{}^{}",
                self.electric_current_unit.to_string(),
                self.electric_current_count
            );
            s_denominator.push(s);
        } else if self.electric_current_count == -1 {
            let s = self.electric_current_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.length_count <= -2 {
            let s = format!("{}^{}", self.length_unit.to_string(), self.length_count);
            s_denominator.push(s);
        } else if self.length_count == -1 {
            let s = self.length_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.luminous_intensity_count <= -2 {
            let s = format!(
                "{}^{}",
                self.luminous_intensity_unit.to_string(),
                self.luminous_intensity_count
            );
            s_denominator.push(s);
        } else if self.luminous_intensity_count == -1 {
            let s = self.luminous_intensity_unit.to_string();
            s_denominator.push(s.to_string());
        }

        if self.mass_count <= -2 {
            let s = format!("{}^{}", self.mass_unit.to_string(), self.mass_count);
            s_denominator.push(s);
        } else if self.mass_count == -1 {
            let s = self.mass_unit.to_string();
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

        let mut s_output = format!("${}\\ ", self.value);
        for s in s_numerator.iter() {
            s_output.push_str(s);
        }

        if !s_denominator.is_empty() {
            s_output.push('/');
        }
        for s in s_denominator.iter() {
            s_output.push_str(s);
        }
        s_output.push('$');
        s_output
    }

    fn multiply_units(self, other: &EngUnit) -> EngUnit {
        let mut new_unit = EngUnit::new();
        new_unit.amount_of_substance_count =
            self.amount_of_substance_count + other.amount_of_substance_count;
        new_unit.electric_current_count =
            self.electric_current_count + other.electric_current_count;
        new_unit.length_count = self.length_count + other.length_count;
        new_unit.luminous_intensity_count =
            self.luminous_intensity_count + other.luminous_intensity_count;
        new_unit.mass_count = self.mass_count + other.mass_count;
        new_unit.temperature_count = self.temperature_count + other.temperature_count;
        new_unit.time_count = self.time_count + other.time_count;

        let mut amount_conversion_factor = AmountOfSubstanceUnit::conversion_factor(
            &other.amount_of_substance_unit,
            &self.amount_of_substance_unit,
        );
        if other.amount_of_substance_count < 0 {
            amount_conversion_factor = 1.0 / amount_conversion_factor;
        }

        let mut electric_conversion_factor = ElectricCurrentUnit::conversion_factor(
            &other.electric_current_unit,
            &self.electric_current_unit,
        );
        if other.electric_current_count < 0 {
            electric_conversion_factor = 1.0 / electric_conversion_factor;
        }

        let mut length_conversion_factor =
            LengthUnit::conversion_factor(&other.length_unit, &self.length_unit);
        if other.length_count < 0 {
            length_conversion_factor = 1.0 / length_conversion_factor;
        }

        let mut luminous_conversion_factor = LuminousIntensityUnit::conversion_factor(
            &other.luminous_intensity_unit,
            &self.luminous_intensity_unit,
        );
        if other.luminous_intensity_count < 0 {
            luminous_conversion_factor = 1.0 / luminous_conversion_factor;
        }

        let mut mass_conversion_factor =
            MassUnit::conversion_factor(&other.mass_unit, &self.mass_unit);
        if other.mass_count < 0 {
            mass_conversion_factor = 1.0 / mass_conversion_factor;
        }

        let mut temperature_conversion_factor = TemperatureDeltaUnit::conversion_factor(
            &other.temperature_unit,
            &self.temperature_unit,
        );
        if other.temperature_count < 0 {
            temperature_conversion_factor = 1.0 / temperature_conversion_factor;
        }

        let mut time_conversion_factor =
            TimeUnit::conversion_factor(&other.time_unit, &self.time_unit);
        if other.time_count < 0 {
            time_conversion_factor = 1.0 / time_conversion_factor;
        }

        new_unit.value = self.value * other.value;
        new_unit.value *= amount_conversion_factor;
        new_unit.value *= electric_conversion_factor;
        new_unit.value *= length_conversion_factor;
        new_unit.value *= luminous_conversion_factor;
        new_unit.value *= mass_conversion_factor;
        new_unit.value *= temperature_conversion_factor;
        new_unit.value *= time_conversion_factor;

        if new_unit.amount_of_substance_count != 0 {
            if self.amount_of_substance_count != 0 {
                new_unit.amount_of_substance_unit = self.amount_of_substance_unit;
            } else {
                new_unit.amount_of_substance_unit = other.amount_of_substance_unit.clone();
            }
        } else {
            new_unit.amount_of_substance_unit = AmountOfSubstanceUnit::None;
        }

        if new_unit.electric_current_count != 0 {
            if self.electric_current_count != 0 {
                new_unit.electric_current_unit = self.electric_current_unit;
            } else {
                new_unit.electric_current_unit = other.electric_current_unit.clone();
            }
        } else {
            new_unit.electric_current_unit = ElectricCurrentUnit::None;
        }

        if new_unit.length_count != 0 {
            if self.length_count != 0 {
                new_unit.length_unit = self.length_unit;
            } else {
                new_unit.length_unit = other.length_unit.clone();
            }
        } else {
            new_unit.length_unit = LengthUnit::None;
        }

        if new_unit.luminous_intensity_count != 0 {
            if self.luminous_intensity_count != 0 {
                new_unit.luminous_intensity_unit = self.luminous_intensity_unit;
            } else {
                new_unit.luminous_intensity_unit = other.luminous_intensity_unit.clone();
            }
        } else {
            new_unit.luminous_intensity_unit = LuminousIntensityUnit::None;
        }

        if new_unit.mass_count != 0 {
            if self.mass_count != 0 {
                new_unit.mass_unit = self.mass_unit;
            } else {
                new_unit.mass_unit = other.mass_unit.clone();
            }
        } else {
            new_unit.mass_unit = MassUnit::None;
        }

        if new_unit.temperature_count != 0 {
            if self.temperature_count != 0 {
                new_unit.temperature_unit = self.temperature_unit;
            } else {
                new_unit.temperature_unit = other.temperature_unit.clone();
            }
        } else {
            new_unit.temperature_unit = TemperatureDeltaUnit::None;
        }

        if new_unit.time_count != 0 {
            if self.time_count != 0 {
                new_unit.time_unit = self.time_unit;
            } else {
                new_unit.time_unit = other.time_unit.clone();
            }
        } else {
            new_unit.time_unit = TimeUnit::None;
        }
        new_unit
    }

    fn divide_units(self, other: &EngUnit) -> EngUnit {
        let recip = other.reciprocal();
        self.multiply_units(&recip)
    }

    pub fn reciprocal(&self) -> EngUnit {
        let mut recip = self.clone();
        recip.value = 1.0 / recip.value;
        recip.amount_of_substance_count *= -1;
        recip.electric_current_count *= -1;
        recip.length_count *= -1;
        recip.luminous_intensity_count *= -1;
        recip.mass_count *= -1;
        recip.temperature_count *= -1;
        recip.time_count *= -1;
        recip
    }

    pub fn get_complex_unit() {}
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

pub fn same_units(unit_1: &EngUnit, unit_2: &EngUnit) -> bool {
    if unit_1.amount_of_substance_unit != unit_2.amount_of_substance_unit {
        return false;
    }
    if unit_1.electric_current_unit != unit_2.electric_current_unit {
        return false;
    }
    if unit_1.length_unit != unit_2.length_unit {
        return false;
    }
    if unit_1.luminous_intensity_unit != unit_2.luminous_intensity_unit {
        return false;
    }
    if unit_1.mass_unit != unit_2.mass_unit {
        return false;
    }
    if unit_1.temperature_unit != unit_2.temperature_unit {
        return false;
    }
    if unit_1.time_unit != unit_2.time_unit {
        return false;
    }

    if unit_1.amount_of_substance_count != unit_2.amount_of_substance_count {
        return false;
    }
    if unit_1.electric_current_count != unit_2.electric_current_count {
        return false;
    }
    if unit_1.length_count != unit_2.length_count {
        return false;
    }
    if unit_1.luminous_intensity_count != unit_2.luminous_intensity_count {
        return false;
    }
    if unit_1.mass_count != unit_2.mass_count {
        return false;
    }
    if unit_1.temperature_count != unit_2.temperature_count {
        return false;
    }
    if unit_1.time_count != unit_2.time_count {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::temperature;
    use crate::*;

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
        let unit_1 = temperature!(1.0, TemperatureDeltaUnit::C);
        let unit_2 = temperature!(2.0, TemperatureDeltaUnit::C);
        let unit_3 = unit_1 / unit_2;
        assert_eq!(0.5, unit_3.value);
        assert_eq!("0.50", unit_3.to_string());
        assert_eq!(0, unit_3.temperature_count);
        assert_eq!(TemperatureDeltaUnit::None, unit_3.temperature_unit);
        assert!(!unit_3.has_units())
    }

    #[test]
    fn temperature_k_div_r() {
        let unit_1 = temperature!(1.0, TemperatureDeltaUnit::K);
        let unit_2 = temperature!(1.0, TemperatureDeltaUnit::R);
        let unit_3 = unit_1 / unit_2;
        let expected = 9.0 / 5.0;
        assert!(f64::abs(expected - unit_3.value) < 0.000001);
        assert_eq!("1.80", unit_3.to_string());
        assert_eq!(TemperatureDeltaUnit::None, unit_3.temperature_unit);
        assert!(!unit_3.has_units())
    }
}
