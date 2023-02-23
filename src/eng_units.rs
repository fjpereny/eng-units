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



use crate::units::Unit;
use crate::conversions::convs;
use crate::fundamental::Fundamental;
use crate::fundamental::get_fundamental;

#[derive(Debug, Clone)]
/// A structure representing the dimensionality and value of an engineering unit.
pub struct EngUnit {
    pub value: f64,

    pub length_count: i8,
    pub mass_count: i8,
    pub time_count: i8,
    pub current_count: i8,
    pub temp_count: i8,
    pub lumin_count: i8,
    pub amount_count: i8,

    // pub length_numerator: bool,
    // pub mass_numerator: bool,
    // pub time_numerator: bool,
    // pub current_numerator: bool,
    // pub temp_numerator: bool,
    // pub lumin_numerator: bool,
    // pub amount_numerator: bool,

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

            length_count: 0,
            mass_count: 0,
            time_count: 0,
            current_count: 0,
            temp_count: 0,
            lumin_count: 0,
            amount_count: 0,

            // length_numerator: true,
            // mass_numerator: true,
            // time_numerator: true,
            // current_numerator: true,
            // temp_numerator: true,
            // lumin_numerator: true,
            // amount_numerator: true,

            length_type: Unit::Meter,
            mass_type: Unit::Temp,
            time_type: Unit::Temp,
            current_type: Unit::Temp,
            temp_type: Unit::Temp,
            lumin_type: Unit::Temp,
            amount_type: Unit::Temp,
         }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut new_unit = EngUnit::new();
        
        let s_split: Vec<&str> = s.split(" ").collect();
        if s_split.len() != 2 {
            return Err("Incorrect string format for EngUnit");
        }

        new_unit.value = s_split[0].parse::<f64>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse numerical value\nError: {err}");
            });    
            
        let num_den: Vec<&str> = s_split[1].split('/').collect();
        if num_den.len() != 1 && num_den.len() != 2 {
            return Err("Unable to parse numerator and denominator for units");
        }

        let num: Vec<&str> = num_den[0].split('-').collect();
        for n in num {
            let n_split: Vec<&str> = n.split('^').collect();
            let unit = n_split[0];
            let mut power: i8 = 0;
            if n_split.len() == 1 {power = 1}
            if n_split.len() == 2 {
                power = n_split[1].parse::<i8>()
                    .unwrap_or_else(|err| {
                        panic!("Unable to parse the exponent of a unit\nError: {err}");
                    })
            }

            match unit {
                "km" => {
                    new_unit.length_type = Unit::Kilometer;
                    new_unit.length_count = power;
                }

                "m" => {
                    new_unit.length_type = Unit::Meter;
                    new_unit.length_count = power;
                }

                "cm" => {
                    new_unit.length_type = Unit::Centimeter;
                    new_unit.length_count = power;
                }

                "mm" => {
                    new_unit.length_type = Unit::Millimeter;
                    new_unit.length_count = power;
                }
                _ => panic!("Unknown unit type"),
            }
        }        
        
        Ok(new_unit)
    }


    pub fn from_unit(unit: Unit, val: f64) -> EngUnit {
        let mut new_unit = EngUnit::new();
        new_unit.value = val;        
        let fundamental = get_fundamental(&unit);
        match fundamental {
            Fundamental::Length => {
                new_unit.length_type = unit;
                new_unit.length_count = 1;
            },
            Fundamental::Mass => {
                new_unit.mass_type = unit;
                new_unit.mass_count = 1;
            },
            Fundamental::Time => {
                new_unit.time_type = unit;
                new_unit.time_count = 1;
            },
            Fundamental::Current => {
                new_unit.current_type = unit;
                new_unit.current_count = 1;
            },
            Fundamental::Temperature => {
                new_unit.temp_type = unit;
                new_unit.temp_count = 1;
            },
            Fundamental::LuminousIntensity => {
                new_unit.lumin_type = unit;
                new_unit.lumin_count = 1;
            },
            Fundamental::AmountOfSubstance => {
                new_unit.amount_type = unit;
                new_unit.amount_count = 1;
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
        
        for _ in 0..unit_power {
            match fundamental {
                Fundamental::Length => self.value *= unit_to_base_val(&self.length_type),
                Fundamental::Mass => self.value *= unit_to_base_val(&self.mass_type),
                Fundamental::Time => self.value *= unit_to_base_val(&self.time_type),
                Fundamental::Current => self.value *= unit_to_base_val(&self.current_type),
                Fundamental::Temperature => self.value *= unit_to_base_val(&self.temp_type),
                Fundamental::LuminousIntensity => self.value *= unit_to_base_val(&self.lumin_type),
                Fundamental::AmountOfSubstance => self.value *= unit_to_base_val(&self.amount_type),
            }
            self.value *= base_to_unit_val(&unit);
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

    /// Provides the name of the unit based on fundamental dimensionality
    /// Example 1: [Length] => "length"
    /// Example 2: [Length] / [Time] => "velocity"
    pub fn unit_name(&self) -> &'static str {
        let counts = self.fundamental_counts();

        match counts {
        //  [Ln Ms Ti Cr Te Lm Am]
            [1, 0, 0, 0, 0, 0, 0] => "length",
            [2, 0, 0, 0, 0, 0, 0] => "area",                
            [3, 0, 0, 0, 0, 0, 0] => "volume",
            [1, 0, -1, 0, 0, 0, 0] => "velocity",
            [1, 0, -2, 0, 0, 0, 0] => "acceleration",
    
            _ => "Not yet implemented...",
        }
    }

}

impl std::fmt::Display for EngUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = &self.value;
        let mut num = String::new();
        let mut den = String::new();

        if self.length_count >= 1 {
            let s = self.length_type.to_string();
            match self.length_count >= 0 {
                true => num.push_str(&s),
                false => den.push_str(&s),
            }            
        }
        if self.length_count >= 2 {
            match self.length_count >= 0 {
                true => {
                    num.push('^');
                    num.push_str(&self.length_count.to_string());
                },
                false => {
                    den.push('^');
                    den.push_str(&self.length_count.to_string());
                }
            } 
        }

        if self.mass_count >= 1 {
            let s = self.mass_type.to_string();
            match self.mass_count >= 0 {
                true => num.push_str(&s),
                false => den.push_str(&s),
            }            
        }
        if self.mass_count >= 2 {
            match self.mass_count >= 0 {
                true => {
                    num.push('^');
                    num.push_str(&self.mass_count.to_string());
                },
                false => {
                    den.push('^');
                    den.push_str(&self.mass_count.to_string());
                }
            } 
        }

        if self.time_count >= 1 {
            let s = self.time_type.to_string();
            match self.time_count >= 0 {
                true => num.push_str(&s),
                false => den.push_str(&s),
            }            
        }
        if self.time_count >= 2 {
            match self.time_count >= 0 {
                true => {
                    num.push('^');
                    num.push_str(&self.time_count.to_string());
                },
                false => {
                    den.push('^');
                    den.push_str(&self.time_count.to_string());
                }
            } 
        }
        

        if num.len() > 0 && den.len() > 0 {
            return write!(f, "{val} ({num})/({den})");
        }
        else if num.len() > 0 {
            return write!(f, "{val} {num}");
        }
        else {
            return write!(f, "{val} /({den})");
        }        
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