pub struct Game {
    id: i32,
    rounds: Vec<Hand>
}

impl Game {
    pub fn is_possible<T: Fn(&Color) -> i32>(&self, max_count: T) -> bool {
        for round in &self.rounds {
            if round.amount > max_count(&round.color) {
                return false;
            }
        }
        true
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub struct Hand {
    amount: i32,
    color: Color 
}

impl Hand {
    pub fn new(color: Color, amount: i32) -> Self {
        Hand { amount, color }
    }
}

pub enum Color {
    red,
    green,
    blue
}

#[cfg(test)]
mod test {
    use crate::logic::{Color, Game, Hand};

    #[test]
    fn test_is_possible_big_Game() {
        let game = Game { id: 1, rounds: vec![
            Hand::new(Color::red, 3),
            Hand::new(Color::green, 3),
            Hand::new(Color::blue, 3),
        ]};
        assert!(!game.is_possible(|_| 2));
    }

    #[test]
    fn test_is_possible_small_Game() {
        let game = Game { id: 1, rounds: vec![
            Hand::new(Color::red, 3),
            Hand::new(Color::green, 3),
            Hand::new(Color::blue, 3),
        ]};
        assert!(game.is_possible(|_| 4));
    }
}