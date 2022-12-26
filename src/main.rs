use clap::Parser;

use wordle::{Config, WordleGame};

fn main() {
    let conf = Config::parse();

    let game = WordleGame::new(conf).expect("Error initializing game");
    println!("Word: {}", &game.word);
}