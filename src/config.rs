use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
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
}
