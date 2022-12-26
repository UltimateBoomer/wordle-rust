use std::io;

use clap::Parser;

use wordle::{Config, WordleGame, cli::WordleSessionCLI};

fn main() {
    let conf = Config::parse();

    let game = WordleGame::from_config(&conf).expect("Error initializing game");
    println!("Word: {}", &game.word);    

    let input = io::stdin().lock();
    let output = io::stdout();
    let mut session = WordleSessionCLI::new(&game, input, output);
    session.run().expect("Error in Wordle session");
}