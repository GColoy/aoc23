use std::{collections::HashSet, fs};

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?<number>\d+)|(?<symbol>\*)|(?<newline>\n)")
        .expect("Invalid regex pattern");
    let input = fs::read_to_string("input")
        .expect("Failed to read input file");

    let mut set: HashSet<Point> = HashSet::new();
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
                number: c.as_str().parse().expect("Failed to parse number"),
            });
        } else if let Some(c) = capture.name("symbol") {
            set.insert(Point(c.start() - byte_offset, line));
        } else if let Some(c) = capture.name("newline") {
            line += 1;
            byte_offset = c.end();
        }
    }

    let mapped_numbers = map.iter()
        .map(|n| (n.get_adjacent_symbols(&set), n))
        .fold(Vec::<(Point, Vec<InsertNumber>)>::new(), |mut acc, (adj, n)| {
            for point in adj {
                if let Some((_point, arr)) = acc.iter_mut().find(|(symbol, _vec)| *symbol == point) {
                    arr.push(*n);
                } else {
                    acc.push((point, vec![*n]));
                }
            }
            acc
        });
    let sum: usize = mapped_numbers.iter()
        .filter(|(_p, arr)| arr.len() > 1)
        .map(|(_p, arr)| arr.iter().map(|n| n.number).product::<usize>())
        .sum();

    println!("Sum of valid number is  {sum}");
}

#[derive(Debug, Clone, Copy)]
struct InsertNumber {
    start: usize, 
    end: usize,
    line: usize,
    number: usize,
}

impl InsertNumber {
    #[allow(dead_code)]
    fn is_valid(&self, set: &HashSet<Point>) -> bool {
        self.get_adjacents().any(|p| set.contains(&p))
    }

    fn get_adjacent_symbols(&self, set: &HashSet<Point>) -> Vec<Point> {
        self.get_adjacents()
            .filter(|p| set.contains(p))
            .collect()
    }

    fn get_adjacents(&self) -> impl Iterator<Item = Point> + '_ {
        let start = self.start.saturating_sub(1);
        let end = self.end.saturating_add(1);
        (start..end).flat_map(move |x| {
            (-1..=1).filter_map(move |y_delta| {
                self.line.checked_sub_signed(y_delta).map(|y| Point(x, y))
            })
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point(value.0, value.1)
    }
}

#[cfg(test)]
mod test {
    use crate::{InsertNumber, Point};

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
        let solution: Vec<Point> = vec![
            Point(4, 1),
            Point(4, 0),
            Point(5, 1),
            Point(5, 0),
            Point(6, 1),
            Point(6, 0),
            Point(7, 1),
            Point(7, 0),
            Point(8, 1),
            Point(8, 0),
        ];
        let result: Vec<Point> = number.get_adjacents().collect();
        assert_eq!(result, solution);
    }
}
