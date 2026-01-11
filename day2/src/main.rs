mod logic;
mod parsing;

use std::fs;

use crate::{logic::{Game, Hand}, parsing::Parse};

fn main() {
    let puzzle_input = fs::read_to_string("input").expect("expect file to be read");
    let max_hand = Hand::new(12, 13, 14);

    let games: Vec<Game> = puzzle_input.lines()
        .map(|line| Game::parse(line))
        .collect::<Option<Vec<Game>>>()
        .unwrap();
    let games = games.iter()
        .filter(|g| g.is_possible(&max_hand));
    let sum: i32 = games.map(|g| g.get_id()).sum();
    println!("Sum of possible game id's: {sum}");
}