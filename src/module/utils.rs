use std::borrow::Cow;

use super::parser::Cli; // 同一ディレクトリ内の別ファイルをインポート

impl Cli {
    pub fn get_path_string(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }
}
