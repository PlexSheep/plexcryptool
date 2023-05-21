#![allow(dead_code)]

/// common functionality for printing
///
/// Implements code that might be used by many other modules
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::cplex::cli::Cli;

use std::fmt::{Debug, LowerHex, Display};

use pyo3::prelude::*;

use clap::CommandFactory;
use num::Integer;

///////////////////////////////////////////////////////////////////////////////////////////////////
// "constant" printing ////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

#[pyfunction]
/// Print version
pub fn version() {
    let b = <Box<Cli> as CommandFactory>::command();
    println!("{} {}", b.get_name(), b.get_version().unwrap());
    return;
}

#[pyfunction]
/// Print about
pub fn about() {
    let b = <Box<Cli> as CommandFactory>::command();
    println!("{}", b.get_about().unwrap());
    return;
}

#[pyfunction]
/// print a seperator
pub fn seperator() {
    println!("{:=<120}", '=');
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// result printing ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

/// process a result with some int
pub fn proc_result_num<T, K>(result: Result<T, K>, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex,
    T: Display,
    K: Debug
{
    if args.verbose {
        seperator();
    }
    match result {
        Ok(num) => {
            proc_num(num, args);
        }
        Err(e) => {
            proc_err(e, args);
        }
    }
}

/// process some int
pub fn proc_num<T>(num: T, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex,
    T: Display,
{
    if args.verbose {
        seperator();
    }
    if args.machine {
        println!("{} ({:#x})", num, num);
    }
    else {
        println!("result is {} ({:#x})", num, num);
    }
}

/// process some int tuple
pub fn proc_result_tup_num<T, K>(result: Result<(T, T), K>, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex,
    T: Display,
    K: Debug
{
    if args.verbose {
        seperator();
    }
    match result {
        Ok(tup) => {
            proc_tup_num(tup, args);
        }
        Err(e) => {
            proc_err(e, args);
        }
    }
}

/// process some int tuple result
pub fn proc_tup_num<T>(num: (T, T), args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex,
    T: Display,
{
    if args.verbose {
        seperator();
    }
    if args.machine {
        println!("({}{}) (({:#x}, {:#x})", num.0, num.1, num.0, num.1);
    }
    else {
        println!("result is ({}{}) (({:#x}, {:#x})", num.0, num.1, num.0, num.1);
    }
}

/// process some vec
pub fn proc_vec<T>(vec: Vec<T>, args: Cli)
    where
    T: Debug,
{
    if args.verbose {
        seperator();
    }
    if args.machine {
        println!("{:#?}", vec);
    }
    else {
        println!("result is\n{:#?}", vec);
    }
}

/// process a result with some vec
pub fn proc_result_vec<T, K>(res: Result<Vec<T>, K>, args: Cli)
    where
    T: Debug,
    K: Debug
{
    if args.verbose {
        seperator();
    }
    match res {
        Ok(vec) => {
            proc_vec(vec, args);
        }
        Err(e) => {
            proc_err(e, args);
        }
    }
}

/// process some error
pub fn proc_err<T>(e: T, args: Cli)
    where
    T: Debug
{
    if args.machine {
        println!("{:#?}", e)
    }
    else {
        println!("could not compute:\n{:#?}", e)
    }
}
