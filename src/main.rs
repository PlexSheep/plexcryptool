mod binary;
mod modular_exponentiation;

use std::str::FromStr;

use modular_exponentiation::modular_exponentiation;

use num_bigint;

pub fn main() {
    let b = num_bigint::BigInt::from_str("17010010101018924824").expect("a");
    let e = num_bigint::BigInt::from_str("2024254424298472398479283759238759375392875932875928375932875239857329857923889289282975291").expect("a");
    let f = num_bigint::BigInt::from_str("101012").expect("a");
    let r = modular_exponentiation(b.clone(), e, f);
    print!("res is {}\n", r)
}
