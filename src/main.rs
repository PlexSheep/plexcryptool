mod binary;
mod modular_exponentiation;

use std::{str::{FromStr, Matches}, fmt::Debug};

use modular_exponentiation::modular_exponentiation;

use clap::{Args, Parser, Subcommand, ValueEnum};
use num_bigint;


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
    Binary
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct MathCommand {
    #[command(subcommand)]
    action: MathActions
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum MathActions {
    #[command(name="modexp")]
    ModExp(ModExpArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
struct ModExpArgs {
    Base: String,
    Exp: String,
    Field: String
}

pub fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Math(action) => {
            match action.action {
                MathActions::ModExp(mod_exp_args) => {
                    let b = num_bigint::BigInt::from_str(&mod_exp_args.Base.as_str()).expect("a");
                    let e = num_bigint::BigInt::from_str(&mod_exp_args.Exp.as_str()).expect("a");
                    let f = num_bigint::BigInt::from_str(&mod_exp_args.Field.as_str()).expect("a");
                    let r = modular_exponentiation(b.clone(), e, f);
                    if args.machine {
                        print!("{}\n", r)
                    }
                    else {
                        print!("res is {}\n", r)
                    }
                }
            }
        }
        Commands::Binary => {
            
        }


        _ => {
            eprintln!("Command not implemented.\n");
        }
    }
}

