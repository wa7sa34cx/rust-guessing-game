//! Guess module

use crate::Game;
use std::io;

/// Guessing state
#[derive(Debug)]
pub struct Guess {
    pub number: Option<u32>,
}

impl Guess {
    /// Initiates new guess number
    ///
    /// # Examples
    ///
    /// ```
    /// let guess = Guess::new();
    /// ```
    pub fn new() -> Self {
        Guess { number: None }
    }

    /// Reads a line of input, and try converting to a number
    ///
    /// # Examples
    ///
    /// ```
    /// let mut guess = Guess::new();
    /// guess.try_guess();
    /// ```
    pub fn try_guess(&mut self) {
        // get string from stdin
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        // try to convert
        self.number = s.trim().parse().ok();
    }

    /// Returns `true` if the number is a [`Some`] value
    ///
    /// # Examples
    ///
    /// ```
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

    /// Returns `true` if the number is out of [`Range`]
    ///
    /// # Examples
    ///
    /// ```
    /// // for range between 1 and 100:
    ///
    /// let game = Game::new();
    ///
    /// let guess = Guess { number: Some(42) };
    /// assert_eq!(guess.is_invalid_range(&game), false);
    ///
    /// let guess = Guess { number: Some(123) };
    /// assert_eq!(guess.is_invalid_range(&game), true);
    ///
    /// let guess = Guess { number: Some(0) };
    /// assert_eq!(guess.is_invalid_range(&game), true);
    /// ```
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
