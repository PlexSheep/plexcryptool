#![allow(dead_code)]
use num::Integer;

/// calculation in a gallois field
///
/// This module contains functions that can be used to calculate things in a gallois field
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::math::modexp;

use core::fmt;

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
/// used when trying to find a root for a number which does not have a root.
pub struct NoInverseError;

impl fmt::Display for NoInverseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inverse for 0 does not exist")
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
/// used when trying to find a root for a number which does not have a root.
pub struct DivisionByZeroError;

impl fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "division by zero")
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
/// used when trying to find a root for a number which does not have a root.
pub struct NoRootError;

impl fmt::Display for NoRootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no root in the specified gallois field")
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
/// represent a gallois field
pub struct GalloisFiled {
    base: u128,
    cha: u128,
    verbose: bool,
}

/// implementations for the gallois field
impl GalloisFiled {
    /// make a new gallois field
    pub fn new(base: u128, verbose: bool) -> Self {
        let mut field = GalloisFiled{
            base,
            // TODO: calculate the characteristic
            cha: 0,
            verbose
        };
        if verbose {
            dbg!(&field);
        }
        return field;
    }

    /// reduce a number to fit into the gallois field
    pub fn reduce(self, n: u128) -> u128 {
        let mut n = n;
        if n < 0 {
            while n < 0 {
                n += self.base;
            }
        }
        n %= self.base;
        return n;
    }

    /// reduce a negative number to fit into the gallois field
    pub fn reduce_neg(self, n: i128) -> u128 {
        let mut n = n;
        if n < 0 {
            while n < 0 {
                n += self.base as i128;
            }
        }
        n %= self.base as i128;
        return n as u128;
    }

    /// calculate the exponent of a base in the field
    pub fn pow(self, base: u128, exp: u128) -> u128 {
        return modexp::modular_exponentiation_wrapper(base, exp, self.base, false);
    }

    /// find the additive inverse of a number
    pub fn a_inverse(self, n: u128) -> u128 {
        return self.base - self.reduce(n);
    }

    /// find the multiplicative inverse of a number
    pub fn inverse(self, n: u128) -> Result<u128, NoInverseError> {
        if n == 0 {
            return Err(NoInverseError);
        }
        let egcd = (n as i128).extended_gcd(&(self.base as i128));
        dbg!(n);
        return Ok(egcd.x as u128);
    }

    pub fn divide(self, a: u128, b: u128) -> Result<u128, DivisionByZeroError> {
        let b = self.inverse(b);
        match b {
            Ok(r) => {
                return Ok(self.reduce(a * r));
            }
            Err(e) => {
                dbg!(e);
                return Err(DivisionByZeroError);
            }
        }
    }

    /// calculate the square root of a number in a field
    pub fn sqrt(self, n: u128) -> Result<u128, NoRootError> {
        // TODO implement this
        panic!("TODO")
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_gallois_sqrt() {
    panic!("TODO")
}

#[test]
fn test_gallois_inverse() {
    let field = GalloisFiled::new(31, true);
    assert_eq!(field.inverse(12).unwrap(), 13);
    assert_eq!(field.inverse(28).unwrap(), 10);
    assert!(field.inverse(0).is_err());

    let field = GalloisFiled::new(83, true);
    assert_eq!(field.inverse(6).unwrap(), 14);
    assert_eq!(field.inverse(54).unwrap(), 20);
    assert!(field.inverse(0).is_err());
}
