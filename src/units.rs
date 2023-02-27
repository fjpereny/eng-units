//  eng-units (https://crates.io/crates/eng-units)
//  Engineering & scientific conversion for Rust.
//  Copyright (C) 2023 Frank Pereny

//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.

//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.

//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.


use crate::fundamental::Fundamental;
use crate::fundamental::get_fundamental;
use crate::conversions::*;
use crate::unit_names;


use std::ops::{
    Add, 
    Sub, 
    Mul, 
    Div, 
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};


#[derive(Debug, Clone)]
/// A structure representing the dimensionality and value of an engineering unit.
pub struct EngUnit {
    // Numerical value of the unit
    // Example: 12.7 m^2 => 12.7
    pub value: f64,

    // Label for the unit (optional)
    // Allows nice printing and organization of units
    // Example: "Distance to my home"
    pub label: String,

    // Power of each fundamental unit
    // Example: m^2 => length_count: 2;
    pub length_count: i8,
    pub mass_count: i8,
    pub time_count: i8,
    pub current_count: i8,
    pub temp_count: i8,
    pub lumin_count: i8,
    pub amount_count: i8,

    pub length_type: Unit,
    pub mass_type: Unit,
    pub time_type: Unit,
    pub current_type: Unit,
    pub temp_type: Unit,
    pub lumin_type: Unit,
    pub amount_type: Unit,
}

impl EngUnit {
    pub fn new() -> Self {
        EngUnit { 
            value: 0.0,

            label: String::from(""),

            length_count: 0,
            mass_count: 0,
            time_count: 0,
            current_count: 0,
            temp_count: 0,
            lumin_count: 0,
            amount_count: 0,

            length_type: Unit::Meter,
            mass_type: Unit::Temp,
            time_type: Unit::Second,
            current_type: Unit::Temp,
            temp_type: Unit::Temp,
            lumin_type: Unit::Temp,
            amount_type: Unit::Temp,
         }
    }

    pub fn from_unit(val: f64, unit: Unit, power: i8) -> EngUnit {
        let mut new_unit = EngUnit::new();
        new_unit.value = val;        
        let fundamental = get_fundamental(&unit);
        match fundamental {
            Fundamental::Length => {
                new_unit.length_type = unit;
                new_unit.length_count = power;
            },
            Fundamental::Mass => {
                new_unit.mass_type = unit;
                new_unit.mass_count = power;
            },
            Fundamental::Time => {
                new_unit.time_type = unit;
                new_unit.time_count = power;
            },
            Fundamental::Current => {
                new_unit.current_type = unit;
                new_unit.current_count = power;
            },
            Fundamental::Temperature => {
                new_unit.temp_type = unit;
                new_unit.temp_count = power;
            },
            Fundamental::LuminousIntensity => {
                new_unit.lumin_type = unit;
                new_unit.lumin_count = power;
            },
            Fundamental::AmountOfSubstance => {
                new_unit.amount_type = unit;
                new_unit.amount_count = power;
            },
            
        }
        new_unit
    }

