use std::io::{BufRead, Write};

/// ゲームのメインロジック
/// 入力と出力を抽象化することでテスト可能にする
pub fn run_game<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
    writeln!(writer, "Guess the number!").unwrap(); // 数を当ててごらん
    writeln!(writer, "Please input your guess.").unwrap(); // ほら、予想を入力してね

    let mut guess = String::new();

    reader.read_line(&mut guess).expect("Failed to read line"); // 行の読み込みに失敗しました

    match parse_guess(&guess) {
        Some(num) => writeln!(writer, "You guessed: {}", num).unwrap(), // 次のように予想しました: {}
        None => writeln!(writer, "Please enter a valid number!").unwrap(), // 有効な数値を入力してください
    }
}

/// 入力文字列を数値に変換する
/// 有効な数値の場合は Some(u32) を返し、無効な場合は None を返す
pub fn parse_guess(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

#[cfg(test)]
mod tests;
