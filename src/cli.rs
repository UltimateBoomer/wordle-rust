/// CLI backend for Wordle.
use std::{io::{self, Write, BufRead}, collections::HashMap};

use colored::{Color, Colorize};

use crate::{WordleSession, WordleGame, LetterValidity, GuessResult, GameResult};

pub struct WordleSessionCLI<R, W> {
    session: WordleSession,
    reader: R,
    writer: W,
    color_map: HashMap<LetterValidity, Color>
}

impl<R: BufRead, W: Write> WordleSessionCLI<R, W> {
    /// Create a `WordleSessionCLI` in starting state.
    pub fn new(game: &WordleGame, reader: R, writer: W) -> WordleSessionCLI<R, W> {
        WordleSessionCLI { 
            session: WordleSession::new(&game),
            reader: reader,
            writer: writer,
            color_map: HashMap::from([
                (LetterValidity::Correct, Color::BrightGreen),
                (LetterValidity::Incorrect, Color::BrightWhite),
                (LetterValidity::WrongPos, Color::BrightYellow),
            ]),
        }   
    }

    /// Run the Wordle game.
    pub fn run(&mut self) -> Result<(), io::Error> {
        writeln!(&mut self.writer, "Welcome to Wordle in Rust!")?;
        
        loop {
            self.print_board()?;
            
            loop {
                writeln!(&mut self.writer, "Enter your word:")?;
                
                let mut input = String::new();
                self.reader.read_line(&mut input)?;
                let input = input.trim().to_string();

                let r = self.session.guess(&input);
                writeln!(&mut self.writer, "You entered: {}", &input)?;
                match r {
                    Ok(result) => match result {
                        GameResult::Cont => break,
                        GameResult::OutOfGuesses => {
                            writeln!(&mut self.writer, "Game over: out of guesses.")?;
                            return Ok(());
                        },
                        GameResult::Win => {
                            writeln!(&mut self.writer, "You win!")?;
                            return Ok(());
                        },
                    },
                    Err(result) => match result {
                        GuessResult::AlreadyUsed => writeln!(&mut self.writer, "You've already used that word!")?,
                        GuessResult::NotInDict => writeln!(&mut self.writer, "That word doesn't exist.")?,
                        GuessResult::Invalid => writeln!(&mut self.writer, "Invalid word.")?,
                        _ => continue,
                    }
                }
            }
        }
    }

    /// Print the previous guesses
    pub fn print_board(&mut self) -> Result<(), io::Error> {
        for (w, v) in self.session.guesses.iter() {
            for (c, lv) in w.chars().into_iter().zip(v) {
                write!(&mut self.writer, "{}", c.to_string().color(*self.color_map.get(lv).unwrap_or_else(|| &Color::White)))?;
            }
            writeln!(&mut self.writer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use colored::{Colorize, Color};

    use crate::{WordleGame};

    use super::WordleSessionCLI;

    #[test]
    fn print_board1() {
        let input = b"";
        let mut output = Vec::new();
        let mut session = WordleSessionCLI::new(&WordleGame { 
            word: String::from("apple"), 
            word_list: vec![String::from("apple"), String::from("grape")], 
            word_len: 5, 
            max_guesses: 2,
        }, input.as_slice(), &mut output);
        session.print_board().expect("Failed to print to output");
        assert!(output.is_empty());
    }

    #[test]
    fn print_board2() {
        let input = b"";
        let mut output = Vec::new();
        let mut session = WordleSessionCLI::new(&WordleGame { 
            word: String::from("apple"), 
            word_list: vec![String::from("apple"), String::from("grape")], 
            word_len: 5, 
            max_guesses: 2,
        }, input.as_slice(), &mut output);
        assert!(matches!(session.session.guess(&String::from("grape")), Result::Ok(_)));
        session.print_board().expect("Failed to print to output");
        let mut expected_output = Vec::new();
        write!(&mut expected_output, "{}", "g".color(Color::BrightWhite)).expect("Failed to write to expected output");
        write!(&mut expected_output, "{}", "r".color(Color::BrightWhite)).expect("Failed to write to expected output");
        write!(&mut expected_output, "{}", "a".color(Color::BrightYellow)).expect("Failed to write to expected output");
        write!(&mut expected_output, "{}", "p".color(Color::BrightYellow)).expect("Failed to write to expected output");
        write!(&mut expected_output, "{}", "e".color(Color::BrightGreen)).expect("Failed to write to expected output");
        writeln!(&mut expected_output).expect("Failed to writeln to expected output");
        
        assert_eq!(String::from_utf8(output).expect("Output not in UTF-8"), 
            String::from_utf8(expected_output).expect("Expected output not in UTF-8"));
    }
}