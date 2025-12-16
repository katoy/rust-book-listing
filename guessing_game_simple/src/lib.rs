//! # Guessing Game Simple
//!
//! 数当てゲームの簡易版ライブラリです。
//! 標準入出力を直接使用するシンプルな実装です。
//!
//! ## Example
//!
//! ```no_run
//! if let Err(e) = guessing_game_simple::run_game() {
//!     eprintln!("Error: {e}");
//! }
//! ```

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
///
/// 標準入出力を直接使用する簡易版です。
/// 1〜100のランダムな数字を生成し、ユーザーに当てさせます。
///
/// # Errors
///
/// 入出力操作が失敗した場合に `io::Error` を返します。
pub fn run_game() -> io::Result<()> {
    let secret_number = rand::rng().random_range(config::MIN_NUMBER..=config::MAX_NUMBER);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

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
                let hint = get_hint(num, secret_number);
                writeln!(writer, "{}", format_hint_message(hint))?;
                if hint == Ordering::Equal {
                    break;
                }
            }
            None => writeln!(writer, "{}", messages::INVALID_INPUT)?,
        }
    }

    Ok(())
}

/// 入力文字列を数値に変換する
///
/// 1〜100の範囲内の有効な数値の場合は `Some(u32)` を返し、
/// それ以外は `None` を返します。
///
/// # Examples
///
/// ```
/// use guessing_game_simple::parse_guess;
///
/// assert_eq!(parse_guess("42"), Some(42));
/// assert_eq!(parse_guess("  50  \n"), Some(50));
/// assert_eq!(parse_guess("abc"), None);
/// assert_eq!(parse_guess("0"), None);
/// assert_eq!(parse_guess("101"), None);
/// ```
#[must_use]
pub fn parse_guess(input: &str) -> Option<u32> {
    input
        .trim()
        .parse()
        .ok()
        .filter(|&n| (config::MIN_NUMBER..=config::MAX_NUMBER).contains(&n))
}

/// 予想と秘密の数字を比較してヒントを返す
///
/// # Examples
///
/// ```
/// use guessing_game_simple::get_hint;
/// use std::cmp::Ordering;
///
/// assert_eq!(get_hint(25, 50), Ordering::Less);
/// assert_eq!(get_hint(75, 50), Ordering::Greater);
/// assert_eq!(get_hint(50, 50), Ordering::Equal);
/// ```
#[must_use]
pub fn get_hint(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

/// Ordering に応じたヒントメッセージを返す
///
/// # Examples
///
/// ```
/// use guessing_game_simple::format_hint_message;
/// use std::cmp::Ordering;
///
/// assert_eq!(format_hint_message(Ordering::Less), "もっと大きいで！");
/// assert_eq!(format_hint_message(Ordering::Greater), "もっと小さいで！");
/// assert_eq!(format_hint_message(Ordering::Equal), "正解や！やったな！");
/// ```
#[must_use]
pub fn format_hint_message(ordering: Ordering) -> &'static str {
    match ordering {
        Ordering::Less => messages::TOO_SMALL,
        Ordering::Greater => messages::TOO_BIG,
        Ordering::Equal => messages::CORRECT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse_guess テスト
    #[test]
    fn test_parse_guess_valid() {
        assert_eq!(parse_guess("50"), Some(50));
        assert_eq!(parse_guess("  42  "), Some(42));
        assert_eq!(parse_guess("42\n"), Some(42));
        assert_eq!(parse_guess("\t42\t"), Some(42));
    }

    #[test]
    fn test_parse_guess_boundary() {
        assert_eq!(parse_guess("0"), None);
        assert_eq!(
            parse_guess(&config::MIN_NUMBER.to_string()),
            Some(config::MIN_NUMBER)
        );
        assert_eq!(
            parse_guess(&config::MAX_NUMBER.to_string()),
            Some(config::MAX_NUMBER)
        );
        assert_eq!(parse_guess("101"), None);
    }

    #[test]
    fn test_parse_guess_invalid() {
        assert_eq!(parse_guess("-1"), None);
        assert_eq!(parse_guess("abc"), None);
        assert_eq!(parse_guess(""), None);
        assert_eq!(parse_guess("   "), None); // 空白のみ
        assert_eq!(parse_guess("12.5"), None);
        assert_eq!(parse_guess("1e10"), None); // 科学的表記
    }

    // get_hint テスト
    #[test]
    fn test_get_hint() {
        assert_eq!(get_hint(50, 75), Ordering::Less);
        assert_eq!(get_hint(75, 50), Ordering::Greater);
        assert_eq!(get_hint(50, 50), Ordering::Equal);
        // 境界値
        assert_eq!(get_hint(1, 100), Ordering::Less);
        assert_eq!(get_hint(100, 1), Ordering::Greater);
    }

    // format_hint_message テスト
    #[test]
    fn test_format_hint_message() {
        assert_eq!(format_hint_message(Ordering::Less), messages::TOO_SMALL);
        assert_eq!(format_hint_message(Ordering::Greater), messages::TOO_BIG);
        assert_eq!(format_hint_message(Ordering::Equal), messages::CORRECT);
    }

    // config テスト
    #[test]
    fn test_config_values() {
        assert_eq!(config::MIN_NUMBER, 1);
        assert_eq!(config::MAX_NUMBER, 100);
        assert!(config::MIN_NUMBER < config::MAX_NUMBER);
    }
}
