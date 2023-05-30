#![allow(dead_code)]
/// eliptic curve cryptography
///
/// This module implements structs and functionalities used for eliptic curve cryptography (ECC).
/// Do not expect it to actually be secure, I made this for cryptography lectures.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use super::gallois::GalloisField;

use pyo3::prelude::*;

/// This is a very special math point, it does not really exist but is useful.
pub const INFINITY_POINT: ElipticCurvePoint = ElipticCurvePoint {
    x: 0,
    y: 0,
    is_infinity_point: true,
    verbose: false
};

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
#[pyclass]
/// represent a specific eliptic curve
///
/// real curves not supported, only in Gallois Fields
pub struct ElipticCurve {
    f: GalloisField,
    a: i128,
    b: i128,
    points: Vec<ElipticCurvePoint>,
    verbose: bool,
    INFINITY_POINT: ElipticCurvePoint,
}

impl ElipticCurve {
    pub fn new(f: GalloisField, a: i128, b: i128, verbose: bool) -> Self {
        let e = ElipticCurve {
            f,
            a,
            b,
            points: Vec::new(),
            verbose,
            INFINITY_POINT
        };
        return e;
    }

    /// calculate a point for coordinates
    pub fn poly(&self, x: i128, y: i128) -> i128 {
        return y.pow(2) - x.pow(3) - (self.a * x) - self.b;
    }

    pub fn check_point(self, p: ElipticCurvePoint) -> bool {
        let mut valid = true;
        let r =  self.f.reduce(self.poly(p.x, p.y));
        if self.verbose {
            println!("F({}, {}) = {}² - {}³ - {} * {} - {} = {r}",
                p.x, p.y, p.y, p.x, self.a, p.x, self.b
            )
        }
        valid &= r == 0;
        return valid;
    }
}

#[test]
fn test_check_point() {
    let f = GalloisField::new(1151, true);
    let ec = ElipticCurve::new(f, 1, 679, true);
    // real points
    let p = vec![
        ElipticCurvePoint::new(298, 531),
        ElipticCurvePoint::new(600, 127),
        ElipticCurvePoint::new(846, 176),
    ];
    // random values, not part of the ec.
    let np = vec![
        ElipticCurvePoint::new(198, 331),
        ElipticCurvePoint::new(100, 927),
        ElipticCurvePoint::new(446, 876),
    ];
    for i in p {
        dbg!(&i);
        assert!(ec.clone().check_point(i));
    }
    for i in np {
        dbg!(&i);
        assert!(!ec.clone().check_point(i));
    }
}


#[derive(Debug, Clone, Copy)]
#[pyclass]
/// represent a specific eliptic curves point
pub struct ElipticCurvePoint {
    x: i128,
    y: i128,
    is_infinity_point: bool,
    verbose: bool
}

impl ElipticCurvePoint {
    pub fn new(x: i128, y: i128) -> Self {
        ElipticCurvePoint {
            x,
            y,
            is_infinity_point: false,
            verbose: false
        }
    }
    
    pub fn get_infinity_point() -> Self {
        return INFINITY_POINT;
    }

    /// add two points
    pub fn add(a: Self, b: Self) -> Self {
        // TODO
        panic!("TODO");
    }

    /// get negative of a point
    pub fn neg(p: Self) -> Self {
        // TODO
        panic!("TODO");
    }

    /// multiply a point by an integer
    pub fn mul(n: u128, a: Self) -> Self {
        // TODO
        panic!("TODO");
    }
}
