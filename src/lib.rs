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
    /// Create a `WordleGame` from the given config.
    /// # Errors
    /// The function will return an error if the word file cannot be read, or if the the word file is empty.
    pub fn new(conf: Config) -> Result<WordleGame, io::Error> {
        let word_file = File::open(&conf.filename)?;
        
        let reader = BufReader::new(word_file);
        let mut word_list: Vec<String> = reader.lines().map(Result::unwrap).collect();

        if word_list.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "Word file is empty"));
        }

        word_list.sort_unstable();
    
        println!("Using word file: {} ({} words)", conf.filename, word_list.len());
        println!("Max guesses: {}", conf.max_guesses);
    
        let selected_word = word_list.choose(&mut rand::thread_rng()).unwrap().clone();
    
        let word_len = word_list.first().unwrap().len();
    
        return Result::Ok(WordleGame {
            word: selected_word, 
            word_list: word_list,
            word_len: word_len,
            max_guesses: conf.max_guesses,
        });
    }
}

/// Defines a Wordle game with a list of previous guesses.
pub struct WordleSession {
    pub game: WordleGame,
    guesses: Vec<(String, Vec<LetterValidity>)>,
}

impl WordleSession {
    /// Makes a guess using `word`. If the guess is valid, then append the guess onto self. 
    pub fn guess(&mut self, word: &String) -> Result<GameResult, GuessResult> {
        let result = self.eval(word);
        match result {
            GuessResult::Ok(r) => {
                self.guesses.push((word.clone(), r));
                if self.game.word == *word {
                    Ok(GameResult::Win)
                } else if self.guesses.len() == self.game.max_guesses.try_into().unwrap() {
                    Ok(GameResult::OutOfGuesses)
                } else {
                    Ok(GameResult::Cont)
                }
            }
            GuessResult::Invalid | GuessResult::NotInDict => {
                Err(result)
            }
        }
    }

    /// Evaluates the individual letters of `word` for whether they are in the right position, and produces a `GuessResult`.
    pub fn eval(&self, word: &String) -> GuessResult {
        if word.len() != self.game.word_len {
            GuessResult::Invalid
        } else if self.game.word_list.binary_search(&word).is_err() {
            GuessResult::NotInDict
        } else {
            GuessResult::Ok(word.chars().enumerate().map(|(i, c)| {
                if self.game.word.chars().nth(i).unwrap() == c {
                    LetterValidity::Correct
                } else if self.game.word.chars().any(|c2| c == c2) {
                    LetterValidity::WrongPos
                } else {
                    LetterValidity::Incorrect
                }
            }).collect())
        }
    }

    pub fn get_guesses(&self) -> &Vec<(String, Vec<LetterValidity>)> {
        &self.guesses
    }
}

pub enum GameResult {
    Win,
    Cont,
    OutOfGuesses,
}

pub enum GuessResult {
    Ok(Vec<LetterValidity>),
    NotInDict,
    Invalid,
}

pub enum LetterValidity {
    Correct,
    WrongPos,
    Incorrect,
}