//! Dialogue module

use crate::Game;

/// All dialogues in the game
pub struct Speech {}

impl Speech {
    pub fn greeting(game: &Game) {
        println!("\n------------------------");
        println!("  The Guessing Game 🤔");
        println!("------------------------\n");
        println!(
            "Let's play the game. I thought of a number between {} and {}.",
            game.range.min, game.range.max
        );
        println!("Try to guess it! Please type your number:\n");
    }

    pub fn not_a_number() {
        println!("\nThis is not a number 😱 try again:\n");
    }

    pub fn invalid_range(game: &Game) {
        println!(
            "\nPlease type a number between {} and {}:\n",
            game.range.min, game.range.max
        );
    }

    pub fn less() {
        println!("\nToo small ⬆️  try again:\n");
    }

    pub fn greater() {
        println!("\nToo big ⬇️  try again:\n");
    }

    pub fn equal() {
        println!("\nCongrats 🎉 you guessed it!\n");
    }
}
