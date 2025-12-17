use guessing_game::run_game;

fn main() {
    // Ctrl-C シグナルハンドラを設定
    ctrlc::set_handler(|| {
        println!("\n中断されました。ゲームを終了します。");
        std::process::exit(0);
    })
    .expect("Ctrl-C ハンドラの設定に失敗しました");

    if let Err(e) = run_game() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
