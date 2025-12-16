use guessing_game::{config, parse_guess, run_game_internal, run_game_with_secret};
use std::io::{self, BufRead, Cursor, Write};

// =============================================================================
// parse_guess テスト
// =============================================================================

#[test]
fn test_parse_guess_valid_number() {
    assert_eq!(parse_guess("42"), Some(42));
    assert_eq!(parse_guess("50"), Some(50));
}

#[test]
fn test_parse_guess_with_whitespace() {
    assert_eq!(parse_guess("  42  "), Some(42));
    assert_eq!(parse_guess("42\n"), Some(42));
    assert_eq!(parse_guess("\t42\t"), Some(42));
}

#[test]
fn test_parse_guess_invalid_input() {
    assert_eq!(parse_guess("abc"), None);
    assert_eq!(parse_guess(""), None);
    assert_eq!(parse_guess("12.5"), None); // 小数は無効
}

#[test]
fn test_parse_guess_negative_number() {
    // u32 なので負の数は None になる
    assert_eq!(parse_guess("-1"), None);
    assert_eq!(parse_guess("-42"), None);
}

#[test]
fn test_parse_guess_out_of_range() {
    // config の範囲外
    assert_eq!(parse_guess("0"), None);
    assert_eq!(parse_guess("101"), None);

    // 範囲内の境界値
    assert_eq!(
        parse_guess(&config::MIN_NUMBER.to_string()),
        Some(config::MIN_NUMBER)
    );
    assert_eq!(
        parse_guess(&config::MAX_NUMBER.to_string()),
        Some(config::MAX_NUMBER)
    );
}

// =============================================================================
// run_game_internal テスト
// =============================================================================

#[test]
fn test_run_game_internal_valid_input() {
    let mut input = Cursor::new("42\n");
    let mut output = Vec::new();

    run_game_internal(&mut input, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("1から100の数字を当ててみぃや！"));
    assert!(output_str.contains("ほな、予想入れてみて！"));
    assert!(output_str.contains("あんたの予想は 42 やな！"));
}

#[test]
fn test_run_game_internal_invalid_input() {
    let mut input = Cursor::new("abc\n");
    let mut output = Vec::new();

    run_game_internal(&mut input, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("1から100の数字を当ててみぃや！"));
    assert!(output_str.contains("ほな、予想入れてみて！"));
    assert!(output_str.contains("ちゃんとした数字入れてや！"));
}

#[test]
fn test_run_game_internal_eof() {
    // 空の入力（即座にEOF）
    let mut input = Cursor::new("");
    let mut output = Vec::new();

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("1から100の数字を当ててみぃや！"));
}

// =============================================================================
// run_game_with_secret テスト（全パスカバー）
// =============================================================================

#[test]
fn test_run_game_with_secret_guess_too_small() {
    // 予想が秘密の数字より小さい場合
    let mut input = Cursor::new("25\n50\n"); // 25は50より小さい、50で正解
    let mut output = Vec::new();

    run_game_with_secret(&mut input, &mut output, 50).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("あんたの予想は 25 やな！"));
    assert!(output_str.contains("もっと大きいで！"));
    assert!(output_str.contains("正解や！やったな！"));
}

#[test]
fn test_run_game_with_secret_guess_too_big() {
    // 予想が秘密の数字より大きい場合
    let mut input = Cursor::new("75\n50\n"); // 75は50より大きい、50で正解
    let mut output = Vec::new();

    run_game_with_secret(&mut input, &mut output, 50).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("あんたの予想は 75 やな！"));
    assert!(output_str.contains("もっと小さいで！"));
    assert!(output_str.contains("正解や！やったな！"));
}

#[test]
fn test_run_game_with_secret_correct_guess() {
    // 最初から正解の場合
    let mut input = Cursor::new("42\n");
    let mut output = Vec::new();

    run_game_with_secret(&mut input, &mut output, 42).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("あんたの予想は 42 やな！"));
    assert!(output_str.contains("正解や！やったな！"));
    assert!(!output_str.contains("もっと大きいで！"));
    assert!(!output_str.contains("もっと小さいで！"));
}

// =============================================================================
// I/O エラーハンドリングテスト用ヘルパー
// =============================================================================

/// 書き込み時に常にエラーを返す Writer
struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "simulated write error",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// 読み込み時に常にエラーを返す Reader
struct FailingReader;

impl BufRead for FailingReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "simulated read error"))
    }

    fn consume(&mut self, _amt: usize) {}
}

impl io::Read for FailingReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "simulated read error"))
    }
}

/// 指定回数の書き込み後にエラーを返す Writer
struct FailAfterNWritesWriter {
    writes_before_fail: usize,
    write_count: usize,
}

impl FailAfterNWritesWriter {
    fn new(writes_before_fail: usize) -> Self {
        Self {
            writes_before_fail,
            write_count: 0,
        }
    }
}

impl Write for FailAfterNWritesWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.write_count >= self.writes_before_fail {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "simulated write error",
            ))
        } else {
            self.write_count += 1;
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// =============================================================================
// I/O エラーハンドリングテスト
// =============================================================================

#[test]
fn test_run_game_internal_write_error_first_writeln() {
    // 最初の writeln! でエラー
    let mut input = Cursor::new("42\n");
    let mut output = FailingWriter;

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[test]
fn test_run_game_internal_read_error() {
    // read_line でエラー
    let mut input = FailingReader;
    let mut output = Vec::new();

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[test]
fn test_run_game_internal_write_error_after_read() {
    // 3回目の書き込み（結果表示）でエラー
    let mut input = Cursor::new("42\n");
    let mut output = FailAfterNWritesWriter::new(2);

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_run_game_internal_write_error_second_writeln() {
    // 2番目の writeln! でエラー（1回書き込み成功後にエラー）
    let mut input = Cursor::new("42\n");
    let mut output = FailAfterNWritesWriter::new(1);

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_run_game_internal_write_error_on_invalid_input_message() {
    // 無効な入力時の結果表示でエラー
    let mut input = Cursor::new("abc\n");
    let mut output = FailAfterNWritesWriter::new(2);

    let result = run_game_internal(&mut input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_run_game_with_secret_write_error_on_less() {
    // "もっと大きいで！" のwriteln!でエラー
    let mut input = Cursor::new("25\n"); // 25 < 50
    let mut output = FailAfterNWritesWriter::new(3); // 3回書き込み後にエラー

    let result = run_game_with_secret(&mut input, &mut output, 50);
    assert!(result.is_err());
}

#[test]
fn test_run_game_with_secret_write_error_on_greater() {
    // "もっと小さいで！" のwriteln!でエラー
    let mut input = Cursor::new("75\n"); // 75 > 50
    let mut output = FailAfterNWritesWriter::new(3); // 3回書き込み後にエラー

    let result = run_game_with_secret(&mut input, &mut output, 50);
    assert!(result.is_err());
}

#[test]
fn test_run_game_with_secret_write_error_on_equal() {
    // "正解や！やったな！" のwriteln!でエラー
    let mut input = Cursor::new("50\n"); // 50 == 50
    let mut output = FailAfterNWritesWriter::new(3); // 3回書き込み後にエラー

    let result = run_game_with_secret(&mut input, &mut output, 50);
    assert!(result.is_err());
}
