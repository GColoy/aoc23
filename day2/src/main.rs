mod logic;
mod parsing;

use std::fs;

use crate::{logic::{Game, Hand}, parsing::Parse};

fn main() {
    let puzzle_input = fs::read_to_string("input").expect("expect file to be read");
    let games: Vec<Game> = puzzle_input.lines()
        .map(|line| Game::parse(line))
        .collect::<Option<Vec<Game>>>()
        .unwrap();

    task1(&games);
    task2(&games);
}

fn task1(games: &Vec<Game>) {
    let max_hand = Hand::new(12, 13, 14);
    let games = games.iter()
        .filter(|g| g.is_possible(&max_hand));
    let sum: i32 = games.map(|g| g.get_id()).sum();
    println!("Sum of possible game id's: {sum}");
}

fn task2(games: &Vec<Game>) {
    let power: i32 = games.iter()
        .map(|g| g.get_min_hand().get_power())
        .sum();
    println!("Sum of minimal game powers: {power}")
}