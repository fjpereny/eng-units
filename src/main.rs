

use eng_units::{EngUnit, Unit};


fn main() {

    let mut unit_1 = EngUnit::new();
    unit_1.value = 1.0;
    unit_1.length_type = Unit::Kilometer;
    unit_1.length_count = 1;
    unit_1.length_numerator = true;
    unit_1.time_type = Unit::Second;
    unit_1.time_count = 2;
    unit_1.time_numerator = false;

    println!("{unit_1}");
    unit_1.change_unit(Unit::Meter);
    unit_1.change_unit(Unit::Hour);
    println!("{unit_1}");
    println!("{}", unit_1.unit_name());

}
