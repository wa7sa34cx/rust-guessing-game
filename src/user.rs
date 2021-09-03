//! User module

use std::io;

/// User state
#[derive(Debug)]
pub struct User {
    pub number: Option<u32>,
}

impl User {
    /// Initiates new user state
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new();
    /// ```
    pub fn new() -> Self {
        User { number: None }
    }

    /// Reads a string from an input and tries to convert it to a number
    ///
    /// # Examples
    ///
    /// ```
    /// let mut user = User::new();
    /// user.try_guess();
    /// ```
    pub fn try_guess(&mut self) {
        // get string from stdin
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        // try to convert
        self.number = s.trim().parse().ok();
    }
}
