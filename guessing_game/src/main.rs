use std::io;

use guessing_game::run_game;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(e) = run_game(&mut stdin.lock(), &mut stdout.lock()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
