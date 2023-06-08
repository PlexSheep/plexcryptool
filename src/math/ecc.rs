#![allow(dead_code)]
use std::{ops::{Mul, Neg}, fmt::Debug, f32::consts::PI};

/// eliptic curve cryptograp.s
///
/// This module implements structs and functionalities used for eliptic curve cryptograp.s (ECC).
/// Do not expect it to actually be secure, I made this for cryptograp.s lectures.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use super::gallois::GalloisField;

use num::Integer;
use pyo3::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_snake_case)]
/// represent a specific eliptic curve
///
/// real curves not supported, only in Gallois Fields
/// Eq and PartialEq might behave badly if the verbosity level is not the same. FIXME
pub struct ElipticCurve {
    field: GalloisField,
    a: u128,
    b: u128,
    points: Vec<ElipticCurvePoint>,
    verbose: bool,
    INFINITY_POINT: ElipticCurvePoint
}

impl ElipticCurve {
    pub fn new(field: GalloisField, a: i128, b: i128, verbose: bool) -> Result<Self, String> {

        // convert numbers to u128 in the fields
        let a = field.reduce(a);
        let b = field.reduce(b);

        // check diskriminante
        let d = 4*a.pow(3) + 27*b.pow(2);
        if field.reduce(d) == 0 {
            if verbose {
                println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n\
                Check for Diskriminante not passed", field.reduce(d));
            }
            return Err(String::from("Diskriminante not 0"));
        }
        else if verbose {
            println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n\
                Check for Diskriminante passed", field.reduce(d));
        }

        let mut infty = ElipticCurvePoint::new(0, 0, field, false);
        infty.is_infinity_point = true;
        let infty = infty;
        let mut e = ElipticCurve {
            field,
            a,
            b,
            points: Vec::new(),
            verbose,
            INFINITY_POINT: infty
        };
        return Ok(e);
    }

    /// calculate a value for coordinates
    pub fn poly<T>(&self, r: T, s: T) -> i128 
        where
        T: Integer,
        T: Mul,
        T: Debug,
        T: num::cast::AsPrimitive<u128>,
        T: Neg
        {
            dbg!(&r);
            dbg!(&s);
            let r: u128 = num::cast::AsPrimitive::as_(r);
            let s: u128 = num::cast::AsPrimitive::as_(s);
            let res =  (s.pow(2) as u128) - (r.pow(3) as u128) - (self.a * r) - self.b;
            let res1 = self.field.reduce(res);
            if self.verbose {
                println!("F({}, {}) = {}² - {}³ - {} * {} - {} = {res} = {res1}",
                r, s, s, r, self.a, r, self.b
                );
            }
            return res1 as i128;
        }

    pub fn check_point(self, p: ElipticCurvePoint) -> bool {
        let mut valid = true;

        // insert into poly
        let left =  self.field.reduce(p.s.pow(2));
        let right =  self.field.reduce((p.r.pow(3) as u128) + self.a*p.r + self.b);
        if self.verbose {
            let unred_left = p.s.pow(2);
            let unred_right = p.r.pow(3) + self.a*p.r + self.b;
            println!("All Points need to fullfill this equation:\n\
                    y²\t= x³ + ax + b\n\
                    {}²\t= {}³ + {}*{} +{}\n\
                    {unred_left}\t= {unred_right}\n\
                    {left}\t= {right}\n\
                    <=> {}\n", 
                    p.s,
                    p.r,
                    self.a,
                    p.r,
                    self.b,
                    left == right
                    );
        }
        valid &= left == right;
        return valid;
    }


