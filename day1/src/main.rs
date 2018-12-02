extern crate failure;

use failure::Error;
use std::collections::HashSet;
use std::env;
use std::fs;

fn part1(input: &[i32]) {
    let answer: i32 = input.iter().sum();
    println!("part 1: {}", answer);
}

fn part2(input: &[i32]) {
    let mut seen = HashSet::new();
    let answer = input
        .iter()
        .cycle()
        .scan(0, |acc, value| {
            *acc += value;
            Some(*acc)
        }).find_map(|value| seen.replace(value))
        .unwrap();

    println!("part 2: {}", answer);
}

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}
