//! Range module

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
    /// Creates range from parameters
    ///
    /// # Panics
    ///
    /// Panics if min >= max
    ///
    /// # Examples
    ///
    /// ```
    /// let range = Range::from(30, 90);
    /// ```
    #[allow(dead_code)]
    pub fn from(min: u32, max: u32) -> Self {
        if min >= max {
            panic!("max ({}) must be greate than min ({})", max, min);
        }

        Range { min, max }
    }
}
