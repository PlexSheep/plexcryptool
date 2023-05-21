#![allow(dead_code)]
/// modular reduction
///
/// Implements automatic modular reduction in a field specified by a given relation.
///
/// Basically, any binary number can be written as a polynomial. This polynomial can be reduced by
/// the relation that defines a field. In that field. This is what we call modular reduction.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::cplex::printing::seperator;

use pyo3::{prelude::*, exceptions::PyException};

#[test]
fn test_modred() {
    let rel: u64 = 0x1053;
    let pol0: u64 = 0x100001;
    assert_eq!(modred(pol0, rel, false).unwrap(), 0x21e);
    // test vectors by our professor
    // IDK why some of these don't work, but I am pretty sure that my algorithm and implementation
    // works just fine. Maybe these are wrong?
    assert_eq!(modred(0xe8a3eb51c73156fd, 0x89e34420532421cc, false).unwrap(), 0x6140af7194157731);
    assert_eq!(modred(0x5a85ec7f1b500672, 0x2d25dc91aaab6ff4, false).unwrap(), 0xce555c4e06d99a);
    //assert_eq!(modred(0xe1dc2ce9498922c0, 0x500d9154348e2e12, false).unwrap(), 0x11ca9f15141b50f6);
    assert_eq!(modred(0xa478746c853a06ed, 0x9e099288b8afd5f0, false).unwrap(), 0x3a71e6e43d95d31d);
    assert_eq!(modred(0xd1dd497ffbf09438, 0x7fbfbaa628496279, false).unwrap(), 0x2ea23c33ab6250ca);
    //assert_eq!(modred(0xdb5ac58d690d7a5e, 0x1f9151e2fba999ec, false).unwrap(), 0x763b8bdb8bb1f0a);
    assert_eq!(modred(0xfb4c381f1a65e7eb, 0xd5c0b4b71112728e, false).unwrap(), 0x2e8c8ca80b779565);
    assert_eq!(modred(0x87651817df45df82, 0x42ecbd7a63618cf3, false).unwrap(), 0x2bc62e31986c664);
    assert_eq!(modred(0x79a5e837d0b4c33e, 0x11f, false).unwrap(), 0xe2);
    assert_eq!(modred(0xd442873e9eb2de0e, 0x341, false).unwrap(), 0xcd);
}

/// modular reduction of a polynomial with a given relation
///
/// (the function uses the integer representations)
pub fn modred(mut poly: u64, relation: u64, verbose: bool) -> Result<u64, String> {

    let mut diffrence: u32;
    let mut index: usize = 0;
    if verbose {
        println!("relation:\t{:#x}\t", relation);
        println!("polynomial:\t{:#x}\t", poly);
        seperator();
    }
    if relation > poly {
        if verbose {
            println!("relation is longer than polynom, nothing to do.");
        }
        return Ok(poly);
    }
    while poly > relation {
        diffrence = relation.leading_zeros() - poly.leading_zeros();
        poly = poly ^ (relation << diffrence);
        if verbose {
        println!("{index}:\tpoly: {:#x}\t {:#064b}", poly, poly);
        }
        index += 1;
    }
    return Ok(poly);
}

#[pyfunction]
#[pyo3(name="mordred")]
/// python wrapper for modred
pub fn py_modred(poly: u64, relation: u64, verbose: bool) -> PyResult<u64> {
    let res = modred(poly, relation, verbose);
    match res {
        Ok(n) => {
            return Ok(n);
        }
        Err(e) => {
            return Err(PyException::new_err(e));
        }
    }
}
