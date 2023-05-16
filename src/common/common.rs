#![allow(dead_code)]

/// common functionality
///
/// Implements code that might be used by many other modules
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use crate::Cli;

use std::fmt::{Debug, LowerHex};

use num::Integer;

/// process a result with some int
pub fn proc_result<T>(result: Result<T, String>, args: Cli)
    where
    T: Debug,
    T: Integer,
    T: LowerHex
{
    match result {
        Ok(res) => {
            if args.machine {
                println!("{:#x}", res);
            }
            else {
                println!("=======================================================================");
                println!("result is {:#x}", res);
            }
        }
        Err(e) => {
            if args.machine {
                println!("{:#?}", e)
            }
            else {
                println!("=======================================================================");
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
    if args.machine {
        println!("{:#x}", num);
    }
    else {
        println!("=======================================================================");
        println!("result is {:#x}", num);
    }
}

/// process some vec
pub fn proc_vec<T>(vec: Vec<T>, args: Cli)
    where
    T: Debug,
{
    if args.machine {
        println!("{:#?}", vec);
    }
    else {
        println!("=======================================================================");
        println!("result is\n{:#?}", vec);
    }
}

/// process a result with some vec
pub fn proc_result_vec<T>(res: Result<Vec<T>, String>, args: Cli)
    where
    T: Debug,
{
    match res {
        Ok(vec) => {
            if args.machine {
                println!("{:#?}", vec);
            }
            else {
                println!("=======================================================================");
                println!("result is {:#?}", vec);
            }
        }
        Err(e) => {
            if args.machine {
                println!("{:#?}", e)
            }
            else {
                println!("=======================================================================");
                println!("could not compute:\n{:#?}", e)
            }
        }
    }
}
