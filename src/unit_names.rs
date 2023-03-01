

use crate::units::*;


    pub fn unit_name(unit: &EngUnit) -> &str {
        let counts = unit.fundamental_counts();

        match counts {
        //  [Ln   Ms  Ti  Cr  Te  Lm  Am]
            [ 1,  0,  0,  0,  0,  0,  0] => "length",
            [ 2,  0,  0,  0,  0,  0,  0] => "area",                
            [ 3,  0,  0,  0,  0,  0,  0] => "volume",
            [ 0,  0,  1,  0,  0,  0,  0] => "time",
            [ 0,  0, -1,  0,  0,  0,  0] => "frequency",
            [ 1,  0, -1,  0,  0,  0,  0] => "velocity",
            [ 1,  0, -2,  0,  0,  0,  0] => "acceleration",
            [ 1,  0, -3,  0,  0,  0,  0] => "jerk",
            [ 0,  1,  0,  0,  0,  0,  0] => "mass",
            [ 0,  1, -1,  0,  0,  0,  0] => "mass rate of change",
            [ 0,  0,  0,  0,  1,  0,  0] => "temperature",
            [ 1,  1, -2,  0,  0,  0,  0] => "force",
            [ 2,  1, -2,  0,  0,  0,  0] => "energy",

            _ => "uknown"           
        }
    }