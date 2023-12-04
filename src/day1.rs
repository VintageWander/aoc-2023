pub fn part1(lines: Vec<String>) {
    let mut result = 0;
    lines.into_iter().for_each(|line| {
        let mut num = 0;
        for ch in line.chars() {
            match ch.to_digit(10) {
                Some(converted) => {
                    num = num + converted;
                    break;
                }
                None => continue,
            }
        }
        for ch in line.chars().rev() {
            match ch.to_digit(10) {
                Some(converted) => {
                    num = num * 10 + converted;
                    break;
                }
                None => continue,
            }
        }
        result = result + num;
    });
    println!("{result}");
}

use std::collections::HashMap;

use fancy_regex::Regex;

pub fn part2(lines: Vec<String>) {
    let regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let rev_regex = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let num_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let rev_num_map = HashMap::from([
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut result = 0;
    lines.into_iter().for_each(|line| {
        let mut num = 0;
        if let Some(found) = regex.find(&line).unwrap() {
            if let Some(converted) = num_map.get(found.as_str()) {
                num += converted;
            }
        }

        let rev_line: String = line.chars().rev().collect();
        if let Some(found) = rev_regex.find(&rev_line).unwrap() {
            if let Some(converted) = rev_num_map.get(found.as_str()) {
                num = num * 10 + converted;
            }
        }
        println!("{num}");
        result += num;
    });
    println!("{result}");
}
