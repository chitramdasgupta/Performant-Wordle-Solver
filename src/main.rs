use wordle_solver::algorithms::Naive;
use wordle_solver::play;

const GAMES: &str = include_str!("./answers.txt");

fn main() {
    let guesser = Naive::new();

    for answer in GAMES.split_whitespace() {
        play(answer, guesser);
    }
}
