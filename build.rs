use std::{fs::{self, create_dir_all}, path::Path, error::Error};

use clap::CommandFactory;

#[path ="src/lib.rs"]
mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = lib::Cli::command();
    let man = clap_mangen::Man::new(cmd);

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let output_dir = Path::new("target/man");
    create_dir_all(output_dir)?;

    // コマンド名.1: セクション1のユーザーコマンドを定義 (一般的なコマンドのマニュアル)
    fs::write(output_dir.join("grrs.1"), buffer)?;
    Ok(())
}
