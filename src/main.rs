use log::debug;

use std::{path::PathBuf, fs::File, io::{BufReader, stdout}};

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf
}

impl Cli {
    pub fn get_path_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> { // 異常時にはErrorトレイトを実装したエラーを返す: Result<T, E = Error>
    env_logger::init(); // env RUST_LOG=debug cargo run -- ... のようにログレベルを設定可能

    let args = Cli::parse();
    debug!("{:?}", args); // {:?} でフォーマットさせるために #[derive(Debug)] が必要
    let path = args.get_path_string();

    let file = File::open(args.path)
        .with_context(|| format!("could not read file: '{}'", path))?; // 異常時にトレース(元のError内容)付きのErrorを返す
    let file_buffer = BufReader::new(file);

    // モジュールを利用
    grrs::find_matches(file_buffer, &args.pattern, &mut stdout())
}
