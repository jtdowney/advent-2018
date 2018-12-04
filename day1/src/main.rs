extern crate failure;
extern crate itertools;

use failure::Error;
use itertools::{FoldWhile, Itertools};
use std::collections::HashSet;
use std::env;
use std::fs;

fn part1(input: &[i32]) {
    let answer: i32 = input.iter().sum();
    println!("part 1: {}", answer);
}

fn part2(input: &[i32]) {
    let (answer, _) = input
        .iter()
        .cycle()
        .fold_while((0, HashSet::new()), |(current, mut seen), value| {
            let current = current + value;
            if seen.insert(current) {
                FoldWhile::Continue((current, seen))
            } else {
                FoldWhile::Done((current, seen))
            }
        }).into_inner();

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
