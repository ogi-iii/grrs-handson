use std::{error::Error, process::Command};

use assert_cmd::prelude::*;
use assert_fs::{NamedTempFile, prelude::FileWriteStr};
use predicates::str::contains;

// 結合テストを実施
#[test]
fn file_does_not_exist() -> Result<(), Box<dyn Error>> {
    // Rustアプリを引数付きで実行し、想定どおり失敗することを確認
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(contains("could not read file:"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    // 検証用の一時ファイルを用意
    let file_name = "temp-file.txt";
    let file = NamedTempFile::new(file_name)?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path()); // 一時ファイルの作成先パスを取得して渡す
    cmd.assert()
        .success()
        .stdout(contains("A test\nAnother test"));
    Ok(())
}

#[test]
fn filepath_does_not_set() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar"); // 引数が不足したまま実行される
    cmd.assert()
        .failure();
    Ok(())
}
