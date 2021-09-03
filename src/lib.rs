use rand::Rng;
use std::io;

// Range for our game. It can be anything,
// but we implement default values.
#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Default for Range {
    fn default() -> Self {
        Range { min: 1, max: 100 }
    }
}

// Game init
#[derive(Debug)]
pub struct Game {
    range: Range,
    pub number: u32,
}

impl Game {
    /// Initiate new game.
    pub fn new() -> Self {
        // Get default range
        let range = Range::default();

        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { range, number }
    }
}

pub struct Speech {}

impl Speech {
    pub fn greeting(game: &Game) {
        println!("\n------------------------");
        println!("  The Guessing Game 🤔");
        println!("------------------------\n");
        println!(
            "Let's play a game. I thought of a number between {} and {}.",
            game.range.min, game.range.max
        );
        println!("Try to guess it! Please type your number:\n");
    }

    pub fn not_a_number() {
        println!("This is not a number 😱 try again:\n");
    }

    pub fn invalid_range(game: &Game) {
        println!(
            "Please type a number between {} and {}:\n",
            game.range.min, game.range.max
        );
    }

    pub fn less() {
        println!("Too small ⬆️  try again:\n");
    }

    pub fn greater() {
        println!("Too big ⬇️  try again:\n");
    }

    pub fn equal() {
        println!("Congrats 🎉 you guessed it!\n");
    }
}

#[derive(Debug)]
pub struct Guess {
    pub number: Option<u32>,
}

impl Guess {
    /// Initiate new guess number
    pub fn new() -> Self {
        Guess { number: None }
    }
}

impl Guess {
    /// Get string from stdin and try converting to a number.
    pub fn try_guess(&mut self) {
        // get string from stdin
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        // try to convert
        self.number = s.trim().parse().ok();
    }

    /// Returns `true` if the number is a [`Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    ///  # use rust_guessing_game::Guess;
    /// let guess = Guess { number: Some(42) };
    /// assert_eq!(guess.is_not_a_number(), false);
    ///
    /// let guess = Guess { number: None };
    /// assert_eq!(guess.is_not_a_number(), true);
    /// ```
    pub fn is_not_a_number(&self) -> bool {
        if self.number.is_some() {
            false
        } else {
            true
        }
    }

    pub fn is_invalid_range(&self, game: &Game) -> bool {
        if self.number.unwrap() < game.range.min {
            true
        } else if self.number.unwrap() > game.range.max {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Guess};

    #[test]
    fn guess_is_not_a_number() {
        let mut guess = Guess::new();

        guess.number = None;
        assert_eq!(guess.is_not_a_number(), true);

        guess.number = Some(42);
        assert_eq!(guess.is_not_a_number(), false);
    }

    #[test]
    fn guess_is_invalid_range() {
        let game = Game::new();
        let mut guess = Guess::new();

        guess.number = Some(42);
        assert_eq!(guess.is_invalid_range(&game), false);
        

        guess.number = Some(game.range.min - 1);
        assert_eq!(guess.is_invalid_range(&game), true);

        guess.number = Some(game.range.max + 1);
        assert_eq!(guess.is_invalid_range(&game), true);
    }
}