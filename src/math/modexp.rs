#![allow(dead_code)]
/// modular exponentiaton
///
/// Implements fast exponentiation with applied modulo. Usefull for calculations in a gallois
/// field.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use num_bigint::BigInt;
use num_traits::ToPrimitive;
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
 * square and multiply
 */
pub fn modular_exponentiation(
    base: BigInt,
    exp: BigInt, 
    field: BigInt,
    verbose: bool) -> BigInt {
    if verbose {
        println!("args:\nbase {base}\nexp {exp}\nfield {field}\nverbose {verbose}");
    }
    if exp == BigInt::from(0) {
        return BigInt::from(1);
    }
    let mut instructions: Vec<bool> = bigint_to_bools(exp.clone());
    // remove the signing bit

    instructions.reverse();

    if verbose {
        println!("exponent to binary/bools (discard first bit):\n{:b}\n{:?}", exp, instructions);
    }

    let mut res = base.clone();
    for (index, instr) in instructions.iter().enumerate() {
        if !instr {
            // square
            if verbose {
                print!("{index}. {instr} -> square:\nres = {res}^2 mod {field} = ");
            }
            res = res.pow(2) % &field;
            if verbose {
                println!("{res}");
            }
        }
        else {
            // square and multiply
            if verbose {
                print!("{index}. {instr} -> square and multiply:\nres = {res}^2 * {base} mod {field} = ");
            }
            res = (res.pow(2) * &base) % &field;
            if verbose {
                println!("{res}");
            }
        }
    }

    return res;
}

/// quick wrapper for modular_exponentiation without BigInts
pub fn modular_exponentiation_wrapper(
    base: u128,
    exp: u128, 
    field: u128,
    verbose: bool) -> u128 {
    
    let base = BigInt::from(base);
    let exp = BigInt::from(exp);
    let field = BigInt::from(field);
    return modular_exponentiation(base, exp, field, verbose).to_u128().expect("number too big");
}

#[pyfunction]
#[pyo3(name="modular_exponentiation")]
#[pyo3(signature=(base, orig_exp, field, verbose = false))]
pub fn py_modular_exponentiation(
    base: i128,
    orig_exp: i128, 
    field: i128,
    verbose: bool) -> PyResult<u128> {
    let big_res = modular_exponentiation(
        BigInt::from(base), 
        BigInt::from(orig_exp), 
        BigInt::from(field),
        verbose
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

/// Dont use this buggy mess
pub fn binary_exponentiation(base: BigInt, exp: BigInt, verbose: bool) -> BigInt {
    if exp.clone() < BigInt::from(0) {
        return binary_exponentiation(1/&base, -exp, verbose);
    }
    else if exp.clone() == BigInt::from(0) {
        return BigInt::from(1);
    }
    else if exp.clone() % 2 == BigInt::from(0) {
        return binary_exponentiation(&base*&base, &exp/2, verbose);
    }
    else if exp.clone() % 2 == BigInt::from(1) {
        return binary_exponentiation(&base*&base, (&exp-1)/2, verbose);
    }
    else {
        panic!("I don't know how we got here")
    }
}

fn bigint_to_bools(item: BigInt) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    let mut modul : BigInt;
    let mut smaller = item;
    loop {
        if smaller < BigInt::from(2) {
            break;
        }
        modul = &smaller % BigInt::from(2);
        smaller = &smaller / BigInt::from(2);
        if modul == BigInt::from(1) {
            result.push(true);
        }
        else {
            result.push(false);
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
