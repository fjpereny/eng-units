

use crate::units::*;


    pub fn unit_name(unit: &EngUnit) -> String {
        let counts = unit.fundamental_counts();

        match counts {
        //  [Ln Ms Ti Cr Te Lm Am]
            [1, 0, 0, 0, 0, 0, 0] => "length".to_string(),
            [2, 0, 0, 0, 0, 0, 0] => "area".to_string(),                
            [3, 0, 0, 0, 0, 0, 0] => "volume".to_string(),
            [0, 0, 1, 0, 0, 0, 0] => "time".to_string(),
            [0, 0, -1, 0, 0, 0, 0] => "frequency".to_string(),
            [1, 0, -1, 0, 0, 0, 0] => "velocity".to_string(),
            [1, 0, -2, 0, 0, 0, 0] => "acceleration".to_string(),

            _ => unit.units()            
        }
    }