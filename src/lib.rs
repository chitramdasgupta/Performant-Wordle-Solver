pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();

    for i in 1..=32 {
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }

        let correctness = Correctness::compute(answer, &guess);

        history.push(Guess {
            word: guess,
            mask: correctness,
        })
    }

    None
}

pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        todo!();
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
