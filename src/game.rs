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

impl Range {
    /// Set the range
    #[allow(dead_code)]
    pub fn set(&mut self, min: u32, max: u32) {
        if min >= max {
            panic!("max ({}) must be greate than min ({})", min, max);
        }

        self.min = min;
        self.max = max;
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
    pub fn new(range: Range) -> Self {
        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { range: range, number }
    }
}
