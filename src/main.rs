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
                    cplex::printing::proc_result_num(result, args);
                }
                MathActions::Pm1(pm1_args) => {
                    let vec: Result<Vec<u128>, String> = math::pm1::p_minus_one(
                        pm1_args.n, 
                        pm1_args.max_prime,
                        args.verbose
                        );
                    cplex::printing::proc_result_vec(vec, args);
                }
                MathActions::Gallois(gal_args) => {
                    let field = math::gallois::GalloisField::new(gal_args.field, args.verbose, gal_args.relation);
                    match gal_args.action {
                        GalloisActions::Sqrt(gal_sqrt_args) => {
                            let result = field.sqrt(gal_sqrt_args.a);
                            cplex::printing::proc_result_tup_num(result, args);
                        }
                        GalloisActions::Reduce(gal_red_args) => {
                            let result = field.reduce::<_, u128>(gal_red_args.n);
                            cplex::printing::proc_num(result, args);
                        }
                        GalloisActions::Inverse(gal_inv_args) => {
                            let result = field.inverse(gal_inv_args.n);
                            cplex::printing::proc_result_num(result, args);
                        }
                        GalloisActions::ECC(ecc_args) => {
                            let ec = math::ecc::ElipticCurve::new(field, ecc_args.a, ecc_args.b, args.verbose).expect("Could not create eliptic curve");
                            match ecc_args.action {
                                ECCActions::Neg(ecc_neg_args) => {
                                    let p = ec.new_point(ecc_neg_args.r, ecc_neg_args.s);
                                    match p {
                                        Ok(p) => {
                                            let item = ec.neg(p);
                                            cplex::printing::proc_display(item, args)
                                        }
                                        Err(e) => {
                                            cplex::printing::proc_err(e, args);
                                        }
                                    }
                                }
                                ECCActions::Mul(ecc_mul_args) => {
                                    let p = ec.new_point(ecc_mul_args.r, ecc_mul_args.s);
                                    if p.is_err() {
                                        cplex::printing::proc_err(p, args);
                                    }
                                    else {
                                        let item = ec.mul(p.unwrap(), ecc_mul_args.n);
                                        if item.is_err() {
                                            cplex::printing::proc_err(item.unwrap_err(), args)
                                        }
                                        else {
                                            cplex::printing::proc_display(item.unwrap(), args);
                                        }
                                    }
                                }
                                ECCActions::Add(ecc_add_args) => {
                                    let p1 = ec.new_point(ecc_add_args.r1, ecc_add_args.s1);
                                    let p2 = ec.new_point(ecc_add_args.r2, ecc_add_args.s2);
                                    if p1.is_err() || p2.is_err() {
                                        cplex::printing::proc_err(p1, args.clone());
                                        cplex::printing::proc_err(p2, args);
                                    }
                                    else {
                                        let item = ec.add(p1.unwrap(), p2.unwrap());
                                        if item.is_err() {
                                            cplex::printing::proc_err(item.unwrap_err(), args)
                                        }
                                        else {
                                            cplex::printing::proc_display(item.unwrap(), args);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                MathActions::Factorize(fac_args) => {
                    let vec = math::factorise::prime_factors(fac_args.n, args.verbose);
                    cplex::printing::proc_vec(vec, args);
                }
                MathActions::Gcd(gcd_args) => {
                    if gcd_args.ext {
                        let vec = math::gcd::egcd(gcd_args.a, gcd_args.b);
                        cplex::printing::proc_vec(vec, args)
                    }
                    else {
                        let num = math::gcd::gcd(gcd_args.a, gcd_args.b);
                        cplex::printing::proc_num(num, args)
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
                    cplex::printing::proc_num(result, args);
                },
                BinaryActions::Xor(bin_xor_args) => {
                    let result: u128 = binary::xor(bin_xor_args.a, bin_xor_args.b);
                    cplex::printing::proc_num(result, args);
                }
                BinaryActions::Pbox(pbox_args) => {
                    let result: u8 = binary::pbox6::pbox6(pbox_args.n);
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
