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
    pub word_list: Vec<String>
}

pub fn setup(conf: Config) -> Result<WordleGame, io::Error> {
    let word_file = File::open(&conf.filename)?;
    
    let reader = BufReader::new(word_file);
    let mut word_list: Vec<String> = reader.lines().map(Result::unwrap).collect();
    word_list.sort_unstable();

    println!("Using word file: {} ({} words)", conf.filename, word_list.len());
    println!("Max guesses: {}", conf.max_guesses);

    return Result::Ok(WordleGame {
        word: word_list.choose(&mut rand::thread_rng())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File is empty"))?
            .clone(), 
        word_list,
    });
}