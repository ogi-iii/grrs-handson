// 依存関係はCargo.toml内の[build-dependencies]に記載

use std::{fs::{self, create_dir_all}, path::Path, error::Error};

use clap::CommandFactory;

#[path ="src/module/parser.rs"]
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = parser::Cli::command();
    let man = clap_mangen::Man::new(cmd);

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?; // man contents -> buffer -> file

    let output_dir = Path::new("target/man");
    create_dir_all(output_dir)?;

    // コマンド名.1: セクション1のユーザーコマンドを定義 (一般的なコマンドのマニュアル)
    fs::write(output_dir.join("grrs.1"), buffer)?;
    Ok(())
}
