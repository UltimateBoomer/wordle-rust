use std::{io::{BufReader, BufRead, self}, fs::File};

use clap::Parser;
use rand::seq::SliceRandom;

/// Configuration for Wordle games.
#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, default_value_t = String::from("words.txt"))]
    pub filename: String,

    #[arg(long, default_value_t = 6)]
    pub max_guesses: u32,
}

/// Defines the starting conditions of a Wordle game.
pub struct WordleGame {
    pub word: String,
    pub word_list: Vec<String>,
    pub word_len: usize,
    pub max_guesses: u32,
}

impl WordleGame {
    /// Create a `WordleGame` with the given config.
    /// # Errors
    /// The function will return an error if the word file cannot be read, or if the the word file is empty.
    pub fn new(conf: Config) -> Result<WordleGame, io::Error> {
        let word_file = File::open(&conf.filename)?;
        
        let reader = BufReader::new(word_file);
        let mut word_list: Vec<String> = reader.lines().map(Result::unwrap).collect();
        word_list.sort_unstable();
    
        println!("Using word file: {} ({} words)", conf.filename, word_list.len());
        println!("Max guesses: {}", conf.max_guesses);
    
        let selected_word = word_list.choose(&mut rand::thread_rng())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File is empty"))?
            .clone();
    
        let word_len = word_list.first().map_or_else(|| 0, String::len);
    
        return Result::Ok(WordleGame {
            word: selected_word, 
            word_list: word_list,
            word_len: word_len,
            max_guesses: conf.max_guesses,
        });
    }
}



