#![allow(dead_code)]
use std::{ops::{Mul, Neg}, fmt::Debug};

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

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
/// represent a specific eliptic curve
///
/// real curves not supported, only in Gallois Fields
pub struct ElipticCurve {
    f: GalloisField,
    a: i128,
    b: i128,
    points: Vec<ElipticCurvePoint>,
    verbose: bool,
    INFINITY_POINT: Option<ElipticCurvePoint>
}

impl ElipticCurve {
    pub fn new(f: GalloisField, a: i128, b: i128, verbose: bool) -> Result<Self, String> {
        // check diskriminante
        let d = 4*a.pow(3) + 27*b.pow(2);
        if f.reduce(d) == 0 {
            if verbose {
                println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n\
                Check for Diskriminante not passed", f.reduce(d));
            }
            return Err(String::from("Diskriminante not 0"));
        }
        else if verbose {
                println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n
                Check for Diskriminante passed", f.reduce(d));
        }

        let mut e = ElipticCurve {
            f,
            a,
            b,
            points: Vec::new(),
            verbose,
            INFINITY_POINT: None
        };
        let infty = ElipticCurvePoint::new(0, 0, e.f);
        e.INFINITY_POINT = Some(infty);
        return Ok(e);
    }

    /// calculate a value for coordinates
    pub fn poly<T>(&self, r: T, s: T) -> i128 
        where
        T: Integer,
        T: Mul,
        T: Debug,
        T: num::cast::AsPrimitive<i128>,
        T: Neg
        {
            dbg!(&r);
            dbg!(&s);
            let r: i128 = num::cast::AsPrimitive::as_(r);
            let s: i128 = num::cast::AsPrimitive::as_(s);
            let res =  s.pow(2) - r.pow(3) - (self.a * r) - self.b;
            let res1 = self.f.reduce(res);
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
        let left =  self.f.reduce(p.s.pow(2));
        let right =  self.f.reduce(p.r.pow(3) + self.a*p.r + self.b);
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
}

#[test]
fn test_check_point() {
    let f = GalloisField::new(13, true, None);
    let ec = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
    // real points
    let p = vec![
        ElipticCurvePoint::new(0, 4, f),
        ElipticCurvePoint::new(0, 9, f),
        ElipticCurvePoint::new(1, 1, f),
        ElipticCurvePoint::new(1, 12, f),
        ElipticCurvePoint::new(4, 4, f),
        ElipticCurvePoint::new(4, 9, f),
        ElipticCurvePoint::new(5, 3, f),
        ElipticCurvePoint::new(5, 10, f),
        ElipticCurvePoint::new(7, 0, f),
        ElipticCurvePoint::new(8, 6, f),
        ElipticCurvePoint::new(9, 4, f),
        ElipticCurvePoint::new(9, 9, f),
        ElipticCurvePoint::new(11, 1, f),
        ElipticCurvePoint::new(11, 12, f),
    ];
    // random values, not part of the e, fc.
    let np = vec![
        ElipticCurvePoint::new(0, 5, f),
        ElipticCurvePoint::new(1, 9, f),
        ElipticCurvePoint::new(1, 4, f),
    ];
    for i in p {
        assert!(ec.clone().check_point(i));
    }
    for i in np {
        assert!(!ec.clone().check_point(i));
    }
}


#[derive(Debug, Clone, Copy)]
/// represent a specific eliptic curves point
pub struct ElipticCurvePoint {
    r: i128,
    s: i128,
    is_infinity_point: bool,
    field: GalloisField
}

impl ElipticCurvePoint {
    pub fn new(r: i128, s: i128, field: GalloisField) -> ElipticCurvePoint {
        ElipticCurvePoint {
            r,
            s,
            is_infinity_point: false,
            field
        }
    }

    /// add two points
    pub fn add(self, point: Self) -> Result<Self, String> {
        if self.field.cha != point.field.cha {
            return Err(String::from("Points are not on the same field"));
        }
        if self.field.prime_base {
            // case 1 both infty
            if self.is_infinity_point && point.is_infinity_point {
                return Ok(point);
            }
            // case 2 one is infty
            else if self.is_infinity_point && !point.is_infinity_point {
                return Ok(point);
            }
            else if !self.is_infinity_point && point.is_infinity_point {
                return Ok(self);
            }
            // case 3 r_1 != r_2
            else if self.r != point.r {
                panic!("TODO");
            }
            // case 4 r_1 = r_2; s_1 = -s_2
            else if self.r == point.r && self.s == point.neg().s {
                return Ok(Self::new(0, 0, self.field));
            }

            // how do we get here?
            // this should never occur
            else {
                panic!("we dont know what to do in this case?")
            }
        }
        else {
            return Err(String::from("Only prime fields are supported currently"));
        }
    }

    /// get negative of a point
    pub fn neg(self) -> Self {
        return ElipticCurvePoint::new(
            self.r, 
            self.field.reduce(-(self.s as i128)) as i128, 
            self.field
            );
    }

    /// multip.s a point by an integer
    pub fn mul(self, n: u128) -> Self {
        // TODO
        panic!("TODO");
    }
}
