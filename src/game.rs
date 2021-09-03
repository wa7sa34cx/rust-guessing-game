//! Game module

use crate::Range;
use rand::Rng;

/// Game state
#[derive(Debug)]
pub struct Game {
    pub number: u32,
}

impl Game {
    /// Initiates new game
    ///
    /// # Example
    ///
    /// ```
    /// let game = Game::new();
    /// ```
    pub fn new(range: &Range) -> Self {
        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { number }
    }
}
