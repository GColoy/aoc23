mod logic;
mod parsing;

use std::fs;

use crate::{logic::{Color, Game, Hand}, parsing::Parse};

fn main() {
    let puzzle_input = fs::read_to_string("input").expect("expect file to be read");
    let games: Vec<Game> = puzzle_input.lines()
        .map(|line| Game::parse(line))
        .collect();
    let games = games.iter()
        .filter(|g| g.is_possible(max_count));
    let sum: i32 = games.map(|g| g.get_id()).sum();
    println!("Sum of possible game id's: {sum}");
}

fn max_count(color: &Color) -> i32 {
    match color {
        Color::red => 12,
        Color::green => 13,
        Color::blue => 14,
    }
}