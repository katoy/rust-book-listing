use super::*;
use std::io::Cursor;

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

    run_game(&mut input, &mut output);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Guess the number!"));
    assert!(output_str.contains("Please input your guess."));
    assert!(output_str.contains("You guessed: 42"));
}

#[test]
fn test_run_game_invalid_input() {
    let mut input = Cursor::new("abc\n");
    let mut output = Vec::new();

    run_game(&mut input, &mut output);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Guess the number!"));
    assert!(output_str.contains("Please input your guess."));
    assert!(output_str.contains("Please enter a valid number!"));
}
