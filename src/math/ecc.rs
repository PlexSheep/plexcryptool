#![allow(dead_code)]
use crate::cplex::printing::seperator;

/// eliptic curve cryptograp.s
///
/// This module implements structs and functionalities used for eliptic curve cryptograp.s (ECC).
/// Do not expect it to actually be secure, I made this for cryptograp.s lectures.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use super::gallois::GalloisField;

use std::fmt::Debug;

use num::{Integer, Unsigned, NumCast};

use bitvec::prelude::*;

use pyo3::{prelude::*, exceptions::PyValueError};

#[pyclass]
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
    pub fn new<T>(field: GalloisField, a: T, b: T, verbose: bool) -> Result<Self, String>
    where
        T: Integer,
        T: Debug,
        T: num::cast::AsPrimitive<i128>,
    {
        // convert from generics to i128
        let a: i128 = num::cast::AsPrimitive::as_(a);
        let b: i128 = num::cast::AsPrimitive::as_(b);

        // reduce a and b if possible
        let a = field.reduce::<_, u128>(a);
        let b = field.reduce::<_, u128>(b);

        if verbose {
            println!("On eliptic curve:\n\
            F(X, Y) = Y² - X³ - {a}X - {b}")
        }

        // check diskriminante
        let d = 4*a.pow(3) + 27*b.pow(2);
        if field.reduce::<_, u128>(d) == 0 {
            if verbose {
                println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n\
                Check for Diskriminante not passed", field.reduce::<_, u128>(d));
            }
            return Err(String::from("Diskriminante not 0"));
        }
        else if verbose {
            println!("4*{a}³ + 27*{b}² = {d} = {} != 0\n\
                Check for Diskriminante passed", field.reduce::<_, u128>(d));
        }

        let mut infty = ElipticCurvePoint::new(0, 0);
        infty.is_infinity_point = true;
        let infty = infty;
        let e = ElipticCurve {
            field,
            a,
            b,
            points: Vec::new(),
            verbose,
            INFINITY_POINT: infty
        };
        return Ok(e);
    }

    /// build a new point in the EC
    pub fn new_point(&self, r: u128, s: u128) -> Result<ElipticCurvePoint, String> {
        let p = ElipticCurvePoint::new(r, s);
        if self.verbose {
            println!("{p}")
        }
        match self.check_point(p, self.verbose) {
            true => {
                return Ok(p);
            }
            false => {
                return Err(String::from("the point you want to create is not on the EC"));
            }
        }
    }

    /// calculate a value for coordinates
    pub fn poly<T>(&self, r: T, s: T) -> i128 
        where
        T: Integer,
        T: Debug,
        T: num::cast::AsPrimitive<u128>,
        {
            let r: u128 = num::cast::AsPrimitive::as_(r);
            let s: u128 = num::cast::AsPrimitive::as_(s);
            let res =  (s.pow(2) as u128) - (r.pow(3) as u128) - (self.a * r) - self.b;
            let res1 = self.field.reduce::<_, u128>(res);
            if self.verbose {
                println!("F({}, {}) = {}² - {}³ - {} * {} - {} = {res} = {res1}",
                r, s, s, r, self.a, r, self.b
                );
            }
            return res1 as i128;
        }

    pub fn check_point(&self, p: ElipticCurvePoint, verbose: bool) -> bool {
        if p.is_infinity_point {
            println!("p is infinity: {p}");
            return true;
        }
        let mut valid = true;

        // insert into poly
        let left =  self.field.reduce::<_, u128>(p.s.pow(2));
        let right =  self.field.reduce::<_, u128>((p.r.pow(3) as u128) + self.a*p.r + self.b);
        if self.verbose && verbose {
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
    pub fn add(&self, p1: ElipticCurvePoint, p2: ElipticCurvePoint) -> 
        Result<ElipticCurvePoint, String> {
        if self.verbose {
            seperator();
            println!("adding {p1} + {p2}");
            seperator();
        }
        if !self.check_point(p1, false) {
            return Err(String::from("{p1} is not a valid point"));
        }
        if !self.check_point(p2, false) {
            return Err(String::from("{p2} is not a valid point"));
        }
        if self.field.prime_base {
            // verbisity stuff
            //if self.verbose {
            //    println!("{} = {}; {} = -{} = {} <=> {}",
            //             p1.r, p2.r, p1.s, p2.s, self.neg(p2).s, 
            //             p1.r == p2.r && p1.s == self.neg(p2).s,
            //             );
            //}
            // case 1: both infty
            if p1.is_infinity_point && p2.is_infinity_point {
                if self.verbose {
                    println!("case 1");
                }
                return Ok(self.INFINITY_POINT);
            }
            // case 2: one is infty
            else if p1.is_infinity_point && !p2.is_infinity_point || 
                    !p1.is_infinity_point && p2.is_infinity_point
                {
                if self.verbose {
                    println!("case 2");
                }
                return Ok(self.INFINITY_POINT);
            }
            // case 3: r_1 != r_2
            else if p1.r != p2.r {
                if self.verbose {
                    println!("case 3");
                }
                if self.field.prime_base {
                    let m: u128 = self.field.reduce::<i128, u128>(p2.s as i128 - p1.s as i128) * 
                        self.field.inverse(
                            self.field.reduce::<i128, u128>(p2.r as i128 - p1.r as i128)
                            ).expect("could not find inverse");
                    let m: i128 = m as i128;
                    if self.verbose {
                        println!("m = [s_2 - s_1]/[r_2 - r_1] = [{} - {}]/[{} - {}] = {} = {}",
                                 p2.s, p1.s, p2.r, p1.r, m, self.field.reduce::<_, u128>(m))
                    }
                    let m: i128 = self.field.reduce(m);

                    let r3 = m.pow(2) - p1.r as i128 - p2.r as i128;
                    if self.verbose {
                        println!("r_3 = m² - r_1 - r_2 = {} - {} - {} = {} = {}",
                                 m.pow(2), p1.r, p2.r, r3, self.field.reduce::<_, u128>(r3));
                    }
                    let r3 = self.field.reduce::<_, u128>(r3);

                    let s3 = m.pow(3) - 2*m*p1.r as i128 - m*p2.r as i128 + p1.s as i128;
                    if self.verbose {
                        println!("s_3 = m³ − 2*m*r_1 − m*r_2 + s1 =\
                        {} - 2*{m}*{} - {m}*{} + {} = {} = {}",
                                 m.pow(3), p1.r, p2.r, p1.s, s3, 
                                 self.field.reduce::<_, u128>(s3));
                    }
                    let s3 = self.field.reduce::<_, u128>(s3) as i128;
                    if self.verbose {
                        println!("-s_3 = - {s3} = {}", self.field.reduce::<_, u128>(-s3));
                        println!("Q = ({}, {})", r3, s3);
                    }
                    let p3 = self.new_point(r3, self.field.reduce::<_, u128>(-s3)).expect("calculated point does not exist");

                    if self.verbose {
                        seperator();
                        println!("result: ({}, {})", p3.r, p3.s);
                        seperator();
                    }
                    return Ok(p3);
                }
                else {
                    panic!("TODO");
                }
            }
            // case 4: r_1 = r_2 && s_1 = -s_2
            else if p1.r == p2.r && p1.s == self.neg(p2).s {
                if self.verbose {
                    println!("case 4");
                }
                return Ok(self.INFINITY_POINT);
            }
            // case 5: P + P where P = (r, 0)
            else if p1 == p2 && p1.s == 0 {
                if self.verbose {
                    println!("case 5");
                }
                return Ok(self.INFINITY_POINT);
            }
            // case 6: P + P where s != 0
            else if p1 == p2 && p1.s != 0 {
                if self.verbose {
                    println!("case 6");
                }
                if self.field.prime_base {
                    let m: i128 = (self.field.reduce::<_, u128>(3 * p1.r.pow(2) + self.a) * 
                        self.field.inverse(
                            self.field.reduce::<u128, u128>(2 * p1.s)
                            ).expect("could not find inverse")) as i128;
                    if self.verbose {
                        println!("m = [3*r² + a]/[2s] = [3*{}² + {}]/[2*{}] = \
                        {}/{} = \
                        {}*{} = \
                        {} = {}",
                                 p1.r, self.a, p1.s,

                                 self.field.reduce::<_, u128>(3 * p1.r.pow(2) + self.a),
                                 2 * p1.s, 

                                 self.field.reduce::<_, u128>(3 * p1.r.pow(2) + self.a),
                                 self.field.inverse(self.field.reduce::<u128, u128>(2 * p1.s)).unwrap(), 

                                 m,
                                 self.field.reduce::<_, u128>(m)
                                 );
                    }
                    let m: i128 = self.field.reduce(m);

                    let r3: i128 = self.field.reduce::<_, i128>(m.pow(2)) - p1.r as i128 - p2.r as i128;
                    if self.verbose {
                        println!("r_3 = m² - r_1 - r_2 = {} - {} - {} = {} = {}",
                                 m.pow(2), p1.r, p2.r, r3, self.field.reduce::<_, u128>(r3));
                    }
                    let r3: i128 = self.field.reduce(r3);

                    let s3: i128 = m.pow(3) - 2*m*p1.r as i128 - m*p2.r as i128 + p1.s as i128;
                    if self.verbose {
                        println!("s_3 = m³ − 2*m*r_1 − m*r_2 + s1 = {} - 2*{m}*{} - {m}*{} + {} = \
                        {} = {}",
                                 m.pow(3), p1.r, p2.r, p1.s, s3, self.field.reduce::<_, u128>(s3));
                    }
                    let s3: i128 = self.field.reduce(s3);
                    let p3 = self.new_point(r3 as u128, self.field.reduce::<_, u128>(-s3)).expect("calculated point does not exist in curve");

                    if self.verbose {
                        seperator();
                        println!("result: ({}, {})", p3.r, p3.s);
                        seperator();
                    }
                    return Ok(p3);
                }
                else {
                    panic!("TODO");
                }
            }

            // how do we get here?
            // this should never occur
            else {
                panic!("No rules for adding these two points, mathmatically impossible.")
            }
        }
        else {
            return Err(String::from("Only prime fields are supported currently"));
        }
    }

    /// get negative of a point
    pub fn neg(&self, p: ElipticCurvePoint) -> ElipticCurvePoint {
        self.new_point(p.r, self.field.reduce::<_, u128>(-(p.s as i128))).expect("negation of \
        point is not on field, math error")
    }

    /// multip.s a point by an integer
    pub fn mul<T>(&self, g: ElipticCurvePoint, t: T) -> Result<ElipticCurvePoint, String>
        where
        T: Integer,
        T: NumCast,
        T: Debug,
        T: Unsigned,
    {
        if !self.check_point(g, false) {
            return Err(String::from("invalid point"));
        }
        let t: usize = num::cast(t).unwrap();
        if t < 1 {
            return Err(String::from("point multiplication works only if t > 0"));
        }
        if self.verbose {
            println!("h = t * g = {t} * {g}\n\
            t = [{:b}]2", t)
        }
        let mut t_bits = BitVec::<_, Msb0>::from_element(t);
        t_bits.reverse();
        while t_bits[t_bits.len() - 1] == false {
            t_bits.pop();
        }
        t_bits.reverse();
        let l = t_bits.len() - 1;
        let mut lh: ElipticCurvePoint = g;
        let mut h: ElipticCurvePoint = g;
        let mut index: usize = l;
        if l == 0 {
            return Ok(h);
        }
        for bit in t_bits {
            if index == l {
                if self.verbose {
                    println!("h_{index} = {h}")
                }
                index -= 1;
                continue;
            }
            h = self.add(lh, lh).expect("error while performing point multiplication");
            if bit == true {
                h = self.add(h, g).expect("error while performing point multiplication");
            }
            // else h = h
            assert!(self.check_point(h, false));
            lh = h;
            if self.verbose {
                println!("h_{index} = {h}")
            }
            if index != 0 {
                index -= 1;
            }
        }
        // now we should have reached h_0

        return Ok(h);
    }
}

#[pymethods]
impl ElipticCurve {
    #[new]
    pub fn py_new(field: GalloisField, a: i128, b: i128, verbose: bool) -> PyResult<Self> {
        match Self::new(field, a, b, verbose) {
            Ok(v) => {return Ok(v)},
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }

    #[pyo3(name="new_point")]
    pub fn py_new_point(&self, r: u128, s: u128) -> PyResult<ElipticCurvePoint> {
        match self.new_point(r, s) {
            Ok(v) => {return Ok(v)},
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }

    #[pyo3(name="poly")]
    pub fn py_poly(&self, x: i128, y: i128) -> i128 {
        self.poly(x, y)
    }

    #[pyo3(name="check_point", signature=(p, verbose = true))]
    pub fn py_check_point(&self, p: ElipticCurvePoint, verbose: bool) -> bool {
        self.check_point(p, verbose)
    }

    #[pyo3(name="add")]
    pub fn py_add(&self, p1: ElipticCurvePoint, p2: ElipticCurvePoint) 
        -> PyResult<ElipticCurvePoint> {
        match self.add(p1, p2) {
            Ok(v) => {return Ok(v)},
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }

    #[pyo3(name="neg")]
    pub fn py_neg(&self, p: ElipticCurvePoint) -> ElipticCurvePoint {
        self.neg(p)
    }

    #[pyo3(name="mul")]
    pub fn py_mul(&self, p1: ElipticCurvePoint, t: u128) 
        -> PyResult<ElipticCurvePoint> {
        match self.mul(p1, t) {
            Ok(v) => {return Ok(v)},
            Err(e) => {
                let py_e = PyValueError::new_err(e.to_string());
                return Err(py_e)
            }
        }
    }

    fn __str__(&self) -> PyResult<String>   {
        Ok(format!("{}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

impl std::fmt::Display for ElipticCurve{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "F(X, Y) = Y² - X³ -{}X - {}", self.a, self.b)
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// represent a specific eliptic curves point
///
/// PartialEq and Eq might behave badly with diffrent verbosity FIXME
pub struct ElipticCurvePoint {
    r: u128,
    s: u128,
    is_infinity_point: bool,
}

#[pymethods]
impl ElipticCurvePoint {
    #[new]
    /// create a new point
    pub fn new(r: u128, s: u128) -> ElipticCurvePoint {
        ElipticCurvePoint {
            r,
            s,
            is_infinity_point: false,
        }
    }

    fn __str__(&self) -> PyResult<String>   {
        Ok(format!("{}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

impl std::fmt::Display for ElipticCurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_infinity_point {
            write!(f, "(∞ INFINITY)")
        }
        else {
            write!(f, "({}, {})", self.r, self.s)
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
            ec.new_point(0,     4   ).unwrap(),
            ec.new_point(0,     9   ).unwrap(),
            ec.new_point(1,     1   ).unwrap(),
            ec.new_point(1,     1   ).unwrap(),
            ec.new_point(4,     4   ).unwrap(),
            ec.new_point(4,     9   ).unwrap(),
            ec.new_point(5,     3   ).unwrap(),
            ec.new_point(5,     10  ).unwrap(),
            ec.new_point(7,     0   ).unwrap(),
            ec.new_point(8,     6   ).unwrap(),
            ec.new_point(9,     4   ).unwrap(),
            ec.new_point(9,     9   ).unwrap(),
            ec.new_point(11,    1   ).unwrap(),
            ec.new_point(11,    12  ).unwrap(),
        ];
        // random values, not part of the e, fc.
        let np = vec![
            ec.new_point(0, 5).unwrap_err(),
            ec.new_point(1, 9).unwrap_err(),
            ec.new_point(1, 4).unwrap_err(),
        ];
    }

    #[test]
    fn test_add_points() {
        let f = GalloisField::new(13, true, None);
        let ec = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
        let p1 = ec.new_point(1, 1).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(5, 3).expect("point is on ec but an error occurs");
        let p3 = ec.new_point(4, 4).expect("point is on ec but an error occurs");
        let p4 = ec.new_point(8, 6).expect("point is on ec but an error occurs");
        let p5 = ec.new_point(11, 12).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p2).expect("error for possible addition"), p3);
        assert_eq!(ec.add(p2, p4).expect("error for possible addition"), p1);
        assert_eq!(ec.add(p1, p1).expect("error for possible addition"), p5);
        let ec = ElipticCurve::new(f, 7, 11, true).expect("ec cant be created");
        let p1 = ec.new_point(4, 5).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(6, 10).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p1).expect("error for possible addition"), p2);

        let f = GalloisField::new(17, true, None);
        let ec = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
        let p1 = ec.new_point(3, 2).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(11, 3).expect("point is on ec but an error occurs");
        let p3 = ec.new_point(7, 6).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p2).expect("error for possible addition"), p3);
        let p4 = ec.new_point(9, 5).expect("point is on ec but an error occurs");
        let p5 = ec.new_point(14, 11).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p4, p4).expect("error for possible addition"), p5);

        let f = GalloisField::new(11, true, None);
        let ec = ElipticCurve::new(f, 1, 1, true).expect("ec cant be created");
        let p1 = ec.new_point(3, 3).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(6, 5).expect("point is on ec but an error occurs");
        let p3 = ec.new_point(0, 10).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p2).expect("error for possible addition"), p3);

        let f = GalloisField::new(19, true, None);
        let ec = ElipticCurve::new(f, 7, 13, true).expect("ec cant be created");
        let p1 = ec.new_point(2, 15).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(6, 10).expect("point is on ec but an error occurs");
        let p3 = ec.new_point(9, 8).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p2).expect("error for possible addition"), p3);
        let ec = ElipticCurve::new(f, 10, 3, true).expect("ec cant be created");
        let p1 = ec.new_point(5, 11).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(5, 8).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p2).expect("error for possible addition"), ec.INFINITY_POINT);
        let ec = ElipticCurve::new(f, 7, 13, true).expect("ec cant be created");
        let p1 = ec.new_point(7, 5).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(2, 15).expect("point is on ec but an error occurs");
        assert_eq!(ec.add(p1, p1).expect("error for possible addition"), p2);
    }

    #[test]
    fn test_mul_points() {
        // from ecc lectures
        let f = GalloisField::new(13, true, None);
        let ec = ElipticCurve::new(f, 7, 11, true).expect("ec cant be created");
        let p1 = ec.new_point(4, 5).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(6, 10).expect("point is on ec but an error occurs");
        let p3 = ec.new_point(4, 8).expect("point is on ec but an error occurs");
        let p4 = ec.new_point(6, 3).expect("point is on ec but an error occurs");
        assert_eq!(ec.mul(p1, 2u32).expect("error for possible addition"), p2);
        assert_eq!(ec.mul(p1, 4u32).expect("error for possible addition"), p3);
        assert_eq!(ec.mul(p3, 2u32).expect("error for possible addition"), p4);
        assert_eq!(ec.mul(p2, 4u32).expect("error for possible addition"), p4);

        let f = GalloisField::new(13, true, None);
        let ec = ElipticCurve::new(f, -3, 3, true).expect("ec cant be created");
        let p1 = ec.new_point(1, 1).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(11, 12).expect("point is on ec but an error occurs");
        assert_eq!(ec.mul(p1, 2u64).expect("error for possible addition"), p2);

        let f = GalloisField::new(17, true, None);
        let ec = ElipticCurve::new(f, 11, 3, true).expect("ec cant be created");
        let p1 = ec.new_point(5, 8).expect("point is on ec but an error occurs");
        let p2 = ec.new_point(6, 8).expect("point is on ec but an error occurs");
        assert_eq!(ec.mul(p1, 10u128).expect("error for possible addition"), p2);
    }
}
