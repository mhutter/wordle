use std::collections::HashSet;

use crate::position::Position;

/// Collects facts about a wordle riddle
#[derive(Debug)]
pub struct Word {
    /// Letters that are not in the word in any spot.
    unused_letters: HashSet<char>,

    /// Letters that are in the word but their position is unknown
    needles: HashSet<char>,

    /// Position-specific information, like letters that can NOT be in this spot
    /// or letters that are known to be in this spot.
    positions: [Position; 5],
}

impl Word {
    pub fn new(unused_letters: &str, tries: &[String]) -> Self {
        let unused_letters = unused_letters.chars().collect();
        let mut positions: [Position; 5] = Default::default();
        let mut needles = HashSet::new();

        // Parse past tries
        for t in tries {
            for (i, c) in t.char_indices() {
                match c {
                    c if c.is_uppercase() => {
                        positions[i].green = Some(c.to_ascii_lowercase());
                    }
                    c if c.is_lowercase() => {
                        needles.insert(c);
                        positions[i].yellow.insert(c);
                    }
                    _ => {}
                }
            }
        }

        Self {
            unused_letters,
            needles,
            positions,
        }
    }

    pub fn filter<'a>(&self, words: Vec<&'a str>) -> Vec<&'a str> {
        let mut words: Vec<&str> = words
            .into_iter()
            // filter out words that contain unused letters
            .filter(|word| !word.chars().any(|c| self.unused_letters.contains(&c)))
            // Filter for words that contain ALL of the needles
            .filter(|word| self.needles.iter().all(|&c| word.contains(c)))
            .collect();

        // filter for positional constraints
        for (i, position) in self.positions.iter().enumerate() {
            words = words
                .into_iter()
                .filter(|word| {
                    let c = word.chars().nth(i).unwrap_or(' ');
                    if let Some(green) = position.green {
                        c == green
                    } else {
                        !position.yellow.contains(&c)
                    }
                })
                .collect();
        }

        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let word = Word::new("", &[]);

        assert_eq!(word.unused_letters, HashSet::new(),);
        assert_eq!(
            word.positions,
            [
                Position::default(),
                Position::default(),
                Position::default(),
                Position::default(),
                Position::default(),
            ],
        );
    }

    #[test]
    fn new_parse_args() {
        let word = Word::new("wupsnm", &["P  ge".to_string(), " e".to_string()]);

        assert_eq!(
            word.unused_letters,
            HashSet::from(['w', 'u', 'p', 's', 'n', 'm'])
        );
        assert_eq!(word.needles, HashSet::from(['g', 'e']));
        assert_eq!(
            word.positions,
            [
                Position {
                    green: Some('p'),
                    ..Default::default()
                },
                Position {
                    yellow: HashSet::from(['e']),
                    ..Default::default()
                },
                Position::default(),
                Position {
                    yellow: HashSet::from(['g']),
                    ..Default::default()
                },
                Position {
                    yellow: HashSet::from(['e']),
                    ..Default::default()
                },
            ],
        );
    }

    #[test]
    fn filter_empty() {
        let word = Word::new("", &[]);
        let words = vec!["foo", "bar", "baz"];
        assert_eq!(words.clone(), word.filter(words));
    }

    #[test]
    fn filter_unused_chars() {
        let word = Word::new("fr", &[]);
        let words = vec!["foo", "bar", "baz"];
        assert_eq!(vec!["baz"], word.filter(words));
    }

    #[test]
    fn filter_known_spots() {
        let word = Word::new("", &["F".to_string()]);
        let words = vec!["foo", "bar", "baz"];
        assert_eq!(vec!["foo"], word.filter(words));
    }

    #[test]
    fn filter_needles() {
        let word = Word::new("", &["ar".to_string()]);
        let words = vec!["foo", "bar", "baz"];
        assert_eq!(vec!["bar"], word.filter(words));

        let word = Word::new("psn", &[" e i ".to_string()]);
        let words = vec!["foxie", "eiaaa"];
        assert_eq!(vec!["eiaaa"], word.filter(words));
    }

    #[test]
    fn filter_wordle() {
        let word = Word::new("aypitvgu", &["W  r".to_string(), "  l".to_string()]);
        let words = vec!["weary", "pilot", "vague", "wordle"];
        assert_eq!(vec!["wordle"], word.filter(words));
    }
}
