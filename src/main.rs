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
//! License:    MIT
//! Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
mod binary;
mod math;
mod algo;
mod cplex;

use cplex::cli::*;

use std::str::FromStr;

use clap::Parser;
use num_bigint;

/*************************************************************************************************/
/// main function of plexcryptool.
///
/// This function is the entrypoint of the binary. It parses Commandline options and calls the
/// internal functions with the corresponding values, then shows the results to the user.
pub fn main() {
    let args = Cli::parse();
    match args.clone().command {
        Commands::Version => {
            cplex::printing::version();
        }
        Commands::Math(action) => {
            match action.action {
                MathActions::Modexp(mod_exp_args) => {
                    let b = num_bigint::BigInt::from_str(&mod_exp_args.base.as_str()).expect("could not make bigint");
                    let e = num_bigint::BigInt::from_str(&mod_exp_args.exp.as_str()).expect("could not make bigint");
                    let f = num_bigint::BigInt::from_str(&mod_exp_args.field.as_str()).expect("could not make bigint");
                    let num = math::modexp::modular_exponentiation(b.clone(), e, f, args.verbose);
                    cplex::printing::proc_num(num, args);
                }
                MathActions::Modred(mod_red_args) => {
                    let result = math::modred::modred(mod_red_args.polynomial, mod_red_args.relation, args.verbose);
                    cplex::printing::proc_result(result, args);
                }
                MathActions::Pm1(pm1_args) => {
                    let vec: Result<Vec<u128>, String> = math::pm1::p_minus_one(
                        pm1_args.n, 
                        pm1_args.max_prime,
                        args.verbose
                        );
                    cplex::printing::proc_result_vec(vec, args);
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
                    cplex::printing::proc_num(result, args);
                },
                BinaryActions::Xor(bin_xor_args) => {
                    let result: u128 = binary::xor(bin_xor_args.a, bin_xor_args.b);
                    cplex::printing::proc_num(result, args);
                }
            }
        }
        Commands::Algo(action) => {
            match action.action {
                AlgoActions::Feistel0Inner(alg_fei_args) => {
                    let result: u16 = algo::feistel0::inner(alg_fei_args.input, alg_fei_args.key, args.verbose);
                    cplex::printing::proc_num(result, args);
                }
                AlgoActions::Feistel0SBOX(alg_fsb_args) => {
                    let result: u8 = algo::feistel0::sbox(alg_fsb_args.index);
                    cplex::printing::proc_num(result, args);
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
                    cplex::printing::proc_num(result, args);
                }
            }
        }
    }
}
