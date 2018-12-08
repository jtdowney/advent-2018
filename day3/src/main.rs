extern crate itertools;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut parts = source.split_whitespace();
        let id = parts.next().unwrap().trim_start_matches('#').parse()?;
        let _ = parts.next();
        let mut position = parts.next().unwrap().trim_end_matches(':').split(',');
        let x = position.next().unwrap().parse()?;
        let y = position.next().unwrap().parse()?;
        let mut size = parts.next().unwrap().split('x');
        let width = size.next().unwrap().parse()?;
        let height = size.next().unwrap().parse()?;

        Ok(Claim {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

fn part1(input: &[Claim]) {
    let answer = input
        .iter()
        .fold(HashMap::new(), |mut acc, claim| {
            (claim.x..claim.x + claim.width)
                .cartesian_product(claim.y..claim.y + claim.height)
                .for_each(|(i, j)| {
                    *acc.entry((i, j)).or_insert(0) += 1;
                });

            acc
        })
        .values()
        .filter(|&n| *n > 1)
        .count();

    println!("part 1: {}", answer);
}

fn part2(input: &[Claim]) {
    let repeated = input
        .iter()
        .fold(HashMap::new(), |mut acc, claim| {
            (claim.x..claim.x + claim.width)
                .cartesian_product(claim.y..claim.y + claim.height)
                .for_each(|(i, j)| {
                    acc.entry((i, j)).or_insert(vec![]).push(claim.id);
                });

            acc
        })
        .values()
        .filter(|claims| claims.len() > 1)
        .flatten()
        .cloned()
        .collect::<HashSet<u16>>();

    let all_claims: HashSet<u16> = input.iter().map(|claim| claim.id).collect();
    let answer = all_claims.difference(&repeated).nth(0).unwrap();

    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Claim>, _>>()
        .expect("Unable to parse input");

    part1(&input);
    part2(&input);
}
