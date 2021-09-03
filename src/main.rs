use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Range for our game. It can be anything,
// but we implement default values
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
    number: u32,
}

impl Game {
    /// Initiate new game
    pub fn new() -> Self {
        // Get default range
        let range = Range::default();

        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { range, number }
    }
}

struct Speech {}

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
struct Guess {
    number: Option<u32>,
}

impl Guess {
    /// Initiate new guess number
    pub fn new() -> Self {
        Guess { number: None }
    }
}

impl Guess {
    /// Get string from stdin and try to convert into the number
    pub fn try_guess(&mut self) {
        // get string from stdin
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        // try to convert
        self.number = s.trim().parse().ok();
    }

    /// Returns true if the number is a Some value.
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

fn main() {
    // Start new game
    let game = Game::new();
    let mut guess = Guess::new();

    // Greeting to game
    Speech::greeting(&game);

    // Guessing loop
    loop {
        // Try guess
        guess.try_guess();

        // Check that the player has entered a number
        if guess.is_not_a_number() {
            Speech::not_a_number();
            continue;
        }

        // Check if guessing number is in valid range
        if guess.is_invalid_range(&game) {
            Speech::invalid_range(&game);
            continue;
        }

        // Compare the guess number with game number
        match guess.number.unwrap().cmp(&game.number) {
            Ordering::Less => Speech::less(),
            Ordering::Greater => Speech::greater(),
            Ordering::Equal => {
                Speech::equal();
                break;
            }
        }
    }
}