    pub fn change_unit(&mut self, unit: Unit) {

        let fundamental = get_fundamental(&unit);
        let unit_power = match fundamental {
            Fundamental::Length => self.length_count,
            Fundamental::Mass => self.mass_count,
            Fundamental::Time => self.time_count,
            Fundamental::Current => self.current_count,
            Fundamental::Temperature => self.temp_count,
            Fundamental::LuminousIntensity => self.lumin_count,
            Fundamental::AmountOfSubstance => self.amount_count,
        };        
        
        for _ in 0..unit_power.abs() {
            if unit_power > 0 {
                match fundamental {
                    Fundamental::Length => self.value *= unit_to_base_val(&self.length_type),
                    Fundamental::Mass => self.value *= unit_to_base_val(&self.mass_type),
                    Fundamental::Time => self.value *= unit_to_base_val(&self.time_type),
                    Fundamental::Current => self.value *= unit_to_base_val(&self.current_type),
                    Fundamental::Temperature => self.value *= unit_to_base_val(&self.temp_type),
                    Fundamental::LuminousIntensity => self.value *= unit_to_base_val(&self.lumin_type),
                    Fundamental::AmountOfSubstance => self.value *= unit_to_base_val(&self.amount_type),
                }
            }
            else if unit_power < 0 {
                match fundamental {
                    Fundamental::Length => self.value /= unit_to_base_val(&self.length_type),
                    Fundamental::Mass => self.value /= unit_to_base_val(&self.mass_type),
                    Fundamental::Time => self.value /= unit_to_base_val(&self.time_type),
                    Fundamental::Current => self.value /= unit_to_base_val(&self.current_type),
                    Fundamental::Temperature => self.value /= unit_to_base_val(&self.temp_type),
                    Fundamental::LuminousIntensity => self.value /= unit_to_base_val(&self.lumin_type),
                    Fundamental::AmountOfSubstance => self.value /= unit_to_base_val(&self.amount_type),
                }
            }    
            
            if unit_power > 0 {
                self.value *= base_to_unit_val(&unit);
            } else {
                self.value /= base_to_unit_val(&unit);
            }
        
        }

        match fundamental {
            Fundamental::Length => {self.length_type = unit},
            Fundamental::Mass => {self.mass_type = unit},
            Fundamental::Time => {self.time_type = unit},
            Fundamental::Current => {self.current_type = unit},
            Fundamental::Temperature => {self.temp_type = unit},
            Fundamental::LuminousIntensity => {self.lumin_type = unit},
            Fundamental::AmountOfSubstance => {self.amount_type = unit},
            
        }
    }

    pub fn push_unit(&mut self, unit: Unit, power: i8) {
        let fundamental = get_fundamental(&unit);
        match fundamental {
            Fundamental::Length => {
                self.length_type = unit;
                self.length_count = power;
            },
            Fundamental::Mass => {
                self.mass_type = unit;
                self.mass_count = power;
            }
            Fundamental::Time => {
                self.time_type = unit;
                self.time_count = power;
            }
            Fundamental::Current => {
                self.current_type = unit;
                self.current_count = power;
            }
            Fundamental::Temperature => {
                self.temp_type = unit;
                self.temp_count = power;
            }
            Fundamental::LuminousIntensity => {
                self.lumin_type = unit;
                self.lumin_count = power;
            }
            Fundamental::AmountOfSubstance => {
                self.amount_type = unit;
                self.amount_count = power;
            }
        }
    }

    pub fn fundamental_counts(&self) -> [i32; 7] {
        let counts = [
            self.length_count as i32,
            self.mass_count as i32,
            self.time_count as i32,
            self.current_count as i32,
            self.temp_count as i32,
            self.lumin_count as i32,
            self.amount_count as i32,
        ];
        counts
    }

    pub fn units(&self) -> String {
        let num = self.print_numerator();
        let den = self.print_denominator();

        if num.len() > 0 && den.len() > 0 {
            return format!("({num})/({den})");
        } else if num.len() == 0 {
            return format!("1/({den})");
        } else {
            return format!("{num}");
        }       
    }

    pub fn has_name(&self) -> bool {
        self.units() != unit_names::unit_name(&self)
    }

    pub fn print_numerator(&self) -> String {
        let mut num = String::new();
        if self.length_count >= 1 {
            let s = self.length_type.to_string();
            num.push_str(&s);
        }
        if self.length_count >= 2 {
            num.push('^');
            num.push_str(&self.length_count.to_string());
        }
        if self.mass_count >= 1 {
            let s = self.mass_type.to_string();
            num.push_str(&s);
        }
        if self.mass_count >= 2 {
            num.push('^');
            num.push_str(&self.mass_count.to_string());
        }
        if self.time_count >= 1 {
            let s = self.time_type.to_string();
            num.push_str(&s);
        }
        if self.time_count >= 2 {
            num.push('^');
            num.push_str(&self.time_type.to_string());
        }
        if self.current_count >= 1 {
            let s = self.current_type.to_string();
            num.push_str(&s);
        }
        if self.current_count >= 2 {
            num.push('^');
            num.push_str(&self.current_count.to_string());
        }
        if self.temp_count >= 1 {
            let s = self.temp_type.to_string();
            num.push_str(&s);
        }
        if self.temp_count >= 2 {
            num.push('^');
            num.push_str(&self.temp_type.to_string());
        }
        if self.lumin_count >= 1 {
            let s = self.lumin_type.to_string();
            num.push_str(&s);
        }
        if self.lumin_count >= 2 {
            num.push('^');
            num.push_str(&self.lumin_count.to_string());
        }
        if self.amount_count >= 1 {
            let s = self.amount_type.to_string();
            num.push_str(&s);
        }
        if self.amount_count >= 2 {
            num.push('^');
            num.push_str(&self.amount_count.to_string());
        }
        num
    }

