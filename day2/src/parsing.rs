use crate::logic::{Game, Hand};
use regex::Regex;

pub trait Parse {
    fn parse(str: &str) -> Option<Self>
    where Self: Sized;
}

impl Parse for Game {
    fn parse(str: &str) -> Option<Self> {
        let re = Regex::new(r"Game (\d*):(.*)").ok()?;
        let matches = re.captures(str)?;
        let id: i32 = matches.get(1)?.as_str().parse().ok()?;
        let rounds: Vec<Hand> = matches.get(2)?.as_str()
            .split(";")
            .map(|h| Hand::parse(h))
            .collect::<Option<Vec<Hand>>>()?;
        return Some(Game::new(id, rounds));
    }
}

impl Parse for Hand {
    fn parse(str: &str) -> Option<Self> {
        let mut hand = Hand::new(0, 0, 0);
        let re = Regex::new(r" (\d*) (\w*)").ok()?;

        let colors: Vec<&str> = str.split(",").collect();
        for color in colors {
            let matches = re.captures(color)?;
            let amount: i32 = matches.get(1)?.as_str().parse().ok()?;
            let color = matches.get(2)?.as_str();
            match color {
                "red" => { hand.red += amount },
                "green" => { hand.green += amount },
                "blue" => { hand.blue += amount },
                _ => { return None }
            }
        }
        Some(hand)
    }
}

#[cfg(test)]
mod test {
    use std::result;

    use crate::{logic::{Game, Hand}, parsing::Parse};

    #[test]
    fn test_parsing() {
        let string = "Game 11: 13 blue, 4 red; 1 red, 2 green, 6 blue, 1 red; 2 green";
        let solution = Game::new(11, vec![
            Hand::new(4, 0, 13),
            Hand::new(2, 2, 6),
            Hand::new(0, 2, 0)
        ]);
        let result = Game::parse(string).unwrap();
        assert_eq!(result, solution);
    }
}