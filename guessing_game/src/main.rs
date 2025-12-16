use guessing_game::run_game;

fn main() {
    if let Err(e) = run_game() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
