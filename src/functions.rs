//! Helper functions module

use crate::Range;

/// Returns `true` if the number is a [`Some`] value
///
/// # Examples
///
/// ```
/// assert_eq!(is_not_a_number(Some(42)), false);
///
/// assert_eq!(is_not_a_number(None), true);
/// ```
pub fn is_not_a_number(number: &Option<u32>) -> bool {
    if number.is_some() {
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
/// let range = Range::from(1, 100);
///
/// assert_eq!(is_invalid_range(&Some(42), &range), false);
///
/// assert_eq!(is_invalid_range(&Some(200), &range), true);
///
/// assert_eq!(is_invalid_range(&Some(0), &range), true);
/// ```
pub fn is_invalid_range(number: &Option<u32>, range: &Range) -> bool {
    if number.unwrap() < range.min {
        true
    } else if number.unwrap() > range.max {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    // use super::Range;
    use super::*;

    #[test]
    fn test_is_not_a_number() {
        assert_eq!(is_not_a_number(&None), true);
        assert_eq!(is_not_a_number(&Some(42)), false);
    }

    #[test]
    fn test_is_invalid_range() {
        let range = Range::from(30, 60);

        assert_eq!(is_invalid_range(&Some(42), &range), false);
        assert_eq!(is_invalid_range(&Some(90), &range), true);
        assert_eq!(is_invalid_range(&Some(10), &range), true);
    }
}
