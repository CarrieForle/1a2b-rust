use one_a_two_b::run_game;

fn main() {
    if let Err(e) = run_game() {
        eprintln!("An error occured during the game: {e}");
    }
}