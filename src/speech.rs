//! Dialogue module

use crate::Range;

/// All dialogues in the game
pub struct Speech {}

impl Speech {
    pub fn greeting(range: &Range) {
        println!("\n------------------------");
        println!("  The Guessing Game 🤔");
        println!("------------------------\n");
        println!(
            "Let's play the game. I thought of a number between {} and {}.",
            range.min, range.max
        );
        println!("Try to guess it! Please type your number:\n");
    }

    pub fn not_a_number() {
        println!("\nThis is not a number 😱 try again:\n");
    }

    pub fn invalid_range(range: &Range) {
        println!(
            "\nPlease type a number between {} and {}:\n",
            range.min, range.max
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
