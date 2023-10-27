use log::debug;
use std::{fs::File, io::{BufReader, stdout, stdin, IsTerminal, ErrorKind}};
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};

use grrs::module::parser::Cli;
use grrs::module::matcher::find_matches;

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
        return find_matches(stdin().lock(), &args.pattern, &mut stdout());
    }

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file: '{}'", path))?; // 異常時にトレース(元のError内容)付きのErrorを返す
    let file_buffer = BufReader::new(file);

    // モジュールを利用: 引数でパスを指定されたファイルから文字列を探索する
    return find_matches(file_buffer, &args.pattern, &mut stdout())
}
