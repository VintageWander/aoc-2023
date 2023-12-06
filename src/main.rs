#![allow(dead_code, unused_variables, unused_imports)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use fancy_regex::Regex;
mod day1;
mod day2;

fn main() {
    let file = File::open("day2_1.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().into_iter().flatten().collect();

    // day1::part1(lines);
    // day1::part2(lines);
    day2::part1(lines);

    // let test_regex = Regex::new(r"\w*(?= green)").unwrap();
    // let mut test = test_regex.find_iter("Game 33: 12 red, 4 green, 11 blue; 4 blue, 10 red, 1 green; 7 green, 10 red, 16 blue; 15 red, 5 blue; 10 green, 4 red; 8 green, 5 blue, 6 red");
    // while let Some(matching) = test.next() {
    //     println!(
    //         "{:?}",
    //         matching
    //             .unwrap()
    //             .as_str()
    //             .to_string()
    //             .parse::<i32>()
    //             .unwrap()
    //     );
    // }
}
