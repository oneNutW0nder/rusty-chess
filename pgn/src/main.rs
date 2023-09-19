extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::fs;

#[derive(Parser)]
#[grammar = "pgn.pest"]
struct PGN;

fn main() -> std::io::Result<()> {
    // Get args and check for possibility of a file path existing in argv[1]
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("No file provided!");
        println!("Usage: pgn PGN_FILE");
        std::process::exit(-1);
    }
    let pgn: &String = &args[1];
    println!("Attempting to parse {}...", pgn);

    // Attempt to open file and then convert bytes to utf8 slice
    let contents: Vec<u8> = fs::read(pgn).expect("Failed to open file");
    let contents: &str = match std::str::from_utf8(&contents) {
        Ok(s) => s,
        Err(e) => panic!("Failed during utf-8 decoding {}", e),
    };

    println!("{contents}");

    // Begin testing pest stuff
    let pairs = PGN::parse(Rule::Game, contents).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
    }

    Ok(())
}
