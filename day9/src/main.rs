extern crate regex;

use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn play(players: usize, marbles: usize) -> HashMap<usize, usize> {
    let mut scores = HashMap::new();
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_front(0);

    for (marble, player) in (1..=marbles).zip((1..=players).cycle()) {
        if marble % 23 == 0 {
            let mut tail = circle.split_off(circle.len() - 7);
            let scored = tail.pop_front().unwrap();

            for &item in tail.iter().rev() {
                circle.push_front(item);
            }

            *scores.entry(player).or_default() += marble + scored;
        } else {
            for _ in 0..2 {
                let current = circle.pop_front().unwrap();
                circle.push_back(current);
            }

            circle.push_front(marble);
        }
    }

    scores
}

fn part1(players: usize, marbles: usize) {
    let scores = play(players, marbles);
    let answer = scores.values().max().unwrap();
    println!("part 1: {}", answer);
}

fn part2(players: usize, marbles: usize) {
    let scores = play(players, marbles * 100);
    let answer = scores.values().max().unwrap();
    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename).expect("File to read");
    let re =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points").expect("Compiled regex");
    let captures = re.captures(&input).expect("Input to match regex");
    let players = captures[1].parse().expect("Valid number of players");
    let marbles = captures[2].parse().expect("Valid number of points");

    part1(players, marbles);
    part2(players, marbles);
}
