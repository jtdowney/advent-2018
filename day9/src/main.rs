extern crate regex;

use regex::Regex;
use std::collections::{HashMap, LinkedList};
use std::env;
use std::fs;

fn play(players: usize, last_points: usize) -> HashMap<usize, usize> {
    let mut circle = [0].iter().cloned().collect::<LinkedList<usize>>();
    let mut scores = HashMap::new();

    for (marble, player) in (1..=last_points).zip((1..=players).cycle()) {
        match marble {
            m if m % 23 == 0 => {
                let mut tail = circle.split_off(circle.len() - 7);
                let other = tail.pop_front().unwrap();
                tail.append(&mut circle);
                circle = tail;

                *scores.entry(player).or_default() += m + other;
            }
            _ => {
                let current = circle.pop_front().unwrap();
                let next = circle.pop_front();
                circle.push_front(marble);
                circle.push_back(current);
                if let Some(next) = next {
                    circle.push_back(next);
                }
            }
        }
    }

    scores
}

fn part1(players: usize, last_points: usize) {
    let scores = play(players, last_points);
    let answer = scores.values().max().unwrap();
    println!("part 1: {}", answer);
}

fn part2(players: usize, last_points: usize) {
    let scores = play(players, last_points * 100);
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
    let last_points = captures[2].parse().expect("Valid number of points");

    part1(players, last_points);
    part2(players, last_points);
}
