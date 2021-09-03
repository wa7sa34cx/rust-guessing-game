//! Game module

use rand::Rng;

/// Range for the game. Default is 1..100
#[derive(Debug)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

impl Default for Range {
    fn default() -> Self {
        Range { min: 1, max: 100 }
    }
}

/// Starting state of the game
#[derive(Debug)]
pub struct Game {
    pub range: Range,
    pub number: u32,
}

impl Game {
    /// Initiates new game
    ///
    /// # Examples
    ///
    /// ```
    /// let game = Game::new();
    /// ```
    pub fn new() -> Self {
        // Get default range
        let range = Range::default();

        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { range, number }
    }
}
