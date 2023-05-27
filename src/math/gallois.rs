#![allow(dead_code)]
/// calculation in a gallois field
///
/// This module contains functions that can be used to calculate things in a gallois field
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::{math::modexp, cplex::printing::seperator};

use core::fmt;

use num::Integer;

use pyo3::{prelude::*, exceptions::PyValueError};

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
#[pyclass]
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
        let field = GalloisFiled{
            base,
            // TODO: calculate the characteristic
            cha: 0,
            verbose
        };
        if verbose {
            seperator();
            println!("In Gallois Field F_{}", field.base);
            seperator();
        }
        return field;
    }

    /// reduce a number to fit into the gallois field
    pub fn reduce(self, n: u128) -> u128 {
        return n % self.base;
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
        let egcd = self.reduce(egcd.x as u128);
        return Ok(egcd);
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
    pub fn sqrt(self, a: u128) -> Result<(u128, u128), NoRootError> {
        let pm1 = self.base - 1;
        let pm1_2 = pm1.checked_div(2).expect("Could not divide p-1 by 2");
        let a_pm1_2 = modexp::modular_exponentiation_wrapper(a, pm1_2, self.base, false);
        if self.verbose {
            println!("p-1 = {pm1}\n[p-1]/[2] = {pm1_2}\na**([p-1]/[2]) = {a_pm1_2}");
        }
        if a_pm1_2 != 1 {
            if self.verbose {
                println!("a**([p-1]/[2]) != 1 => a has no root.");
            }
            return Err(NoRootError);
        }

        // 4 | (p + 1):
        if 4 % (self.base + 1) == 0 {
            let w1 = a_pm1_2;
            let w1 = self.reduce(w1);
            let w2 = self.a_inverse(w1);
            if self.verbose {
                seperator();
                println!("4 divides p+1");
                println!("found sqrt of {a} as ({w1}, {w2})");
            }
            return Ok((w1, w2));
        }
        // 4 !| (p + 1):
        else {
            if self.verbose {
                seperator();
                println!("4 does not divide p+1");
                seperator();
            }
            let mut l: u128 = 0;
            let t: u128;
            loop {
                if pm1_2.is_multiple_of(&2u128.pow((l+1) as u32)) {
                    l += 1;
                }
                else {
                    // no more divisible
                    t = pm1_2.checked_div(2u128.pow(l as u32)).expect("Could not divide by 2**l as calculated");
                    // t must be odd
                    assert_eq!(t % 2, 1);
                    break;
                }
            }
            // chose a b so that b_pm1_2 == -1
            let mut b: Option<u128> = None;
            let mut b_pm1_2: u128;
            for b_candidate in 0..self.base {
                b_pm1_2 = modexp::modular_exponentiation_wrapper(b_candidate, pm1_2, self.base, false);
                if self.reduce(b_pm1_2) == self.reduce_neg(-1) {
                    b = Some(b_candidate);
                    if self.verbose {
                        println!("b^([p-1]/[2]) = {}^({pm1_2}) = -1 (mod {})", b.unwrap(), self.base);
                        println!("found a b that fits the criteria: {}", b.unwrap());
                        seperator();
                    }
                    break;
                }
            }
            if b.is_none() {
                if self.verbose {
                    seperator();
                    println!("found no fitting b");
                }
                return Err(NoRootError);
            }
            let b = b.unwrap();
            let mut n: Vec<u128> = vec![0];
            let mut c: Vec<u128> = vec![];
            let mut tmp: u128;
            if self.verbose {
                println!("l = {l}\tt = {t}\tb = {b}");
                println!("let n_0 = 0");
            }
            for index in 0..l {
                if self.verbose {
                    println!("Calculating c_{index}");
                }
                // l-(i+1)
                tmp = l - (index+1);
                if self.verbose {
                    println!("{index}.\tl-(i+1) = {l}-({index}+1) = {tmp}");
                }
                tmp = modexp::modular_exponentiation_wrapper(2, tmp, self.base, false);
                if self.verbose {
                    println!("{index}.\t2^[l-(i+1)] = 2^[{l}-({index}+1)] = {tmp}");
                }
                tmp *= t;
                if self.verbose {
                    println!("{index}.\t2^[l-(i+1)]*t = 2^[{l}-({index}+1)]*t = {tmp}");
                }
                tmp = self.reduce(tmp);
                if self.verbose {
                    println!("{index}.\t2^[l-(i+1)]*t = 2^[{l}-({index}+1)]*t = {tmp} (mod {})", self.base);
                }
                // multiplication with overflow vvvvvvvvvvvvvv
                tmp = modexp::modular_exponentiation_wrapper(a, tmp, self.base, false);
                if self.verbose {
                    println!("{index}.\ta^(2^[l-(i+1)]*t) = {a}^(2^[{l}-({index}+1)]*t) = {tmp}");
                }
                tmp *= modexp::modular_exponentiation_wrapper(b, n[index as usize], self.base, false);
                tmp = self.reduce(tmp);
                if self.verbose {
                    println!("{index}.\ta^(2^[l-(i+1)]*t) * b^(n_{index}) = {a}^(2^[{l}-({index}+1)]*{t}) * {b}^({}) = {tmp} (mod {})", 
                             n[index as usize],
                             self.base
                             );
                }
                c.push(tmp);
                if self.verbose {
                    println!("{index}.\tc_{index} = {}", c[index as usize]);
                    println!("Calculating n_{}", index + 1);
                }
                if c[index as usize] == 1 {
                    if self.verbose {
                        println!("{index}.\tc_{index} = 1 => n_{} = [n_{index}]/[2]", index + 1);
                    }
                    n.push(n[index as usize].checked_div(2).expect("could not compute n[i+1]"));
                    if self.verbose {
                        println!("{index}.\tn_{} = [n_{index} / 2] = [{}]/[2] = {}", 
                                 index + 1, 
                                 n[index as usize], 
                                 n[index as usize]
                                 );
                    }
                }
                else {
                    if self.verbose {
                        println!("{index}.\tc_{index} != 1 => n_{} = [n_{index}]/[2] + [p-1]/[4]", index + 1);
                    }
                    let mut tmp: u128 = n[index as usize].checked_div(2).expect("could not compute n[i+1]");
                    tmp += pm1.checked_div(4).expect("could not compute n[i+1]");
                    n.push(tmp);
                    assert_eq!(n.last().unwrap(), &tmp);
                    if self.verbose {
                        println!("{index}.\tn_{} = [n_{index} / 2] + [p-1]/[4] = [{}]/[2] + [{pm1}]/[4] = {}", 
                                 index + 1, 
                                 n[index as usize], 
                                 n.last().unwrap()
                                 );
                    }
                }
            }
            let exp = (t+1).checked_div(2).expect("cant divide to int");
            let mut w1: u128 = modexp::modular_exponentiation_wrapper(a, exp, self.base, false);
            if self.verbose {
                seperator();
                println!("a^([t+1]/[2]) = {w1}");
            }
            w1 *= modexp::modular_exponentiation_wrapper(b, n[l as usize], self.base, false);
            if self.verbose {
                println!("w_1 = [a^(t+1)]/[2] * b^(n_l) = [{a}^([{t}+1])]/[2] * {b}^{} = {}", n[l as usize], w1);
            }
            w1 = self.reduce(w1);
            if self.verbose {
                println!("w_1 = [a^(t+1)]/[2] * b^(n_l) = [{a}^([{t}+1])]/[2] * {b}^{} = {} (mod {})", 
                       n[l as usize], 
                       w1, 
                       self.base
                       );
            }
            let w2 = self.a_inverse(w1);
            if self.verbose {
                println!("w_2 = -w_1 = -{w1} = {w2} (mod {})", self.base);
            }
            if self.verbose {
                println!("found sqrt of {a} as ({w1}, {w2})");
            }
            return Ok((w1, w2));
        }
    }
}

