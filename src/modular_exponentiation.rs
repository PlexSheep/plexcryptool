#![allow(dead_code)]

use num_bigint::{BigInt, BigUint};
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
    let mut instructions: Vec<bool> = bigint_to_bools(exp);
    // remove the signing bit
    if verbose {
        println!("pre instructions {:?}",instructions);
    }

    instructions.reverse();
    if verbose {
        println!("instructions {:?}",instructions);
    }

    let mut res = base.clone();
    for instr in instructions {
        if verbose {
            println!("current res: {res}");
        }
        if !instr {
            // square
            if verbose {
                println!("square");
            }
            res = res.pow(2) % &field;
        }
        else {
            // square and multiply
            if verbose {
                println!("square and multiply");
            }
            res = (res.pow(2) * &base) % &field;
        }
    }

    return res;
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
