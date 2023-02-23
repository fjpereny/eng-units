

use eng_units::{EngUnit, Unit};


fn main() {

    let mut unit_1 = EngUnit::from_str("1 m").unwrap();
    
    println!("{unit_1}");
    unit_1.change_unit(Unit::Meter);
    println!("{unit_1}");
    println!("{}", unit_1.unit_name());

}
