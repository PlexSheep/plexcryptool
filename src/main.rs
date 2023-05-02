mod binary;
mod iterated_squaring;

use std::str::FromStr;

use iterated_squaring::calc_exp_in_field;

use num_bigint;

pub fn main() {
    let b = num_bigint::BigInt::from_str("17").expect("a");
    let e = num_bigint::BigInt::from_str("1011201391039").expect("a");
    let f = num_bigint::BigInt::from_str("101").expect("a");
    let r = calc_exp_in_field(b.clone(), e, f);
    assert_eq!(r, b);
    print!("res is {}\n", r)
}
