extern crate failure;
extern crate itertools;

use failure::Error;
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
        }).map(|freqs| {
            let two = freqs.values().find(|&n| *n == 2).map(|_| 1).unwrap_or(0);
            let three = freqs.values().find(|&n| *n == 3).map(|_| 1).unwrap_or(0);
            (two, three)
        }).fold((0, 0), |(two, three), (x, y)| (two + x, three + y));

    let answer = two_count * three_count;
    println!("part 1: {}", answer);
}

fn part2(input: &[String]) {
    let item_chars = input
        .iter()
        .map(|item| item.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let answer: String = item_chars
        .iter()
        .cartesian_product(item_chars.iter())
        .filter(|(item1, item2)| item1 != item2)
        .find_map(|(item1, item2)| {
            let edits = item1
                .iter()
                .zip(item2.iter())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            if edits == 1 {
                let common = item1
                    .iter()
                    .zip(item2.iter())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect();
                Some(common)
            } else {
                None
            }
        }).unwrap();

    println!("part 2: {}", answer);
}

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    part1(&input);
    part2(&input);

    Ok(())
}
