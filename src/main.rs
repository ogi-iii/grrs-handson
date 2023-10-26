use log::debug;
use serde_derive::{Serialize, Deserialize};

use std::{path::PathBuf, fs::File, io::{BufReader, stdout, stdin, IsTerminal, ErrorKind}, borrow::Cow};

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Debug, Parser, Serialize, Deserialize)]
#[command(arg_required_else_help = true)] // 必須引数の未指定時にヘルプを出力
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read, use - to read from stdin (must not be a tty)
    path: PathBuf
}

impl Cli {
    fn get_path_string(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli { pattern: String::new(), path: PathBuf::new() }
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> { // 異常時にはErrorトレイトを実装したエラーを返す: Result<T, E = Error>
    env_logger::init(); // env RUST_LOG=debug cargo run -- ... のようにログレベルを設定可能

    // 設定ファイルの読み込み: 見つからない時はデフォルト値がセットされる
    let cfg: Cli = confy::load("grrs", "cfg-name")?;
    debug!("config: {:?}", cfg);
    // // 設定ファイルへの書き込み
    // confy::store("grrs", "cfg-name", cfg)?;

    // コマンドライン引数から構造体を生成
    let args = Cli::parse();
    debug!("args: {:?}", args); // {:?} でフォーマットさせるために #[derive(Debug)] が必要

    let path = args.get_path_string();
    if path.eq("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap(); // clap::CommandFactory が必要
            return Err(anyhow::Error::from(std::io::Error::from(ErrorKind::InvalidInput)));
        }
        // 標準入力から文字列を探索する
        return grrs::find_matches(stdin().lock(), &args.pattern, &mut stdout());
    }

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file: '{}'", path))?; // 異常時にトレース(元のError内容)付きのErrorを返す
    let file_buffer = BufReader::new(file);

    // モジュールを利用: 引数でパスを指定されたファイルから文字列を探索する
    return grrs::find_matches(file_buffer, &args.pattern, &mut stdout())
}
