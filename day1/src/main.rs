use std::path::Path;
use std::io;
use std::fs;

fn main() {
    let calibrations: i32 = read_input(Path::new("input"))
        .expect("File should be readable")
        .lines()
        .map(find_calibration)
        .map(|r| r.expect("calibration should be found"))
        .sum();
    fs::write("output", calibrations.to_string()).expect("expected to write");
}

fn read_input(file: &Path) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

fn find_calibration(line: &str) -> Option<i32> {
    let line = replace_digits(line);
    let numbers: Vec<char> = line.chars()
        .filter(|c| c.is_digit(10))
        .collect();
    let first = numbers.first().expect("expect to find number in string").to_string();
    let second = numbers.last().expect("expect to find number in string").to_string();
    let combined = format!("{}{}", first, second);
    let output: i32 = combined.parse().expect("expect first to be a number");
    Some(output)
}

fn replace_digits(line: &str) -> String {
    let mut line = line.to_lowercase();
    let replacements = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];
    for (from, to) in replacements {
        line = line.replace(from, to);
    }
    line
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{find_calibration, read_input};

    #[test]
    fn test_example_calibrations() {
        let tests = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("two1nine", 29),
            ("eightwothree", 83)
        ];
        for (line, solution) in tests {
            assert_eq!(find_calibration(line), Some(solution));
        };
    }

    #[test]
    fn test_file_import() {
        let text = String::from("Some Test\nhahah");
        let file_path = Path::new("testData.txt");
        let result= read_input(file_path).unwrap();
        println!("result: {}", result);
        assert_eq!(result, text)
    }
}