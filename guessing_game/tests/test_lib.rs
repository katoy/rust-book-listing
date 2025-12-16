use guessing_game::{parse_guess, run_game};
use std::io::{self, BufRead, Cursor, Write};

#[test]
fn test_parse_guess_valid_number() {
    assert_eq!(parse_guess("42"), Some(42));
    assert_eq!(parse_guess("1"), Some(1));
    assert_eq!(parse_guess("100"), Some(100));
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
fn test_run_game_valid_input() {
    let mut input = Cursor::new("42\n");
    let mut output = Vec::new();

    run_game(&mut input, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Guess the number!"));
    assert!(output_str.contains("Please input your guess."));
    assert!(output_str.contains("You guessed: 42"));
}

#[test]
fn test_run_game_invalid_input() {
    let mut input = Cursor::new("abc\n");
    let mut output = Vec::new();

    run_game(&mut input, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Guess the number!"));
    assert!(output_str.contains("Please input your guess."));
    assert!(output_str.contains("Please enter a valid number!"));
}

// I/O エラーをシミュレートするためのヘルパー構造体

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

#[test]
fn test_run_game_write_error_first_writeln() {
    // 最初の writeln! でエラー
    let mut input = Cursor::new("42\n");
    let mut output = FailingWriter;

    let result = run_game(&mut input, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[test]
fn test_run_game_read_error() {
    // read_line でエラー
    let mut input = FailingReader;
    let mut output = Vec::new();

    let result = run_game(&mut input, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[test]
fn test_run_game_write_error_after_read() {
    // 3回目の書き込み（結果表示）でエラー
    let mut input = Cursor::new("42\n");
    let mut output = FailAfterNWritesWriter::new(2);

    let result = run_game(&mut input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_run_game_write_error_second_writeln() {
    // 2番目の writeln! でエラー（1回書き込み成功後にエラー）
    let mut input = Cursor::new("42\n");
    let mut output = FailAfterNWritesWriter::new(1);

    let result = run_game(&mut input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_run_game_write_error_on_invalid_input_message() {
    // 無効な入力時の結果表示でエラー
    let mut input = Cursor::new("abc\n");
    let mut output = FailAfterNWritesWriter::new(2);

    let result = run_game(&mut input, &mut output);
    assert!(result.is_err());
}
