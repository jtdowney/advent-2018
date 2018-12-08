extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;

fn part1(input: &[String]) {
    let (two_count, three_count) = input
        .iter()
        .map(|item| {
            item.chars().fold(HashMap::new(), |mut freqs, c| {
                *freqs.entry(c).or_insert(0) += 1;
                freqs
            })
        })
        .map(|freqs| {
            let two = freqs.values().find(|&n| *n == 2).map(|_| 1).unwrap_or(0);
            let three = freqs.values().find(|&n| *n == 3).map(|_| 1).unwrap_or(0);
            (two, three)
        })
        .fold((0, 0), |(two, three), (x, y)| (two + x, three + y));

    let answer = two_count * three_count;
    println!("part 1: {}", answer);
}

fn part2(input: &[String]) {
    let answer: String = input
        .iter()
        .tuple_combinations()
        .filter(|(item1, item2)| item1 != item2)
        .find(|(item1, item2)| {
            let edits = item1
                .chars()
                .zip(item2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            edits == 1
        })
        .map(|(item1, item2)| {
            item1
                .chars()
                .zip(item2.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap();

    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    part1(&input);
    part2(&input);
}
