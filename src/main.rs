mod game;
mod guess;
mod speech;

use game::Game;
use guess::Guess;
use speech::Speech;
use std::cmp::Ordering;

/// Let's play the game!
fn main() {
    // Start new game
    let game = Game::new();
    let mut guess = Guess::new();

    // Welcome to the game
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
