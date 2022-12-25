use std::{io::{BufReader, BufRead, self}, fs::File};

use clap::Parser;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, default_value_t = String::from("words.txt"))]
    pub filename: String,

    #[arg(long, default_value_t = 6)]
    pub max_guesses: u32,
}

pub struct WordleGame {
    pub word: String,
}

pub fn setup(conf: Config) -> Result<WordleGame, io::Error> {
    let word_file = File::open(&conf.filename)?;
    
    let reader = BufReader::new(word_file);
    let words: Vec<String> = reader.lines().map(Result::unwrap).collect();

    println!("Using word file: {} ({} words)", conf.filename, words.len());
    println!("Max guesses: {}", conf.max_guesses);

    return Result::Ok(WordleGame { word: words.choose(&mut rand::thread_rng()).unwrap().clone() });
}