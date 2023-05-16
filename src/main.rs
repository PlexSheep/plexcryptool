#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//!
//! This is a mixed rust/python library that also offers an executable.
//! The intended usage is the solving of tasks for cryptology and maybe math, in the context of a
//! # various tools for use in cryptology contexts
//! university degree. I wrote this for cryptology at DHBW Mannheim.
//!
//! ## main function
//! This project contains an executable, see [main.rs](main.rs)
//!
//! ## lib module
//! This project contains is a library, see [lib.rs](lib.rs).
//! Note that this library offers Python bindings using [PyO3](pyo3.rs)
//! ___
//! Author:     Christoph J. Scherr <software@cscherr.de>
//!
//! License:    MIT
//!
//! Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
mod binary;
mod math;
mod algo;
mod common;

use std::{str::FromStr, fmt::Debug};

use clap::{Args, Parser, Subcommand};
use clap_num::maybe_hex;
use num_bigint;
/*************************************************************************************************/
// This is just structures for parsing Cli args
#[derive(Parser, Debug, Clone)]
#[clap(name="plexcryptool", author="Christoph J. Scherr", version, about="Various tools for use with math and cryptology, includes executable and a library.")]
pub struct Cli {
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

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Use math functions
    Math(MathCommand),
    /// Use binary functions
    Binary(BinaryCommand),
    /// Use custom algorithms
    Algo(AlgoCommand),
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

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct AlgoCommand {
    #[command(subcommand)]
    action: AlgoActions
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum MathActions {
    #[command(name="modexp")]
    Modexp(ModexpArgs),
    Modred(ModredArgs),
    Pm1(PM1Args),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct ModexpArgs {
    base: String,
    exp: String,
    field: String
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct ModredArgs {
    #[clap(value_parser=maybe_hex::<u64>)]
    polynomial: u64,
    #[clap(value_parser=maybe_hex::<u64>)]
    relation: u64,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct PM1Args {
    #[clap(value_parser=maybe_hex::<u128>)]
    n: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    max_prime: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum BinaryActions {
    /// bit rotation/circular shifting (only 32bit)
    #[command(name="rotate")]
    Rotate(RotateArgs),
    Xor(XorArgs)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct RotateArgs {
    #[arg(short, long, default_value_t = false)]
    left: bool,
    #[clap(value_parser=maybe_hex::<u32>)]
    base: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    shift_width: u32,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct XorArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    a: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    b: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum AlgoActions {
    #[command(name="feistel0-i")]
    Feistel0Inner(Feistel0InnerArgs),
    #[command(name="feistel0-sbox")]
    Feistel0SBOX(Feistel0SBOXArgs),
    #[command(name="feistel0")]
    Feistel0(Feistel0Args)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct Feistel0InnerArgs {
    #[clap(value_parser=maybe_hex::<u16>)]
    input: u16,
    #[clap(value_parser=maybe_hex::<u16>)]
    key: u16,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct Feistel0SBOXArgs {
    #[clap(value_parser=maybe_hex::<u8>)]
    index: u8,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct Feistel0Args{
    #[clap(value_parser=maybe_hex::<u32>)]
    input: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    key: u32,
    #[arg(short, long, default_value_t = false)]
    decrypt: bool,
}

/*************************************************************************************************/
/// main function of plexcryptool.
///
/// This function is the entrypoint of the binary. It parses Commandline options and calls the
/// internal functions with the corresponding values, then shows the results to the user.
pub fn main() {
    let args = Cli::parse();
    match args.clone().command {
        Commands::Math(action) => {
            match action.action {
                MathActions::Modexp(mod_exp_args) => {
                    let b = num_bigint::BigInt::from_str(&mod_exp_args.base.as_str()).expect("could not make bigint");
                    let e = num_bigint::BigInt::from_str(&mod_exp_args.exp.as_str()).expect("could not make bigint");
                    let f = num_bigint::BigInt::from_str(&mod_exp_args.field.as_str()).expect("could not make bigint");
                    let num = math::modexp::modular_exponentiation(b.clone(), e, f, args.verbose);
                    common::common::proc_num(num, args);
                }
                MathActions::Modred(mod_red_args) => {
                    let result = math::modred::modred(mod_red_args.polynomial, mod_red_args.relation, args.verbose);
                    common::common::proc_result(result, args);
                }
                MathActions::Pm1(pm1_args) => {
                    let vec: Result<Vec<u128>, String> = math::pm1::p_minus_one(
                        pm1_args.n, 
                        pm1_args.max_prime,
                        args.verbose
                        );
                    common::common::proc_result_vec(vec, args);
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
                    common::common::proc_num(result, args);
                },
                BinaryActions::Xor(bin_xor_args) => {
                    let result: u128 = binary::xor(bin_xor_args.a, bin_xor_args.b);
                    common::common::proc_num(result, args);
                }
            }
        }
        Commands::Algo(action) => {
            match action.action {
                AlgoActions::Feistel0Inner(alg_fei_args) => {
                    let result: u16 = algo::feistel0::inner(alg_fei_args.input, alg_fei_args.key, args.verbose);
                    common::common::proc_num(result, args);
                }
                AlgoActions::Feistel0SBOX(alg_fsb_args) => {
                    let result: u8 = algo::feistel0::sbox(alg_fsb_args.index);
                    common::common::proc_num(result, args);
                }
                AlgoActions::Feistel0(alg_fe0_args) => {
                    let keys = algo::feistel0::key_scheduler(alg_fe0_args.key);
                    let result: u32;
                    if alg_fe0_args.decrypt {
                        result = algo::feistel0::decrypt(alg_fe0_args.input, keys, args.verbose);
                    }
                    else {
                        result = algo::feistel0::encrypt(alg_fe0_args.input, keys, args.verbose);
                    }
                    common::common::proc_num(result, args);
                }
            }
        }
    }
}
