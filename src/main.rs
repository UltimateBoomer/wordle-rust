use clap::Parser;

use wordle::{Config};

fn main() {
    let conf = Config::parse();

    let game = wordle::setup(conf).expect("Error initializing game");
    println!("Word: {}", &game.word);
}