#![allow(dead_code)]
/// eliptic curve cryptography
///
/// This module implements structs and functionalities used for ECC.
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
}


#[derive(Debug, Clone)]
#[pyclass]
/// represent a specific eliptic curves point
pub struct ElipticCurvePoint {
    x: i128,
    y: i128,
    is_infinity_point: bool,
    verbose: bool
}

impl ElipticCurvePoint {
    pub fn new(x: i128, y: i128, verbose: bool) -> Self {
        ElipticCurvePoint {
            x,
            y,
            is_infinity_point: false,
            verbose
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

    /// multiply a point by an integer
    pub fn mul(n: u128, a: Self) -> Self {
        // TODO
        panic!("TODO");
    }
}