#[pymethods]
/// python wrappers for the gallois field
impl GalloisFiled {
    #[new]
    pub fn py_new(base: u128, verbose: bool) -> Self {
        return GalloisFiled::new(base, verbose);
    }

    #[pyo3(name="pow")]
    /// calculate the exponent of a base in the field
    pub fn py_pow(&self, base: u128, exp: u128) -> u128 {
        return self.pow(base, exp);
    }
    
    #[pyo3(name="reduce")]
    /// reduce any int
    pub fn py_reduce(&self, n: i128) -> u128 {
        if n.is_negative() {
            return self.reduce_neg(n);
        }
        return self.reduce(n as u128);
    }

    #[pyo3(name="a_inverse")]
    /// find the additive inverse of a number
    pub fn py_a_inverse(&self, n: u128) -> u128 {
        return self.a_inverse(n)
    }

    #[pyo3(name="sqrt")]
    /// calculate the square root of a number in a field
    pub fn py_sqrt(&self, a: u128) -> PyResult<(u128, u128)> {
        match self.sqrt(a) {
            Ok(v) => Ok(v),
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }

    #[pyo3(name="inverse")]
    /// get multiplicative inverse
    pub fn py_inverse(&self, n: u128) -> PyResult<u128> {
        match self.inverse(n) {
            Ok(v) => Ok(v),
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_gallois_sqrt() {
    let field = GalloisFiled::new(977, true);
    assert_eq!(field.sqrt(269).expect("function says there is no root but there is"), (313, 664));
    assert_eq!(field.sqrt(524).expect("function says there is no root but there is"), (115, 862));
    assert_eq!(field.sqrt(275).expect("function says there is no root but there is"), (585, 392));
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

    // TODO add a test for a field that has a non prime base
}