    pub fn print_denominator(&self) -> String {
        let mut den = String::new();
        if self.length_count <= -1 {
            let s = self.length_type.to_string();
            den.push_str(&s);
        }
        if self.length_count <= -2 {
            den.push('^');
            let power = -self.length_count;
            den.push_str(&power.to_string());
        }
        if self.mass_count <= -1 {
            let s = self.mass_type.to_string();
            den.push_str(&s);
        }
        if self.mass_count <= -2 {
            den.push('^');
            let power = -self.mass_count;
            den.push_str(&power.to_string());
        }
        if self.time_count <= -1 {
            let s = self.time_type.to_string();
            den.push_str(&s);
        }
        if self.time_count <= -2 {
            den.push('^');
            let power = -self.time_count;
            den.push_str(&power.to_string());
        }
        if self.current_count <= -1 {
            let s = self.current_type.to_string();
            den.push_str(&s);
        }
        if self.current_count <= -2 {
            den.push('^');
            let power = -self.current_count;
            den.push_str(&power.to_string());
        }
        if self.temp_count <= -1 {
            let s = self.temp_type.to_string();
            den.push_str(&s);
        }
        if self.temp_count <= -2 {
            den.push('^');
            let power = -self.temp_count;
            den.push_str(&power.to_string());
        }
        if self.lumin_count <= -1 {
            let s = self.lumin_type.to_string();
            den.push_str(&s);
        }
        if self.lumin_count <= -2 {
            den.push('^');
            let power = -self.lumin_count;
            den.push_str(&power.to_string());
        }
        if self.amount_count <= -1 {
            let s = self.amount_type.to_string();
            den.push_str(&s);
        }
        if self.amount_count <= -2 {
            den.push('^');
            let power = -self.amount_count;
            den.push_str(&power.to_string());
        }
        den
    }

    pub fn unit_name(&self) -> String {
        unit_names::unit_name(&self)
    }


}

impl std::fmt::Display for EngUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut label = String::new();
        if self.label != "" {
            label.push_str(&self.label);
            label.push_str(": ");
        }
        let val = self.value;
        let num = self.print_numerator();
        let den = self.print_denominator();

        if num.len() > 0 && den.len() > 0 {
            return write!(f, "{label}{val} ({num})/({den})");
        } else if num.len() == 0 {
            return write!(f, "{label}{val} /({den})");
        } else {
            return write!(f, "{label}{val} {num}");
        }        
    }
}

impl Add<f64> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs;
        new_unit
    }
} 
impl Add<f32> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: f32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
} 
impl Add<i64> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: i64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<i32> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: i32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<i16> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: i16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<i8> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: i8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<u64> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: u64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<u32> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: u32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<u16> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: u16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}
impl Add<u8> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: u8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value += rhs as f64;
        new_unit
    }
}      
impl Add<EngUnit> for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new_unit = self.clone();        
        let rhs_convert = convert_other(&self, &rhs);
        new_unit.value = self.value + rhs_convert.value;
        new_unit
    }
}
impl AddAssign<f64> for EngUnit {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}
impl AddAssign<f32> for EngUnit {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs as f64;
    }
}
impl AddAssign<i64> for EngUnit {
    fn add_assign(&mut self, rhs: i64) {
        self.value += rhs as f64;
    }
}
impl AddAssign<i32> for EngUnit {
    fn add_assign(&mut self, rhs: i32) {
        self.value += rhs as f64;
    }
}
impl AddAssign<i16> for EngUnit {
    fn add_assign(&mut self, rhs: i16) {
        self.value += rhs as f64;
    }
}
impl AddAssign<i8> for EngUnit {
    fn add_assign(&mut self, rhs: i8) {
        self.value += rhs as f64;
    }
}
impl AddAssign<u64> for EngUnit {
    fn add_assign(&mut self, rhs: u64) {
        self.value += rhs as f64;
    }
}
impl AddAssign<u32> for EngUnit {
    fn add_assign(&mut self, rhs: u32) {
        self.value += rhs as f64;
    }
}
impl AddAssign<u16> for EngUnit {
    fn add_assign(&mut self, rhs: u16) {
        self.value += rhs as f64;
    }
}
impl AddAssign<u8> for EngUnit {
    fn add_assign(&mut self, rhs: u8) {
        self.value += rhs as f64;
    }
}
impl AddAssign<EngUnit> for EngUnit {
    fn add_assign(&mut self, rhs: Self) {
        let rhs_convert = convert_other(self, &rhs);
        self.value += rhs_convert.value;
    }    
}


