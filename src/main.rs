use std::{env, process};

use word::Word;

mod position;
mod word;

const WORDS: &str = include_str!("../words.txt");

const USAGE: &str = "usage: wordle UNUSED TRY_1 [TRY_N...]

  UNUSED:
    List of letters that are not used (grey letters).

  TRY_N:
    Past results, but only yellow & green letters.
    - lower case indicates YELLOW letter
    - upper case indicates GREEN letter
";

fn main() {
    // Collect command line args
    let args: Vec<String> = env::args().collect();

    // Ensure we have enough arguments
    if args.len() < 2 {
        eprintln!("{}", USAGE);
        process::exit(1);
    }

    // Construct the word filter
    let word = Word::new(&args[1], &args[2..]);

    // Filter for possible words
    let words = word.filter(WORDS.lines().collect()).join("\n");

    println!("{}", words);
}
