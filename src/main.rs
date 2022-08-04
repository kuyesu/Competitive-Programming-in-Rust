//import public module game
pub mod game;
//
use std::io;

fn main() {
    println!("Enter A Guess : ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to take in input");
    println!("{guess}");
    game::game::gamer::game();
}
