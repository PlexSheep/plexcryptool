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

///////////////////////////////////////////////////////////////////////////////////////////////////
/// This is just structures for parsing Cli args
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about)] // Read from `Cargo.toml`
#[command(
    help_template = "{about-section}\n\t{name} {version}\n\tAuthor: {author-with-newline}\n{usage-heading} {usage} \n {all-args} {tab}"
)]
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
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub enum Commands {
    /// Use math functions
    Math(MathCommand),
    /// Use binary functions
    Binary(BinaryCommand),
    /// Use custom algorithms
    Algo(AlgoCommand),
    /// Print version
    Version,
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
    /// perform exponentiation with a constant modulo applied
    Modexp(ModexpArgs),
    /// perform modular reduction
    Modred(ModredArgs),
    /// p minus 1 prime test
    Pm1(PM1Args),
    /// calculate in a gallois field
    /// includes Eliptic curves
    Gallois(GalloisAction),
    /// Euklidian Algorithm
    Gcd(GcdArgs),
    /// factorize a natural number
    Factorize(FactorizeArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ModexpArgs {
    pub base: String,
    pub exp: String,
    pub field: String
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ModredArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub polynomial: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub relation: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct PM1Args {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub n: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub max_prime: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GalloisAction {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub field: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub relation: Option<u128>,
    #[command(subcommand)]
    pub action: GalloisActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GcdArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    /// first number
    pub a: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    /// second number
    pub b: u128,
    #[arg(short, long, default_value_t = false)]
    /// use extended gcd
    pub ext: bool
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct FactorizeArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub n: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum GalloisActions {
    /// draw the root of n
    Sqrt(GalloisSqrtArgs),
    /// reduce n to the range of the field
    Reduce(GalloisReduceArgs),
    /// calculate the (multiplicative) inverse of n
    Inverse(GalloisInverseArgs),
    /// eliptic curves
    ECC(ECCAction)
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GalloisSqrtArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub a: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GalloisReduceArgs {
    pub n: i128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GalloisInverseArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub n: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ECCAction {
    #[clap(allow_hyphen_values=true)]   // allow negative inputs like -19
    pub a: i128,
    #[clap(allow_hyphen_values=true)]   // allow negative inputs like -19
    pub b: i128,
    #[command(subcommand)]
    pub action: ECCActions
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum ECCActions {
    /// negate a point
    Neg(ECCNegArgs),
    /// add a twp poimts
    Add(ECCAddArgs),
    /// multiply a point with an integer
    /// uses double and add
    Mul(ECCMulArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ECCNegArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub r: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub s: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ECCMulArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub r: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub s: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub n: u128,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ECCAddArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    pub r1: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub s1: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub r2: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    pub s2: u128,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum BinaryActions {
    /// bit rotation/circular shifting (only 32bit)
    #[command(name="rotate")]
    Rotate(RotateArgs),
    /// regular binary xor
    Xor(XorArgs),
    /// use a pbox
    Pbox(PboxArgs),
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

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct PboxArgs {
    #[clap(value_parser=maybe_hex::<u8>)]
    pub n: u8,
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
