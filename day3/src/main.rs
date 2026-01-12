use std::{collections::HashSet, fs, usize};

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?<number>\d+)|(?<symbol>[^\d\.\n])|(?<newline>\n)").unwrap();
    let input = fs::read_to_string("input").unwrap();

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut map: Vec<InsertNumber> = Vec::new();

    let captures = re.captures_iter(&input);
    let mut line: usize = 0;
    let mut byte_offset = 0;
    for capture in captures {
        if let Some(c) = capture.name("number") {
            map.push(InsertNumber {
                start: c.start() - byte_offset,
                end: c.end() - byte_offset,
                line,
                number: c.as_str().parse().unwrap(),
            });
        }
        if let Some(c) = capture.name("symbol") {
            set.insert((c.start() - byte_offset, line));
        }
        if let Some(c) = capture.name("newline") {
            line += 1;
            byte_offset = c.end();
        }
    }


    let filter = map.iter().filter(|n| n.is_valid(&set));
    let valid_sum: usize = filter.fold(0, |acc, n| acc + n.get_number());

    println!("Sum of valid number is  {valid_sum}");
}

#[derive(Debug)]
struct InsertNumber {
    start: usize,
    end: usize,
    line: usize,
    number: usize,
}

impl InsertNumber {
    fn is_valid(&self, set: &HashSet<(usize, usize)>) -> bool {
        self.get_adjacents().iter().any(|p| set.contains(p))
    }

    fn get_adjacents(&self) -> Vec<(usize, usize)> {
        let mut vec: Vec<(usize, usize)> = Vec::new();
        let start = self.start.saturating_sub(1);
        let end = self.end.saturating_add(1);
        for x in start..end {
            for y_delta in -1..=1 {
                if let Some(y) = self.line.checked_sub_signed(y_delta) {
                    vec.push((x, y));
                }
            }
        }
        vec
    }

    fn get_number(&self) -> usize {
        self.number
    }
}

#[cfg(test)]
mod test {
    use crate::InsertNumber;

    #[test]
    fn basic_test() {
        for num in -1..=3 {
            println!("{}", num);
        }
    }

    #[test]
    fn test_get_adjacents() {
        let number = InsertNumber {
            start: 5,
            end: 8,
            line: 0,
            number: 114,
        };
        let solution: Vec<(usize, usize)> = vec![
            (4, 1),
            (4, 0),
            (5, 1),
            (5, 0),
            (6, 1),
            (6, 0),
            (7, 1),
            (7, 0),
            (8, 1),
            (8, 0),
        ];
        let result = number.get_adjacents();
        assert_eq!(result, solution);
    }
}
