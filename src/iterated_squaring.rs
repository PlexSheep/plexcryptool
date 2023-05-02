use std::{str::FromStr, array};

use num_bigint::{BigInt, BigUint}; 

/// works, but is forbidden for class
pub fn calc_exp_in_field_lib(
    base: BigInt,
    exp: BigInt, 
    field: BigInt) -> BigInt {
    base.modpow(&exp, &field)
}

/**
 *  Umwandlung des Exponenten k in die zugehörige Binärdarstellung.
 *  Ersetzen jeder 0 durch Q und jeder 1 durch QM.
 *  Nun wird Q als Anweisung zum Quadrieren und M als Anweisung zum Multiplizieren aufgefasst.
 *  Somit bildet die resultierende Zeichenkette von links nach rechts gelesen eine Vorschrift zur Berechnung von x k . 
 *  Man beginne mit 1, quadriere für jedes gelesene Q das bisherige Zwischenergebnis und 
 *  multipliziere es für jedes gelesene M mit x .
 */
pub fn calc_exp_in_field(
    base: BigInt,
    exp: BigInt, 
    field: BigInt) -> BigInt {
    let binary_repr = exp.to_bytes_be();
    dump_bin(&binary_repr.1);


    let instructions: Vec<bool> = bytes_to_bools(&binary_repr.1);
    dbg!(instructions);

    return base;
}

fn bytes_to_bools(bytes: &Vec<u8>) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    for byte in bytes {
        for c in format!("{:b}", byte).chars() {
            result.push(c == '1');
        }
    }
    result
}

fn dump_bin(bytes: &Vec<u8>) {
    let mut total: Vec<u8> = Vec::new();
    for byte in bytes.iter() {
        println!("{:#08b}\t| {:#02x}", byte, byte);
    }
    print!("0b");
    for byte in bytes.iter() {
        print!("{:08b}", byte);
    }
    println!();

}
