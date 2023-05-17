#![allow(dead_code)]

/// common functionality
///
/// Implements code that might be used by many other modules
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::cplex::cli::Cli;

use std::fmt::{Debug, LowerHex};

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
pub fn proc_result<T>(result: Result<T, String>, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex
{
    if args.verbose {
        seperator();
    }
    match result {
        Ok(res) => {
            if args.machine {
                println!("{:#x}", res);
            }
            else {
                seperator();
                println!("result is {:#x}", res);
            }
        }
        Err(e) => {
            if args.machine {
                println!("{:#?}", e)
            }
            else {
                seperator();
                println!("could not compute:\n{:#?}", e)
            }
        }
    }
}

/// process some int
pub fn proc_num<T>(num: T, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex
{
    if args.verbose {
        seperator();
    }
    if args.machine {
        println!("{:#x}", num);
    }
    else {
        seperator();
        println!("result is {:#x}", num);
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
        seperator();
        println!("result is\n{:#?}", vec);
    }
}

/// process a result with some vec
pub fn proc_result_vec<T>(res: Result<Vec<T>, String>, args: Cli)
    where
    T: Debug,
{
    if args.verbose {
        seperator();
    }
    match res {
        Ok(vec) => {
            if args.machine {
                println!("{:#?}", vec);
            }
            else {
                seperator();
                println!("result is {:#?}", vec);
            }
        }
        Err(e) => {
            if args.machine {
                println!("{:#?}", e)
            }
            else {
                seperator();
                println!("could not compute:\n{:#?}", e)
            }
        }
    }
}
