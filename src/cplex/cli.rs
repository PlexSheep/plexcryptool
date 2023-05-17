/// command line options
///
/// this module contains structs and enums that are used to parse command line arguments.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
///
use clap::{Args, Parser, Subcommand};
use clap_num::maybe_hex;

/// This is just structures for parsing Cli args
#[derive(Parser, Debug, Clone)]
#[clap(name="plexcryptool", author="Christoph J. Scherr", version, about="Various tools for use with math and cryptology, includes executable and a library.")]
pub struct Cli {
    /// Which submodule to use
    #[command(subcommand)]
    pub command: Commands,

    /// Machine output
    #[arg(short, long, default_value_t = false, global = true)]
    pub machine: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Use math functions
    Math(MathCommand),
    /// Use binary functions
    Binary(BinaryCommand),
    /// Use custom algorithms
    Algo(AlgoCommand),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct MathCommand {
    #[command(subcommand)]
    pub action: MathActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct BinaryCommand {
    #[command(subcommand)]
    pub action: BinaryActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct AlgoCommand {
    #[command(subcommand)]
    pub action: AlgoActions
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum MathActions {
    #[command(name="modexp")]
    Modexp(ModexpArgs),
    Modred(ModredArgs),
    Pm1(PM1Args),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ModexpArgs {
    pub base: String,
    pub exp: String,
    pub field: String
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ModredArgs {
    #[clap(value_parser=maybe_hex::<u64>)]
    pub polynomial: u64,
    #[clap(value_parser=maybe_hex::<u64>)]
    pub relation: u64,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct PM1Args {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub n: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub max_prime: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum BinaryActions {
    /// bit rotation/circular shifting (only 32bit)
    #[command(name="rotate")]
    Rotate(RotateArgs),
    Xor(XorArgs)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct RotateArgs {
    #[arg(short, long, default_value_t = false)]
    pub left: bool,
    #[clap(value_parser=maybe_hex::<u32>)]
    pub base: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    pub shift_width: u32,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct XorArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub a: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub b: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum AlgoActions {
    #[command(name="feistel0-i")]
    Feistel0Inner(Feistel0InnerArgs),
    #[command(name="feistel0-sbox")]
    Feistel0SBOX(Feistel0SBOXArgs),
    #[command(name="feistel0")]
    Feistel0(Feistel0Args)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct Feistel0InnerArgs {
    #[clap(value_parser=maybe_hex::<u16>)]
    pub input: u16,
    #[clap(value_parser=maybe_hex::<u16>)]
    pub key: u16,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct Feistel0SBOXArgs {
    #[clap(value_parser=maybe_hex::<u8>)]
    pub index: u8,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct Feistel0Args{
    #[clap(value_parser=maybe_hex::<u32>)]
    pub input: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    pub key: u32,
    #[arg(short, long, default_value_t = false)]
    pub decrypt: bool,
}