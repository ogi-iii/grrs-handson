use std::{io::{BufRead, Write}, borrow::Cow};

use anyhow::Result;

pub mod module; // インポートしたモジュールを公開

use module::parser::Cli;

impl Cli {
    pub fn get_path_string(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
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
