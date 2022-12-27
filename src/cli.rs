/// CLI backend for Wordle.
use std::{io::{self, Write, BufRead}, collections::HashMap, fmt};

use termion::{color, style};

use crate::{WordleSession, WordleGame, LetterValidity, GuessResult, GameResult};

pub struct WordleSessionCLI<R, W> {
    session: WordleSession,
    reader: R,
    writer: W,
    color_map: HashMap<LetterValidity, Box<dyn fmt::Display>>
}

impl<R: BufRead, W: Write> WordleSessionCLI<R, W> {
    /// Create a `WordleSessionCLI` in starting state.
    pub fn new(game: &WordleGame, reader: R, writer: W) -> WordleSessionCLI<R, W> {
        WordleSessionCLI { 
            session: WordleSession::new(&game),
            reader: reader,
            writer: writer,
            color_map: HashMap::from([
                (LetterValidity::Correct, Box::new(color::Fg(color::LightGreen)) as Box<dyn fmt::Display>),
                (LetterValidity::Incorrect, Box::new(color::Fg(color::LightWhite))),
                (LetterValidity::WrongPos, Box::new(color::Fg(color::LightYellow))),
            ]),
        }   
    }

    /// Run the Wordle game.
    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            write!(&mut self.writer, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
            self.print_board()?;
            
            loop {
                writeln!(&mut self.writer, "Enter your word:")?;
                
                let mut input = String::new();
                self.reader.read_line(&mut input)?;
                let input = input.trim().to_string();

                write!(&mut self.writer, "\r")?;

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
    fn print_board(&mut self) -> Result<(), io::Error> {
        for (w, v) in self.session.guesses.iter() {
            for (c, lv) in w.chars().into_iter().zip(v) {
                write!(&mut self.writer, "{}{}", self.color_map.get(lv).unwrap(), c.to_string())?;
            }
            writeln!(&mut self.writer, "{}", style::Reset)?;
        }
        // Print spaces for remaining attempts
        for _ in self.session.guesses.len()..(self.session.game.max_guesses as usize) {
            writeln!(&mut self.writer, "{}", "·".repeat(self.session.game.word_len))?;
        }   

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use termion::{color, style};

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
            max_guesses: 3,
        }, input.as_slice(), &mut output);
        session.print_board().expect("Failed to print to output");
        let mut expected_output = Vec::new();
        for _ in 0..3 {
            writeln!(&mut expected_output, "·····").expect("Failed to write to expected output");
        }
        assert_eq!(String::from_utf8(output).expect("Output not in UTF-8"), 
            String::from_utf8(expected_output).expect("Expected output not in UTF-8"));
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
        writeln!(&mut expected_output, "{}g{}r{}a{}p{}e{}", 
            color::Fg(color::LightWhite), 
            color::Fg(color::LightWhite), 
            color::Fg(color::LightYellow), 
            color::Fg(color::LightYellow), 
            color::Fg(color::LightGreen),
            style::Reset).expect("Failed to write to expected output");
        writeln!(&mut expected_output, "·····").expect("Failed to write to expected output");
        
        assert_eq!(String::from_utf8(output).expect("Output not in UTF-8"), 
            String::from_utf8(expected_output).expect("Expected output not in UTF-8"));
    }
}