impl Sub<f64> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs;
        new_unit
    }
} 
impl Sub<f32> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: f32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
} 
impl Sub<i64> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: i64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<i32> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: i32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<i16> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: i16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<i8> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: i8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<u64> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: u64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<u32> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: u32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<u16> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: u16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}
impl Sub<u8> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: u8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value -= rhs as f64;
        new_unit
    }
}      
impl Sub<EngUnit> for EngUnit {
    type Output = EngUnit;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_unit = self.clone();        
        let rhs_convert = convert_other(&self, &rhs);
        new_unit.value = self.value - rhs_convert.value;
        new_unit
    }
}
impl SubAssign<f64> for EngUnit {
    fn sub_assign(&mut self, rhs: f64) {
        self.value -= rhs;
    }
}
impl SubAssign<f32> for EngUnit {
    fn sub_assign(&mut self, rhs: f32) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<i64> for EngUnit {
    fn sub_assign(&mut self, rhs: i64) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<i32> for EngUnit {
    fn sub_assign(&mut self, rhs: i32) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<i16> for EngUnit {
    fn sub_assign(&mut self, rhs: i16) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<i8> for EngUnit {
    fn sub_assign(&mut self, rhs: i8) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<u64> for EngUnit {
    fn sub_assign(&mut self, rhs: u64) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<u32> for EngUnit {
    fn sub_assign(&mut self, rhs: u32) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<u16> for EngUnit {
    fn sub_assign(&mut self, rhs: u16) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<u8> for EngUnit {
    fn sub_assign(&mut self, rhs: u8) {
        self.value -= rhs as f64;
    }
}
impl SubAssign<EngUnit> for EngUnit {
    fn sub_assign(&mut self, rhs: Self) {
        let rhs_convert = convert_other(self, &rhs);
        self.value -= rhs_convert.value;
    }    
}



impl Mul<f64> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs;
        new_unit
    }
} 
impl Mul<f32> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
} 
impl Mul<i64> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: i64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<i32> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: i32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<i16> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: i16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<i8> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: i8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<u64> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: u64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<u32> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: u32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<u16> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: u16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}
impl Mul<u8> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: u8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value *= rhs as f64;
        new_unit
    }
}      
impl Mul<EngUnit> for EngUnit {
    type Output = EngUnit;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_unit = self.clone();
        let other = convert_other(&self, &rhs);
        new_unit.value *= other.value;
        new_unit.length_count += other.length_count;
        new_unit.mass_count += other.mass_count;
        new_unit.current_count += other.current_count;
        new_unit.temp_count += other.temp_count;
        new_unit.time_count += other.time_count;
        new_unit.lumin_count += other.lumin_count;
        new_unit.amount_count += other.amount_count;
        new_unit        
    }
}
impl MulAssign<f64> for EngUnit {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}
impl MulAssign<f32> for EngUnit {
    fn mul_assign(&mut self, rhs: f32) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<i64> for EngUnit {
    fn mul_assign(&mut self, rhs: i64) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<i32> for EngUnit {
    fn mul_assign(&mut self, rhs: i32) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<i16> for EngUnit {
    fn mul_assign(&mut self, rhs: i16) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<i8> for EngUnit {
    fn mul_assign(&mut self, rhs: i8) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<u64> for EngUnit {
    fn mul_assign(&mut self, rhs: u64) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<u32> for EngUnit {
    fn mul_assign(&mut self, rhs: u32) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<u16> for EngUnit {
    fn mul_assign(&mut self, rhs: u16) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<u8> for EngUnit {
    fn mul_assign(&mut self, rhs: u8) {
        self.value *= rhs as f64;
    }
}
impl MulAssign<EngUnit> for EngUnit {
    fn mul_assign(&mut self, rhs: Self) {
        let other = convert_other(self, &rhs);
        self.value *= other.value;
        self.length_count += other.length_count;
        self.mass_count += other.mass_count;
        self.current_count += other.current_count;
        self.temp_count += other.temp_count;
        self.time_count += other.time_count;
        self.lumin_count += other.lumin_count;
        self.amount_count += other.amount_count;
    }    
}

impl Div<f64> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: f64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs;
        new_unit
    }
} 

impl Div<f32> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: f32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<i64> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: i64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<i32> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: i32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<i16> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: i16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<i8> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: i8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<u64> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: u64) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<u32> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: u32) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<u16> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: u16) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<u8> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: u8) -> Self::Output {
        let mut new_unit = self.clone();
        new_unit.value /= rhs as f64;
        new_unit
    }
} 

impl Div<EngUnit> for EngUnit {
    type Output = EngUnit;
    fn div(self, rhs: Self) -> Self::Output {
        let mut new_unit = self.clone();
        let other = convert_other(&self, &rhs);
        new_unit.value /= other.value;
        new_unit.length_count += other.length_count;
        new_unit.mass_count += other.mass_count;
        new_unit.current_count += other.current_count;
        new_unit.temp_count += other.temp_count;
        new_unit.time_count += other.time_count;
        new_unit.lumin_count += other.lumin_count;
        new_unit.amount_count += other.amount_count;
        new_unit        
    }
}

impl DivAssign<f64> for EngUnit {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}
impl DivAssign<f32> for EngUnit {
    fn div_assign(&mut self, rhs: f32) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<i64> for EngUnit {
    fn div_assign(&mut self, rhs: i64) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<i32> for EngUnit {
    fn div_assign(&mut self, rhs: i32) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<i16> for EngUnit {
    fn div_assign(&mut self, rhs: i16) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<i8> for EngUnit {
    fn div_assign(&mut self, rhs: i8) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<u64> for EngUnit {
    fn div_assign(&mut self, rhs: u64) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<u32> for EngUnit {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<u16> for EngUnit {
    fn div_assign(&mut self, rhs: u16) {
        self.value /= rhs as f64;
    }
}
impl DivAssign<u8> for EngUnit {
    fn div_assign(&mut self, rhs: u8) {
        self.value /= rhs as f64;
    }
}

impl DivAssign<EngUnit> for EngUnit {
    fn div_assign(&mut self, rhs: Self) {
        let other = convert_other(self, &rhs);
        self.value /= other.value;
        self.length_count += other.length_count;
        self.mass_count += other.mass_count;
        self.current_count += other.current_count;
        self.temp_count += other.temp_count;
        self.time_count += other.time_count;
        self.lumin_count += other.lumin_count;
        self.amount_count += other.amount_count;
    }    
}



fn convert_other(this: &EngUnit, other: &EngUnit) -> EngUnit {
    let mut other_converted = other.clone();
        
    if other.length_count != 0 {other_converted.change_unit(this.length_type);}
    if other.mass_count != 0 {other_converted.change_unit(this.mass_type);}
    if other.time_count != 0 {other_converted.change_unit(this.time_type);}
    if other.current_count != 0 {other_converted.change_unit(this.current_type);}
    if other.temp_count != 0 {other_converted.change_unit(this.temp_type);}
    if other.lumin_count != 0 {other_converted.change_unit(this.lumin_type);}
    if other.amount_count != 0 {other_converted.change_unit(this.amount_type);}

    other_converted
}

