use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, disable_version_flag = true)]
pub struct Config {

    /// Language plugin to use from the 'plugins' directory (e.g., 'c', 'python').
    #[arg(short = 'l', long)]
    pub lang: String,

    /// Input file to convert.
    pub input_file: PathBuf,

    /// Output file path. Prints to stdout if not provided.
    #[arg(short, long)]
    pub output_file: Option<PathBuf>,

    /// The variable name for the array.
    #[arg(short = 'n', long, default_value = "binary_data")]
    pub array_name: String,

    /// Disable the default null terminator (0x00) at the end of the array.
    #[arg(long)]
    pub no_null: bool,

    /// Number of items per line in output arrays (default: 16)
    #[arg(long, default_value_t = 16)]
    pub line_length: usize,

    /// Array type for output (C, default: "const unsigned char")
    #[arg(long, default_value = "const unsigned char")]
    pub array_type: String,

    /// Array type for output (Python, default: "bytes")
    #[arg(long, default_value = "bytes")]
    pub python_type: String,

    /// Array type for output (Rust, default: "u8")
    #[arg(long, default_value = "u8")]
    pub rust_type: String,

    /// Indent size for output (default: 4)
    #[arg(long, default_value_t = 4)]
    pub indent: usize,
    /// Print verbose log
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Print version info and exit
    #[arg(long, default_value_t = false)]
    pub version: bool,
}
