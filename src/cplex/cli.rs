/// This is just structures for parsing Cli args
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
    action: MathActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct BinaryCommand {
    #[command(subcommand)]
    action: BinaryActions
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct AlgoCommand {
    #[command(subcommand)]
    action: AlgoActions
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
    base: String,
    exp: String,
    field: String
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ModredArgs {
    #[clap(value_parser=maybe_hex::<u64>)]
    polynomial: u64,
    #[clap(value_parser=maybe_hex::<u64>)]
    relation: u64,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct PM1Args {
    #[clap(value_parser=maybe_hex::<u128>)]
    n: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    max_prime: u128,
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
    left: bool,
    #[clap(value_parser=maybe_hex::<u32>)]
    base: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    shift_width: u32,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct XorArgs {
    #[clap(value_parser=maybe_hex::<u128>)]
    a: u128,
    #[clap(value_parser=maybe_hex::<u128>)]
    b: u128,
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
    input: u16,
    #[clap(value_parser=maybe_hex::<u16>)]
    key: u16,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct Feistel0SBOXArgs {
    #[clap(value_parser=maybe_hex::<u8>)]
    index: u8,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct Feistel0Args{
    #[clap(value_parser=maybe_hex::<u32>)]
    input: u32,
    #[clap(value_parser=maybe_hex::<u32>)]
    key: u32,
    #[arg(short, long, default_value_t = false)]
    decrypt: bool,
}
