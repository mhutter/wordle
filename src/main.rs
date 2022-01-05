use std::{collections::HashSet, env};

use itertools::Itertools;
use regex::Regex;

const WORDS: &str = include_str!("../words.txt");

const USAGE: &str = "usage: wordle UNUSED TRY_1 [TRY_N...]

  UNUSED:
    List of letters that are not used (grey letters).

  TRY_N:
    Past results, but only yellow & green letters.
    - lower case indicates YELLOW letter
    - upper case indicates GREEN letter
";

#[derive(Debug)]
struct Pos {
    must_not: HashSet<char>,
    must: Option<char>,
}

impl Pos {
    fn new(unused: HashSet<char>) -> Self {
        Self {
            must_not: unused,
            must: None,
        }
    }

    fn to_pattern(&self) -> String {
        if let Some(must) = self.must {
            return String::from(must);
        }

        format!("[^{}]", self.must_not.iter().collect::<String>())
    }
}

impl ToString for Pos {
    fn to_string(&self) -> String {
        if let Some(must) = self.must {
            return String::from(must);
        }

        let s: String = self.must_not.iter().collect();
        format!("[^{}]", s)
    }
}

fn main() {
    let mut args = env::args();
    // first entry is $0
    args.next();

    // Unused chars are not allowed in ANY position
    let unused: HashSet<char> = args.next().expect(USAGE).chars().collect();

    // "needles" are chars that ARE present in the word but their position is
    // unknown
    let mut needles = HashSet::new();

    // By default, every position is a negative match of all unused chars
    let mut pos: Vec<Pos> = (0..5).map(|_| Pos::new(unused.clone())).collect();

    // Parse past tries
    for t in args {
        for (i, c) in t.char_indices() {
            match c {
                c if c.is_uppercase() => {
                    pos[i].must = Some(c.to_ascii_lowercase());
                }
                c if c.is_lowercase() => {
                    needles.insert(c);
                    pos[i].must_not.insert(c);
                }
                _ => {}
            }
        }
    }

    // Generated needle permutations
    let mut chars = [' '; 5];
    for (i, c) in needles.iter().enumerate() {
        chars[i] = *c;
    }

    let permutations: Vec<Vec<char>> = chars
        .into_iter()
        .permutations(5)
        // filter out duplicates caused by duplicate spaces (empty slots)
        .unique()
        .filter(|s| {
            !pos.iter().enumerate().any(|(i, p)| {
                let c = s[i];
                // skip elements that have a definite value or that have the
                // current char in ther "must not" list at this position
                p.must != None || p.must_not.contains(&c)
            })
        })
        // convert to strings
        .collect();

    let patterns: Vec<String> = permutations
        .iter()
        .map(|permutation| {
            let mut out = String::new();
            for (i, c) in permutation.iter().enumerate() {
                out.push_str(&match c {
                    ' ' => pos[i].to_pattern(),
                    c => c.to_string(),
                });
            }
            out
        })
        .collect();

    let pattern = format!("^({})$", patterns.join("|"));
    let re = Regex::new(&pattern).unwrap();
    let words: String = WORDS.lines().filter(|line| re.is_match(line)).join("\n");
    println!("{}", words);
}
