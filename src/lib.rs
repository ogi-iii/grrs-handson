use std::{io::{BufRead, Write}, path::PathBuf, borrow::Cow};

use anyhow::Result;
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

impl Cli {
    pub fn get_path_string(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli { pattern: String::new(), path: PathBuf::new() }
    }
}

// モジュールとして分離
pub fn find_matches(buffered_reader: impl BufRead, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in buffered_reader.lines() {
        let line = line?; // 異常時はErrorが返される: .unwrap()だとpanic!が返されるため
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?; // writerにlineを書き込む: エラーハンドリングとして ? も必要
        }
    }
    Ok(())
}

#[test] // テストコードを記載
fn find_a_match() {
    let mut result = vec![]; // writerはバイトデータを書き込む
    find_matches("lorem ipsum\ndolor sit amet".as_bytes(), "lorem", &mut result).unwrap(); // BufReadは可変長のバイトデータが対象
    assert_eq!(result, b"lorem ipsum\n"); // 比較対象を左右逆にはできない: 可変長のベクトルを先に渡す必要がある(固定長の文字列を先に渡すと比較対象も固定長でなければならないため)
}
