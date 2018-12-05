extern crate itertools;

use itertools::Itertools;
use std::collections::LinkedList;
use std::env;
use std::fs;

trait PolymerUnit {
    fn is_reacting(&self, other: Self) -> bool;
}

impl PolymerUnit for char {
    fn is_reacting(&self, other: Self) -> bool {
        if self.is_ascii_uppercase() {
            self.to_ascii_lowercase() == other
        } else {
            self.to_ascii_uppercase() == other
        }
    }
}

fn fully_react(input: &[char]) -> usize {
    let mut input: LinkedList<char> = input.iter().cloned().collect();

    while let Some(p) = input
        .iter()
        .tuple_windows()
        .position(|(a, b)| a.is_reacting(*b))
    {
        let mut rest = input.split_off(p);
        let _ = rest.pop_front();
        let _ = rest.pop_front();
        input.append(&mut rest);
    }

    input.len()
}

fn part1(input: &[char]) {
    let answer = fully_react(input);
    println!("part 1: {}", answer);
}

fn part2(input: &[char]) {
    let answer = (97u8..=122)
        .map(|n| n as char)
        .map(|c| {
            input
                .to_owned()
                .into_iter()
                .filter(|&u| u != c && u.to_ascii_lowercase() != c)
                .collect::<Vec<char>>()
        }).map(|input| fully_react(&input))
        .min()
        .unwrap();
    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("Unable to read file")
        .trim()
        .chars()
        .collect::<Vec<char>>();

    part1(&input);
    part2(&input);
}
