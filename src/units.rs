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
use crate::conversions::convs;


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
pub enum Unit {
    
    // Length
    // (metric)
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    // (Imperial)
    Inch,
    Foot,
    Yard,
    Mile,

    // Time
    Second,
    Minute,
    Hour,

    Temp,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {

            // Length
            // (metric)
            Unit::Kilometer => "km",
            Unit::Meter => "m",
            Unit::Centimeter => "cm",
            Unit::Millimeter => "mm",
            // (Imperial)
            Unit::Inch => "in",
            Unit::Foot => "ft",
            Unit::Yard => "yard",
            Unit::Mile => "mile",

            // Time
            Unit::Second => "sec",
            Unit::Minute => "min",
            Unit::Hour => "hr",

            _ => "Need to finish all the Unit strings",
        };
        write!(f, "{text}")
    }
}



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
            time_type: Unit::Temp,
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

    fn fundamental_counts(&self) -> [i32; 7] {
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

    /// Provides the name of the unit based on fundamental dimensionality
    /// Example 1: [Length] => "length"
    /// Example 2: [Length] / [Time] => "velocity"
    pub fn unit_name(&self) -> String {
        let counts = self.fundamental_counts();

        match counts {
        //  [Ln Ms Ti Cr Te Lm Am]
            [1, 0, 0, 0, 0, 0, 0] => "length".to_string(),
            [2, 0, 0, 0, 0, 0, 0] => "area".to_string(),                
            [3, 0, 0, 0, 0, 0, 0] => "volume".to_string(),
            [0, 0, 1, 0, 0, 0, 0] => "time".to_string(),
            [0, 0, -1, 0, 0, 0, 0] => "frequency".to_string(),
            [1, 0, -1, 0, 0, 0, 0] => "velocity".to_string(),
            [1, 0, -2, 0, 0, 0, 0] => "acceleration".to_string(),

            _ => self.units()
            
        }
    }

    pub fn has_name(&self) -> bool {
        self.units() != self.unit_name()
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

impl Add for EngUnit {
    type Output = EngUnit;
    fn add(self, rhs: Self) -> Self::Output {
        
        if self.length_count != rhs.length_count {
            panic!("Tried to add incomaptible units (length)")
        }
        if self.mass_count != rhs.mass_count {
            panic!("Tried to add incomaptible units (mass)")
        }
        if self.time_count != rhs.time_count {
            panic!("Tried to add incomaptible units (time)")
        }
        if self.current_count != rhs.current_count {
            panic!("Tried to add incomaptible units (eletric current)")
        }
        if self.lumin_count != rhs.lumin_count {
            panic!("Tried to add incomaptible units (luminous intensity)")
        }
        if self.amount_count != rhs.amount_count {
            panic!("Tried to add incomaptible units (amount of substance)")
        }
        
        let mut new_unit = self.clone();        
        let mut rhs_convert = rhs.clone();
        
        if rhs.length_count != 0 {rhs_convert.change_unit(self.length_type);}
        if rhs.mass_count != 0 {rhs_convert.change_unit(self.mass_type);}
        if rhs.time_count != 0 {rhs_convert.change_unit(self.time_type);}
        if rhs.current_count != 0 {rhs_convert.change_unit(self.current_type);}
        if rhs.temp_count != 0 {rhs_convert.change_unit(self.temp_type);}
        if rhs.lumin_count != 0 {rhs_convert.change_unit(self.lumin_type);}
        if rhs.amount_count != 0 {rhs_convert.change_unit(self.amount_type);}

        new_unit.value = self.value + rhs_convert.value;
        new_unit
    }
}

/// Converts a unit's value to the base unit value.
/// Returns the base unit value and base unit type.
/// Example: Unit::KM => 1,000.0, Unit::M
pub fn unit_to_base_val(unit: &Unit) -> f64 {
    match unit {
        
        // Length
        Unit::Kilometer => convs::KILOMETER_TO_BASE,
        Unit::Meter => convs::METER_TO_BASE,
        Unit::Centimeter => convs::CENTIMETER_TO_BASE,
        Unit::Millimeter => convs::MILLIMETER_TO_BASE,

        // TIme
        Unit::Second => convs::SECOND_TO_BASE,
        Unit::Minute => convs::MINUTE_TO_BASE,
        Unit::Hour => convs::HOUR_TO_BASE,

        _ => 0.0,
    }
}

pub fn base_to_unit_val(unit: &Unit) -> f64 {
    1.0 / unit_to_base_val(&unit)
}

pub fn get_base_unit(unit: &Unit) -> Unit {
    let fundamental = get_fundamental(&unit);
    match fundamental {
        Fundamental::Length => Unit::Meter,
        _ => Unit::Temp,
    }
}