    /// add two points
    pub fn add(&self, p1: ElipticCurvePoint, p2: ElipticCurvePoint) -> Result<ElipticCurvePoint, String> {
        if p1.field != p2.field {
            return Err(String::from("Points are not on the same field"));
        }
        if p1.field.prime_base {
            // case 1: both infty
            if p1.is_infinity_point && p2.is_infinity_point {
                return Ok(self.INFINITY_POINT);
            }
            // case 2: one is infty
            else if p1.is_infinity_point && !p2.is_infinity_point {
                return Ok(self.INFINITY_POINT);
            }
            else if !p1.is_infinity_point && p2.is_infinity_point {
                return Ok(p1);
            }
            // case 3: r_1 != r_2
            else if p1.r != p2.r {
                if self.field.prime_base {
                    let m = self.field.reduce(p2.s - p1.s) * 
                        self.field.inverse(
                            self.field.reduce(p2.r - p1.r)
                            ).expect("could not find inverse");
                    if self.verbose || p2.verbose {
                        println!("m = [s_2 - s_1]/[r_2 - r_1] = [{} - {}]/[{} - {}] = {} = {}",
                                 p2.s, p1.s, p2.r, p1.r, m, p1.field.reduce(m))
                    }
                    let m = self.field.reduce(m);

                    let r3 = self.field.reduce(m.pow(3)) - p1.r - p2.r;
                    if self.verbose {
                        println!("r_3 = m³ - r_1 - r_2 = {} - {} - {} = {} = {}",
                                 m.pow(3), p1.r, p2.r, r3, p1.field.reduce(r3));
                    }
                    let r3 = self.field.reduce(r3);

                    let s3 = m.pow(3) - 2*m*p1.r - m*p2.r + p1.s;
                    if self.verbose || p2.verbose {
                        println!("s_3 = m³ − 2*m*r_1 − m*r_2 + s1 = {} - 2*{m}*{} - {m}*{} + {} = {} = {}",
                                 m.pow(3), p1.r, p2.r, p1.s, s3, self.field.reduce(s3));
                    }
                    let s3 = self.field.reduce(s3) as i128;
                    let p = ElipticCurvePoint::new(r3, self.field.reduce(-s3), self.field, self.verbose);

                    panic!("TODO");
                }
                else {
                    panic!("TODO");
                }
            }
            // case 4: r_1 = r_2 && s_1 = -s_2
            else if p1.r == p2.r && p1.s == self.neg(p2).s {
                return Ok(self.INFINITY_POINT);
            }
            // case 5: P + P where P = (r, 0)
            else if p1 == p2 && p1.s == 0 {
                return Ok(self.INFINITY_POINT);
            }
            // case 6: P + P where s != 0
            else if p1 == p2 && p1.s != 0 {
                if self.field.prime_base {
                    panic!("TODO");
                }
                else {
                    panic!("TODO");
                }
            }

            // how do we get here?
            // this should never occur
            else {
                panic!("No rules for adding these two points, should not be possible mathmatically.")
            }
        }
        else {
            return Err(String::from("Only prime fields are supported currently"));
        }
    }

    /// get negative of a point
    pub fn neg(&self, p: ElipticCurvePoint) -> ElipticCurvePoint {
        return ElipticCurvePoint::new(
            p.r, 
            p.field.reduce(-(p.s as i128)), 
            p.field,
            p.verbose
            );
    }

    /// multip.s a point by an integer
    pub fn mul(self, p: ElipticCurvePoint, n: u128) -> ElipticCurvePoint {
        // TODO
        panic!("TODO");
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// represent a specific eliptic curves point
///
/// PartialEq and Eq might behave badly with diffrent verbosity FIXME
pub struct ElipticCurvePoint {
    r: u128,
    s: u128,
    is_infinity_point: bool,
    field: GalloisField,
    verbose: bool
}

impl ElipticCurvePoint {
    /// create a new point
    pub fn new(r: u128, s: u128, field: GalloisField, verbose: bool) -> ElipticCurvePoint {
        ElipticCurvePoint {
            r,
            s,
            is_infinity_point: false,
            field,
            verbose
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test] 
    fn test_eliptic_curve_new() {
        let f = GalloisField::new(7, true, None);
        let _ = ElipticCurve::new(f, 1, 2, true).expect_err("invalid ec can be created");
        let _ = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
    }

    #[test]
    fn test_check_point() {
        let f = GalloisField::new(13, true, None);
        let ec = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
        // real points
        let p = vec![
            ElipticCurvePoint::new(0, 4, f, false),
            ElipticCurvePoint::new(0, 9, f, false),
            ElipticCurvePoint::new(1, 1, f, false),
            ElipticCurvePoint::new(1, 12, f, false),
            ElipticCurvePoint::new(4, 4, f, false),
            ElipticCurvePoint::new(4, 9, f, false),
            ElipticCurvePoint::new(5, 3, f, false),
            ElipticCurvePoint::new(5, 10, f, false),
            ElipticCurvePoint::new(7, 0, f, false),
            ElipticCurvePoint::new(8, 6, f, false),
            ElipticCurvePoint::new(9, 4, f, false),
            ElipticCurvePoint::new(9, 9, f, false),
            ElipticCurvePoint::new(11, 1, f, false),
            ElipticCurvePoint::new(11, 12, f, false),
        ];
        // random values, not part of the e, fc.
        let np = vec![
            ElipticCurvePoint::new(0, 5, f, false),
            ElipticCurvePoint::new(1, 9, f, false),
            ElipticCurvePoint::new(1, 4, f, false),
        ];
        for i in p {
            assert!(ec.clone().check_point(i));
        }
        for i in np {
            assert!(!ec.clone().check_point(i));
        }
    }
}
