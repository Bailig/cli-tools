mod algorithms;

use algorithms::array::max_water;

use crate::algorithms::array::backspace_compare;

fn main() {
    let result_1 = max_water(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{}", result_1);
    let result_2 = backspace_compare(String::from("ab###acd"), String::from("ad#cddd###d"));
    println!("{}", result_2);
}
