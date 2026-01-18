#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, fs, iter::Scan, str::FromStr};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read string");
    let stack: ScratchCardStack = input.parse().unwrap();
    let score = stack.get_card_amount();
    println!("Score is: {score}")
}

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: HashSet<i32>,
    personal_numbers: Vec<i32>
}

impl ScratchCard {
    fn get_score(&self) -> i32 {
        Self::calculate_points(self.get_win_count())
    }

    fn get_win_count(&self) -> i32 {
        let count = self.personal_numbers.iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
            .try_into()
            .expect("Count bigger than i32::MAX");
        count
    }
    
    fn calculate_points(correct_numbers: i32) -> i32 {
        if correct_numbers > 0 {
            2_i32.pow((correct_numbers - 1).try_into().unwrap_or(0))
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct ScratchCardStack {
    stack: HashMap<CardId, ScratchCard>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CardId(i32);

impl CardId {
    fn get_next_ids(&self, range: i32) -> impl Iterator<Item = CardId> {
        let start = self.0 + 1;
        let end = self.0 + range;
        (start..=end).map(|n| CardId(n))
    }
}

impl ScratchCardStack {
    fn get_score_sum(&self) -> i32 {
        self.stack.iter()
            .map(|(_id, num)| num.get_score())
            .sum()
    }

    fn get_card_amount(&self) -> i32 {
        let mut cache: HashMap<CardId, i32> = HashMap::new();
        self.stack.iter()
            .map(|(id, _card)| self.get_card_amount_of_card(id, &mut cache) )
            .sum()
    }

    fn get_card_amount_of_card(&self, start: &CardId, cache: &mut HashMap<CardId, i32>) -> i32 {
        if let Some(result) = cache.get(&start) { return *result; }
        // println!("Calculating Amount for {start:?}");
        let score = self.stack.get(&start).expect(&format!("Start {start:?} doesn't exist in stack"))
            .get_win_count();
        let result = start.get_next_ids(score)
            .map(|s| self.get_card_amount_of_card(&s, cache))
            .sum::<i32>() + 1;
        cache.insert(*start, result);
        result
    }
}

impl FromStr for ScratchCardStack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let regex = Regex::new(r"(?<digit>\d+)|(?<divider>\|)").ok().ok_or("Regex is wrong")?;
        let mut stack: HashMap<CardId, ScratchCard> = HashMap::new();
        for line in s.lines() {
            let mut captures = regex.captures_iter(line);
            let card_id: i32 = captures.next().ok_or(format!("Inccorect card syntax: {line}"))?
                .name("digit").ok_or(format!("Inccorect card syntax: {line}"))?
                .as_str().parse().ok().ok_or("Should have been number according to regex")?;

            let mut winning_numbers: HashSet<i32> = HashSet::new();
            while let Some(win) = captures.next().ok_or(format!("Inccorect card syntax: {line}"))?.name("digit") {
                winning_numbers.insert(win.as_str().parse().ok().ok_or("Should have been number, according to regex")?);
            }

            let mut personal_numbers: Vec<i32> = Vec::new();
            while let Some(capture) = captures.next() {
                let personal = capture.name("digit").ok_or(format!("Inccorect card syntax: {line}"))?;
                personal_numbers.push(personal.as_str().parse().ok().ok_or("Should have been number, according to regex")?);
            }
            stack.insert(CardId(card_id), ScratchCard { winning_numbers, personal_numbers });
        }
        Ok(ScratchCardStack { stack })
    }
}

#[cfg(test)]
mod test {
    use crate::CardId;

    #[test]
    fn test_get_next_id() {
        let mut ids = CardId(4).get_next_ids(2);
        assert_eq!(Some(CardId(5)), ids.next());
        assert_eq!(Some(CardId(6)), ids.next());
    }
}