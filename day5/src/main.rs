use std::collections::VecDeque;
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

fn fully_react(mut input: VecDeque<char>) -> usize {
    'outer: loop {
        for i in 0..input.len() {
            let current = input[i];
            if let Some(&next) = input.get(i + 1) {
                if current.is_reacting(next) {
                    let _ = input.drain(i..=i + 1);
                    continue 'outer;
                }
            }
        }

        break;
    }

    input.len()
}

fn part1(input: VecDeque<char>) {
    let answer = fully_react(input);
    println!("part 1: {}", answer);
}

fn part2(input: &VecDeque<char>) {
    let answer = (97u8..=122)
        .map(|n| n as char)
        .map(|c| {
            input
                .iter()
                .filter(|&u| *u != c && u.to_ascii_lowercase() != c)
                .cloned()
                .collect::<VecDeque<char>>()
        })
        .map(fully_react)
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
        .collect::<VecDeque<char>>();

    part1(input.clone());
    part2(&input);
}
