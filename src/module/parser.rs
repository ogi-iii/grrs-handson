use std::path::PathBuf;

use clap::Parser;
use serde_derive::{Serialize, Deserialize};

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Debug, Parser, Serialize, Deserialize)]
#[command(arg_required_else_help = true)] // 必須引数の未指定時にヘルプを出力
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read, use - to read from stdin (must not be a tty)
    pub path: PathBuf
}

impl Default for Cli {
    fn default() -> Self {
        Cli { pattern: String::new(), path: PathBuf::new() }
    }
}
