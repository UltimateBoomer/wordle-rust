use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args = Config::parse();

    println!("Using word file: {}", args.filename);
    println!("Max guesses: {}", args.max_guesses);

    let word_file = File::open(args.filename).expect("Failed to open file");
    
    let reader = BufReader::new(word_file);
    let words: Vec<String> = reader.lines().map(Result::unwrap).collect();

    println!("Lines: {}", words.len());
}

#[derive(Parser, Debug)]
struct Config {
    #[arg(long, default_value_t = String::from("words.txt"))]
    filename: String,

    #[arg(long, default_value_t = 6)]
    max_guesses: u32,
}