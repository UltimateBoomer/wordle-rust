pub mod cli;

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
#[derive(Clone)]
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
    pub fn from_config(conf: &Config) -> Result<WordleGame, io::Error> {
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
    /// Create a `WordleSession` in starting state.
    pub fn new(game: &WordleGame) -> WordleSession {
        WordleSession { 
            game: game.clone(), 
            guesses: Vec::new(), 
        }
    }

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
            _ => {
                Err(result)
            }
        }
    }

    /// Evaluates the individual letters of `word` for whether they are in the right position, and produces a `GuessResult`.
    pub fn eval(&self, word: &String) -> GuessResult {
        if word.len() != self.game.word_len {
            GuessResult::Invalid
        } else if self.guesses.iter().any(|w| w.0 == *word) {
            GuessResult::AlreadyUsed
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
    AlreadyUsed,
    Invalid,
}

#[derive(PartialEq, Eq, Hash)]
pub enum LetterValidity {
    Correct,
    WrongPos,
    Incorrect,
}

#[cfg(test)]
mod tests {
    use std::{vec};

    use crate::{WordleGame, WordleSession, GuessResult, LetterValidity, Config};

    #[test]
    fn new_wordle_game() {
        let game = WordleGame::from_config(&Config { 
            filename: String::from("words.txt"), 
            max_guesses: 5 
        });
        assert!(game.is_ok());
        let game = game.unwrap();
        assert!(!game.word_list.is_empty());
    }

    #[test]
    fn eval1() {
        let ws = WordleSession {
            game: WordleGame { 
                word: String::from("aaaaa"), 
                word_list: vec![String::from("aaaaa"), String::from("bbbbb")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        assert!(matches!(ws.eval(&String::from("x")), GuessResult::Invalid))
    }

    #[test]
    fn eval2() {
        use LetterValidity::*;

        let ws = WordleSession {
            game: WordleGame { 
                word: String::from("apple"), 
                word_list: vec![String::from("apple"), String::from("grape")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        let r = ws.eval(&String::from("grape"));
        assert!(match r {
            GuessResult::Ok(v) => v == vec![Incorrect, Incorrect, WrongPos, WrongPos, Correct],
            _ => false,
        });
    }

    #[test]
    fn eval3() {
        let ws = WordleSession {
            game: WordleGame { 
                word: String::from("aaaaa"), 
                word_list: vec![String::from("aaaaa"), String::from("bbbbb")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        assert!(matches!(ws.eval(&String::from("ccccc")), GuessResult::NotInDict))
    }

    #[test]
    fn eval4() {
        let mut ws = WordleSession {
            game: WordleGame { 
                word: String::from("aaaaa"), 
                word_list: vec![String::from("aaaaa"), String::from("bbbbb")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        assert!(ws.guess(&String::from("bbbbb")).is_ok());
        assert!(matches!(ws.eval(&String::from("bbbbb")), GuessResult::AlreadyUsed))
    }

    #[test]
    fn guess1() {
        use LetterValidity::*;

        let mut ws = WordleSession {
            game: WordleGame { 
                word: String::from("aaaaa"), 
                word_list: vec![String::from("aaaaa"), String::from("bbbbb")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        assert!(ws.guess(&String::from("bbbbb")).is_ok());
        assert!(*ws.guesses.get(0).unwrap() ==
            (String::from("bbbbb"), vec![Incorrect, Incorrect, Incorrect, Incorrect, Incorrect]))
    }

    #[test]
    fn guess2() {
        let mut ws = WordleSession {
            game: WordleGame { 
                word: String::from("aaaaa"), 
                word_list: vec![String::from("aaaaa"), String::from("bbbbb")], 
                word_len: 5, 
                max_guesses: 2,
            },
            guesses: Vec::new(),
        };
        assert!(ws.guess(&String::from("ccccc")).is_err());
        assert!(ws.guesses.is_empty());
    }
}