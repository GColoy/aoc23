pub struct Game {
    id: i32,
    rounds: Vec<Hand>
}

impl Game {
    pub fn is_possible(&self, max_count: &Hand) -> bool {
        self.rounds.iter().all(|h| h.is_possible(&max_count))
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub struct Hand {
    red: i32,
    green: i32, 
    blue: i32
}

impl Hand {
    pub fn new(red: i32, green: i32, blue: i32) -> Self {
        Hand { red, green, blue }
    }
    
    fn is_possible(&self, max_count: &Hand) -> bool {
        if self.red < max_count.red { return false; }
        if self.green < max_count.green { return false; }
        if self.blue < max_count.blue { return false; }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::logic::{Game, Hand};

    #[test]
    fn test_is_possible_big_Game() {
        let game = Game { id: 1, rounds: vec![
            Hand::new(3, 3, 3),
        ]};
        assert!(!game.is_possible(&Hand::new(2, 2, 2)));
    }

    #[test]
    fn test_is_possible_small_Game() {
        let game = Game { id: 1, rounds: vec![
            Hand::new(3, 3, 3),
        ]};
        assert!(game.is_possible(&Hand::new(4, 4, 4)));
    }
}