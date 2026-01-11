#[derive(PartialEq, Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Hand>
}

impl Game {
    pub fn new(id: i32, rounds: Vec<Hand>) -> Game {
        Game {id, rounds}
    }

    pub fn is_possible(&self, max_count: &Hand) -> bool {
        self.rounds.iter().all(|h| h.is_possible(&max_count))
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_min_hand(&self) -> Hand {
        self.rounds.iter().fold(Hand::new(0, 0, 0), |acc, h| acc.min_possible(h))
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Hand {
    pub(super) red: i32,
    pub(super) green: i32, 
    pub(super) blue: i32
}

impl Hand {
    pub fn new(red: i32, green: i32, blue: i32) -> Self {
        Hand { red, green, blue }
    }
    
    fn is_possible(&self, max_count: &Hand) -> bool {
        if self.red > max_count.red { return false; }
        if self.green > max_count.green { return false; }
        if self.blue > max_count.blue { return false; }
        true
    }

    fn min_possible(&self, min_count: &Hand) -> Self {
        let mut hand = self.clone();
        if self.red < min_count.red { hand.red = min_count.red }
        if self.green < min_count.green { hand.green = min_count.green }
        if self.blue < min_count.blue { hand.blue = min_count.blue }
        hand
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

    mod min_possible {
        use crate::logic::{Game, Hand};

        #[test]
        fn test_min_possible() {
            let hand = Hand::new(2, 3, 1);
            let min_count = Hand::new(3, 2, 2);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(3, 3, 2));
        }

        #[test]
        fn test_min_possible_all_below() {
            let hand = Hand::new(1, 1, 1);
            let min_count = Hand::new(5, 5, 5);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(5, 5, 5));
        }

        #[test]
        fn test_min_possible_all_above() {
            let hand = Hand::new(10, 10, 10);
            let min_count = Hand::new(2, 2, 2);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(10, 10, 10));
        }

        #[test]
        fn test_min_possible_mixed() {
            let hand = Hand::new(4, 2, 6);
            let min_count = Hand::new(3, 5, 2);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(4, 5, 6));
        }
        #[test]
        fn test_min_possible_single_value() {
            let hand = Hand::new(5, 5, 5);
            let min_count = Hand::new(5, 5, 5);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(5, 5, 5));
        }

        #[test]
        fn test_min_possible_zero_values() {
            let hand = Hand::new(0, 0, 0);
            let min_count = Hand::new(1, 1, 1);
            let result = hand.min_possible(&min_count);
            assert_eq!(result, Hand::new(1, 1, 1));
        }

        #[test]
        fn test_get_min_hand_single_round() {
        let game = Game::new(1, vec![Hand::new(3, 4, 2)]);
        assert_eq!(game.get_min_hand(), Hand::new(3, 4, 2));
        }

        #[test]
        fn test_get_min_hand_multiple_rounds() {
        let game = Game::new(1, vec![
            Hand::new(3, 4, 2),
            Hand::new(6, 2, 5),
            Hand::new(2, 3, 1),
        ]);
        assert_eq!(game.get_min_hand(), Hand::new(6, 4, 5));
        }

        #[test]
        fn test_get_min_hand_all_zeros() {
            let game = Game::new(1, vec![
                Hand::new(0, 0, 0),
                Hand::new(0, 0, 0),
            ]);
            assert_eq!(game.get_min_hand(), Hand::new(0, 0, 0));
        }
    }
}