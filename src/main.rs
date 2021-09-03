mod functions;
mod game;
mod range;
mod speech;
mod user;

use functions::*;
use game::Game;
use range::Range;
use speech::Speech;
use std::cmp::Ordering;
use user::User;

/// Let's play the game!
fn main() {
    // Start new game
    // let range = Range::from(30, 90); // Possible range
    let range = Range::default();
    let game = Game::new(&range);
    let mut user = User::new();

    // Welcome to the game
    Speech::greeting(&range);

    // Guessing loop
    loop {
        // Try to get a number from input
        user.try_guess();

        // Check that the player has entered a number
        if is_not_a_number(&user.number) {
            Speech::not_a_number();
            continue;
        }

        // Check if user number is in valid range
        if is_invalid_range(&user.number, &range) {
            Speech::invalid_range(&range);
            continue;
        }

        // Compare the guess number with game number
        match user.number.unwrap().cmp(&game.number) {
            Ordering::Less => Speech::less(),
            Ordering::Greater => Speech::greater(),
            Ordering::Equal => {
                Speech::equal();
                break;
            }
        }
    }
}
