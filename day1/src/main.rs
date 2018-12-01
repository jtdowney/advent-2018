extern crate failure;

use failure::Error;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Default)]
struct ScanState {
    current: i32,
    seen: HashSet<i32>,
    done: bool,
}

fn part1(frequencies: &[i32]) {
    let answer: i32 = frequencies.iter().sum();
    println!("part 1: {}", answer);
}

fn part2(frequencies: &[i32]) {
    let answer = frequencies
        .iter()
        .cycle()
        .scan(ScanState::default(), |state, &value| {
            if state.done {
                return None;
            }

            state.current += value;
            state.done = !state.seen.insert(state.current);
            Some(state.current)
        }).last()
        .unwrap();

    println!("part 2: {}", answer);
}

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("No file provided");
    let frequencies = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    part1(&frequencies);
    part2(&frequencies);

    Ok(())
}
