use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// ゲームの設定値
pub mod config {
    /// 予想可能な数字の最小値
    pub const MIN_NUMBER: u32 = 1;
    /// 予想可能な数字の最大値
    pub const MAX_NUMBER: u32 = 100;
}

/// ゲームで使用するメッセージ（大阪弁）
mod messages {
    pub const GAME_START: &str = "1から100の数字を当ててみぃや！";
    pub const PROMPT: &str = "ほな、予想入れてみて！";
    pub const TOO_SMALL: &str = "もっと大きいで！";
    pub const TOO_BIG: &str = "もっと小さいで！";
    pub const CORRECT: &str = "正解や！やったな！";
    pub const INVALID_INPUT: &str = "ちゃんとした数字入れてや！";
}

/// ゲームのメインロジック（引数なし版）
/// 標準入出力を使用するシンプルなAPI
///
/// # Errors
/// 入出力操作が失敗した場合に `io::Error` を返す
pub fn run_game() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_game_internal(&mut stdin.lock(), &mut stdout.lock())
}

/// ジェネリック入出力を使用するゲームロジック（テスト用に公開）
///
/// # Errors
/// 入出力操作が失敗した場合に `io::Error` を返す
pub fn run_game_internal<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<()> {
    let secret_number = rand::rng().random_range(config::MIN_NUMBER..=config::MAX_NUMBER);
    run_game_with_secret(reader, writer, secret_number)
}

/// 秘密の数字を指定してゲームを実行する（テスト用に公開）
///
/// # Errors
/// 入出力操作が失敗した場合に `io::Error` を返す
pub fn run_game_with_secret<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    secret_number: u32,
) -> io::Result<()> {
    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] 秘密の数字は {secret_number} やで！");

    writeln!(writer, "{}", messages::GAME_START)?;
    writeln!(writer, "{}", messages::PROMPT)?;

    let mut guess = String::new();

    loop {
        guess.clear();
        let bytes_read = reader.read_line(&mut guess)?;
        if bytes_read == 0 {
            break; // EOF
        }

        match parse_guess(&guess) {
            Some(num) => {
                writeln!(writer, "あんたの予想は {num} やな！")?;
                match num.cmp(&secret_number) {
                    Ordering::Less => writeln!(writer, "{}", messages::TOO_SMALL)?,
                    Ordering::Greater => writeln!(writer, "{}", messages::TOO_BIG)?,
                    Ordering::Equal => {
                        writeln!(writer, "{}", messages::CORRECT)?;
                        break;
                    }
                }
            }
            None => writeln!(writer, "{}", messages::INVALID_INPUT)?,
        }
    }

    Ok(())
}

/// 入力文字列を数値に変換する
/// 1〜100の範囲内の有効な数値の場合は `Some(u32)` を返し、それ以外は `None` を返す
#[must_use]
pub fn parse_guess(input: &str) -> Option<u32> {
    input
        .trim()
        .parse()
        .ok()
        .filter(|&n| (config::MIN_NUMBER..=config::MAX_NUMBER).contains(&n))
}
