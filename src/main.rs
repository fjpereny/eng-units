

use eng_units::{EngUnit, Fundamental};


fn main() {

    let mut unit_1 = EngUnit::new();    
    unit_1.numerators.push(Fundamental::Length);
    unit_1.numerators.push(Fundamental::Length);
    unit_1.denominators.push(Fundamental::Time);
    unit_1.value = 5.0;


    let mut unit_2 = EngUnit::new();
    unit_2.numerators.push(Fundamental::Temperature);
    unit_2.denominators.push(Fundamental::Length);
    unit_2.denominators.push(Fundamental::Length);
    unit_2.denominators.push(Fundamental::Length);
    unit_2.value = 2.0;

    let mut unit_3 = unit_1 * unit_2;

    println!("{:?}", unit_3);

    unit_3.rectify_units();
    println!("{:?}", unit_3);

}
