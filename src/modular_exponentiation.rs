#![allow(dead_code)]

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use num_traits::FromPrimitive;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// works, but is forbidden for class
pub fn calc_exp_in_field_lib(
    base: BigInt,
    exp: BigInt, 
    field: BigInt) -> BigInt {
    base.modpow(&exp, &field)
}

/**
 *  modular exponentiation algorithm with big numbers.
 *
 *  Umwandlung des Exponenten k in die zugehörige Binärdarstellung.
 *  Ersetzen jeder 0 durch Q und jeder 1 durch QM.
 *  Nun wird Q als Anweisung zum Quadrieren und M als Anweisung zum Multiplizieren aufgefasst.
 *  Somit bildet die resultierende Zeichenkette von links nach rechts gelesen eine Vorschrift zur Berechnung von x k . 
 *  Man beginne mit 1, quadriere für jedes gelesene Q das bisherige Zwischenergebnis und 
 *  multipliziere es für jedes gelesene M mit x .
 */
pub fn modular_exponentiation(
    base: BigInt,
    orig_exp: BigInt, 
    field: BigInt) -> BigInt {
    let binary_repr = orig_exp.to_bytes_be();

    let instructions: Vec<bool> = bytes_to_bools(&binary_repr.1);

    let mut exp = BigInt::from(1);
    for instr in instructions {
        if instr {
            // square
            exp = (exp.pow(2) * &base) % &field;
        }
        else {
            // square and multiply
            exp = exp.pow(2) % &field;
        }
    }

    return exp;
}

#[pyfunction]
pub fn py_modular_exponentiation(
    base: i128,
    orig_exp: i128, 
    field: i128) -> PyResult<u128> {
    let big_res = modular_exponentiation(
        BigInt::from(base), 
        BigInt::from(orig_exp), 
        BigInt::from(field)
        );
    let res = big_res.to_u128();
    match res {
        Some(v) => {
            return Ok(v);
        }
        None => {
            return Err(PyValueError::new_err("result is too big!"));
        }
    }

}

// Vec<u8> to Vec<bool> ( binary representation interpreted otherwise )
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
    for byte in bytes.iter() {
        println!("{:#08b}\t| {:#02x}", byte, byte);
    }
    print!("0b");
    for byte in bytes.iter() {
        print!("{:08b}", byte);
    }
    println!();

}
