mod binary;
mod modular_exponentiation;

use std::{str::FromStr, fmt::Debug};

use modular_exponentiation::modular_exponentiation;

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
pub fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Math(action) => {
            match action.action {
                MathActions::Modexp(mod_exp_args) => {
                    let b = num_bigint::BigInt::from_str(&mod_exp_args.base.as_str()).expect("could not make bigint");
                    let e = num_bigint::BigInt::from_str(&mod_exp_args.exp.as_str()).expect("could not make bigint");
                    let f = num_bigint::BigInt::from_str(&mod_exp_args.field.as_str()).expect("could not make bigint");
                    let result = modular_exponentiation(b.clone(), e, f);
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

        // Fallback, this should ideally not execute
        _ => {
            eprintln!("Command not implemented.\n");
        }
    }
}
