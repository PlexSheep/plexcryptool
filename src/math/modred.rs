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

#[test]
fn test_modred() {
    let rel: u64 = 0x1053;
    let pol0: u64 = 0x100001;
    assert_eq!(modred(pol0, rel, false).unwrap(), 0x21e);
}

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
