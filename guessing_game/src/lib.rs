use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// ゲームのメインロジック
/// 入力と出力を抽象化することでテスト可能にする
///
/// # Errors
/// 入出力操作が失敗した場合に `io::Error` を返す
pub fn run_game<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<()> {
    let secret_number = rand::rng().random_range(1..=100);
    run_game_with_secret(reader, writer, secret_number)
}

/// 秘密の数字を指定してゲームを実行する（テスト用）
///
/// # Errors
/// 入出力操作が失敗した場合に `io::Error` を返す
pub fn run_game_with_secret<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    secret_number: u32,
) -> io::Result<()> {
    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] 秘密の数字は {} やで！", secret_number);

    writeln!(writer, "1から100の数字を当ててみぃや！")?;
    writeln!(writer, "ほな、予想入れてみて！")?;

    let mut guess = String::new();

    loop {
        guess.clear();
        let bytes_read = reader.read_line(&mut guess)?;
        if bytes_read == 0 {
            break; // EOF
        }

        match parse_guess(&guess) {
            Some(num) => {
                writeln!(writer, "あんたの予想は {} やな！", num)?;
                match num.cmp(&secret_number) {
                    Ordering::Less => writeln!(writer, "もっと大きいで！")?,
                    Ordering::Greater => writeln!(writer, "もっと小さいで！")?,
                    Ordering::Equal => {
                        writeln!(writer, "正解や！やったな！")?;
                        break;
                    }
                }
            }
            None => writeln!(writer, "ちゃんとした数字入れてや！")?,
        }
    }

    Ok(())
}

/// 入力文字列を数値に変換する
/// 1〜100の範囲内の有効な数値の場合は Some(u32) を返し、それ以外は None を返す
pub fn parse_guess(input: &str) -> Option<u32> {
    input
        .trim()
        .parse()
        .ok()
        .filter(|&n| (1..=100).contains(&n))
}
