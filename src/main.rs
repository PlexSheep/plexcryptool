#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//!
//! This is a mixed rust/python library that also offers an executable.
//! The intended usage is the solving of tasks for cryptology and maybe math, in the context of a
//! # various tools for use in cryptology contexts
//! university degree. I wrote this for cryptology at DHBW Mannheim.
//!
//! ___
//! Author:     Christoph J. Scherr <software@cscherr.de>
//!
//! License:    MIT
//!
//! Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
mod binary;
mod math;

use std::{str::FromStr, fmt::Debug};

use clap::{Args, Parser, Subcommand};
use num_bigint;
/*************************************************************************************************/
// This is just structures for parsing Cli args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Which submodule to use
    #[command(subcommand)]
    command: Commands,

    /// Machine output
    #[arg(short, long, default_value_t = false, global = true)]
    machine: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Use math functions
    Math(MathCommand),
    /// Use binary functions
    Binary(BinaryCommand)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct MathCommand {
    #[command(subcommand)]
    action: MathActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct BinaryCommand {
    #[command(subcommand)]
    action: BinaryActions
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum MathActions {
    #[command(name="modexp")]
    Modexp(ModexpArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct ModexpArgs {
    base: String,
    exp: String,
    field: String
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum BinaryActions {
    /// bit rotation/circular shifting (only 32bit)
    #[command(name="rotate")]
    Rotate(RotateArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct RotateArgs {
    #[arg(short, long, default_value_t = false)]
    left: bool,
    base: u32,
    shift_width: u32,
}

/*************************************************************************************************/
/// main function of plexcryptool.
///
/// This function is the entrypoint of the binary. It parses Commandline options and calls the
/// internal functions with the corresponding values, then shows the results to the user.
pub fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Math(action) => {
            match action.action {
                MathActions::Modexp(mod_exp_args) => {
                    let b = num_bigint::BigInt::from_str(&mod_exp_args.base.as_str()).expect("could not make bigint");
                    let e = num_bigint::BigInt::from_str(&mod_exp_args.exp.as_str()).expect("could not make bigint");
                    let f = num_bigint::BigInt::from_str(&mod_exp_args.field.as_str()).expect("could not make bigint");
                    let result = math::modexp::modular_exponentiation(b.clone(), e, f, args.verbose);
                    if args.machine {
                        println!("{}", result)
                    }
                    else {
                        println!("result is {}", result)
                    }
                }
            }
        }
        Commands::Binary(action) => {
            match action.action {
                BinaryActions::Rotate(bin_rot_args) => {
                    let result: u32;
                    if bin_rot_args.left {
                        result = binary::rotl32(bin_rot_args.base, bin_rot_args.shift_width);
                    }
                    else {
                        result = binary::rotr32(bin_rot_args.base, bin_rot_args.shift_width);
                    }
                    if args.machine {
                        println!("{}", result)
                    }
                    else {
                        println!("result is {}", result)
                    }
                }
            }
            
        }
    }
}
