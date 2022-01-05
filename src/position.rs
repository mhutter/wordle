use std::collections::HashSet;

/// Position represents specific rules for a single position in the word.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Position {
    /// List of characters that must NOT appear at the given position
    ///
    /// Usually for indicating a yellow letter at this position: The letter MUST
    /// be part of the sting but must NOT be at this position.
    pub yellow: HashSet<char>,
    /// Indicates a green letter. Overrules everything else.
    pub green: Option<char>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements_default() {
        let it = Position::default();

        assert_eq!(None, it.green);
    }
}
