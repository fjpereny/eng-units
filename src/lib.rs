use std::ops::Mul;



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

        EngUnit { numerators: new_numerators, denominators: new_denominators, value: new_value }
    }
}