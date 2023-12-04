#![allow(dead_code, unused_variables)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};
mod day1;

fn main() {
    let file = File::open("input2.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().into_iter().flatten().collect();

    // day1::part1(lines);
    day1::part2(lines);
}
