use std::collections::HashSet;

pub struct ScratchCard {
    winning_numbers: WinningNumbers,
    personal_numbers: PersonalNumbers,
}

impl ScratchCard {
    pub fn get_points(&self) -> i32 {
        let correct = self
            .personal_numbers
            .get_correct_amount(&|n| self.winning_numbers.is_winning(n));
        ScratchCard::calculate_points(correct)
    }

    fn calculate_points(correct_numbers: i32) -> i32 {
        if correct_numbers > 0 {
            2_i32.pow((correct_numbers - 1).try_into().unwrap_or(0))
        } else {
            0
        }
    }
}


struct WinningNumbers {
    nums: HashSet<i32>
}
impl WinningNumbers {
    fn is_winning(&self, number: i32) -> bool {
        self.nums.contains(&number)
    }
}

struct PersonalNumbers {
    nums: Vec<i32>
}
impl PersonalNumbers {
    fn get_correct_amount<F: Fn(i32) -> bool>(&self, is_winning: &F) -> i32 {
        let mut correct = 0;
        for num in &self.nums {
            if is_winning(*num) {
                correct += 1;
            }
        };
        correct
    }
}

#[cfg(test)]
mod test {
    use crate::logic::ScratchCard;

    #[test]
    fn test_point_calculation() {
        assert_eq!(ScratchCard::calculate_points(0), 0);
        assert_eq!(ScratchCard::calculate_points(1), 1);
        assert_eq!(ScratchCard::calculate_points(2), 2);
        assert_eq!(ScratchCard::calculate_points(3), 4);
        assert_eq!(ScratchCard::calculate_points(4), 8);
    }
}