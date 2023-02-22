

use std::ops::Mul;
use std::ops::Div;


#[derive(Debug, Clone)]
pub enum Unit {
    // Lengths (metric)
    KM,
    M,
    CM,
    MM,
    // Lengths (Imperial)
    Inch,
    Foot,
    Yard,
    Mile,
}


/// Converts a unit's value to the base unit value.
/// Returns the base unit value and base unit type.
/// Example: Unit::KM => 1,000.0, Unit::M
fn unit_to_base_val(unit: Unit) -> (f64, Unit) {
    match unit {
        Unit::KM => (1_000.0, Unit::M),
        Unit::M => (1.0, Unit::M),
        Unit::CM => (100.0, Unit::M),
        Unit::MM => (1.0 / 1000.0, Unit::M),
        _ => (0.0, Unit::Mile)
    }
}


#[derive(Debug, Clone)]
/// # Fundamental engineering units
/// All units of measurement are derived from one or more fundamental units.
pub enum Fundamental {
    /// Length - meter (m)
    Length,    
    /// Mass - kilogram (kg)
    Mass,
    /// Time - seconds (s)
    Time,
    /// Electrical Current - ampere (A)
    Current,
    /// Temperature - degree Kelvin (K)
    Temperature,
    /// Luminous Intensity - candela (cd)
    LuminousIntensity,
    /// Amount of Substance - mole (mol)
    AmountOfSubstance,
}

#[derive(Debug, Clone)]
/// A structure representing the dimensionality and value of an engineering unit.
pub struct EngUnit {
    /// The dimensional value of the unit numerator
    /// example: 10.23m^2/s -> <m><m>
    pub numerators: Vec<Fundamental>,
    /// The dimensional value of the unit denominator
    /// example: 10.23m^2/s -> <s>
    pub denominators: Vec<Fundamental>,
    /// The current value of the unit
    /// example: 10.23m^2/s -> 10.23
    pub value: f64,
}

impl EngUnit {
    pub fn new() -> Self {
        EngUnit { numerators: Vec::new(), denominators: Vec::new(), value: 0.0 }
    }

    fn numerator_repeat(&mut self, unit: Fundamental, count: i32) {
        for _ in 0..count {
            let new_unit = unit.clone();
            self.numerators.push(new_unit);
        }
    }

    fn denominator_repeat(&mut self, unit: Fundamental, count: i32) {
        for _ in 0..count {
            let new_unit = unit.clone();
            self.denominators.push(new_unit);
        }
    }    

    pub fn rectify_units(&mut self) {
        let mut count_num_length = 0;    
        let mut count_num_mass = 0;
        let mut count_num_time = 0;
        let mut count_num_current = 0;
        let mut count_num_temp = 0;
        let mut count_num_lumin = 0;
        let mut count_num_amount = 0;
        let mut count_den_length = 0;
        let mut count_den_mass = 0;
        let mut count_den_time = 0;
        let mut count_den_current = 0;
        let mut count_den_temp = 0;
        let mut count_den_lumin = 0;
        let mut count_den_amount = 0;

        for num in &self.numerators {
            match num {
                Fundamental::Length => count_num_length += 1,
                Fundamental::Mass => count_num_mass += 1,
                Fundamental::Time => count_num_time += 1,
                Fundamental::Current => count_num_current += 1,
                Fundamental::Temperature => count_num_temp += 1,
                Fundamental::LuminousIntensity => count_num_lumin += 1,
                Fundamental::AmountOfSubstance => count_num_amount += 1,
            }
        }
        for den in &self.denominators {
            match den {
                Fundamental::Length => count_den_length += 1,
                Fundamental::Mass => count_den_mass += 1,
                Fundamental::Time => count_den_time += 1,
                Fundamental::Current => count_den_current += 1,
                Fundamental::Temperature => count_den_temp += 1,
                Fundamental::LuminousIntensity => count_den_lumin += 1,
                Fundamental::AmountOfSubstance => count_den_amount += 1,
            }
        }

        self.numerators = Vec::new();
        self.denominators = Vec::new();

        if count_num_length > count_den_length {
            self.numerator_repeat(Fundamental::Length, count_num_length - count_den_length);
        } else if count_den_length > count_num_length {
            self.denominator_repeat(Fundamental::Length, count_den_length - count_num_length);
        }

        if count_num_mass > count_den_mass {
            self.numerator_repeat(Fundamental::Mass, count_num_mass - count_den_mass);
        } else if count_den_mass > count_num_mass {
            self.denominator_repeat(Fundamental::Mass, count_den_mass - count_num_mass);
        }

        if count_num_time > count_den_time {
            self.numerator_repeat(Fundamental::Time, count_num_time - count_den_time);
        } else if count_den_time > count_num_time {
            self.denominator_repeat(Fundamental::Time, count_den_time - count_num_time);
        }

        if count_num_current > count_den_current {
            self.numerator_repeat(Fundamental::Current, count_num_current - count_den_current);
        } else if count_den_current > count_num_current {
            self.denominator_repeat(Fundamental::Current, count_den_current - count_num_current);
        }

        if count_num_temp > count_den_temp {
            self.numerator_repeat(Fundamental::Temperature, count_num_temp - count_den_temp);
        } else if count_den_temp > count_num_temp {
            self.denominator_repeat(Fundamental::Temperature, count_den_temp - count_num_temp);
        }

        if count_num_lumin > count_den_lumin {
            self.numerator_repeat(Fundamental::LuminousIntensity, count_num_lumin - count_den_lumin);
        } else if count_den_lumin > count_num_lumin {
            self.denominator_repeat(Fundamental::LuminousIntensity, count_den_lumin - count_num_lumin);
        }

        if count_num_amount > count_den_amount {
            self.numerator_repeat(Fundamental::AmountOfSubstance, count_num_amount - count_den_amount);
        } else if count_den_amount > count_num_amount {
            self.denominator_repeat(Fundamental::AmountOfSubstance, count_den_amount - count_num_amount);
        }

    }
}


impl Mul<EngUnit> for EngUnit {
    type Output = EngUnit;

    /// Multiply two units together to build a new unit
    fn mul(self, other: EngUnit) -> EngUnit {
        
        let mut new_numerators = self.numerators;
        for numerator in other.numerators {
            new_numerators.push(numerator);
        }

        let mut new_denominators = self.denominators;
        for denominator in other.denominators {
            new_denominators.push(denominator);
        }

        let new_value = self.value * other.value;

        let mut new_unit = EngUnit { 
            numerators: new_numerators, 
            denominators: new_denominators, 
            value: new_value 
        };
        new_unit.rectify_units();
        new_unit
    }
}


impl Div<EngUnit> for EngUnit {
    type Output = EngUnit;

    /// Divide two units together to build a new unit
    fn div(self, other: EngUnit) -> EngUnit {
        
        let mut new_numerators = self.numerators;
        for denominator in other.denominators {
            new_numerators.push(denominator);
        }

        let mut new_denominators = self.denominators;
        for numerator in other.numerators {
            new_denominators.push(numerator);
        }

        let new_value = self.value / other.value;

        let mut new_unit = EngUnit { 
            numerators: new_numerators, 
            denominators: new_denominators, 
            value: new_value 
        };
        new_unit.rectify_units();
        new_unit 
    }
}


