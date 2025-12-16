use std::io;

use guessing_game::run_game;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_game(&mut stdin.lock(), &mut stdout.lock());
}
