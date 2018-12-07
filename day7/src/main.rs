extern crate failure;

use failure::Error;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn completion_time(step: char) -> usize {
    (step as usize) - 64 + 60
}

fn parse_step(source: &str) -> (char, char) {
    let mut parts = source.split_whitespace();
    let id = parts.nth(1).and_then(|p| p.chars().nth(0)).unwrap();
    let block = parts.nth(5).and_then(|p| p.chars().nth(0)).unwrap();

    (id, block)
}

fn part1(
    step_blocks: &HashMap<char, Vec<char>>,
    step_blocked_by: &HashMap<char, Vec<char>>,
    ready: &HashSet<char>,
) {
    let mut answer = String::new();
    let mut completed = HashSet::new();
    let mut ready = ready.clone();

    while let Some(&step) = ready.iter().min() {
        answer.push(step);
        ready.remove(&step);
        completed.insert(step);

        let children = step_blocks
            .get(&step)
            .map(|c| c.as_slice())
            .unwrap_or_default();
        for &child in children {
            if let Some(parents) = step_blocked_by.get(&child) {
                if parents.iter().all(|p| completed.contains(p)) {
                    ready.insert(child);
                }
            } else {
                ready.insert(child);
            }
        }
    }

    println!("part 1: {}", answer);
}

fn part2(
    step_blocks: &HashMap<char, Vec<char>>,
    step_blocked_by: &HashMap<char, Vec<char>>,
    ready: &HashSet<char>,
) {
    let mut completed = HashSet::new();
    let mut ready = ready.clone();
    let mut workers: [Option<(char, usize)>; 5] = Default::default();

    let max_steps = step_blocks
        .keys()
        .chain(step_blocked_by.keys())
        .collect::<HashSet<&char>>()
        .len();

    for t in 0.. {
        for worker in workers.iter_mut() {
            match worker {
                Some((step, done)) if t == *done => {
                    completed.insert(*step);

                    let children = step_blocks
                        .get(&step)
                        .map(|c| c.as_slice())
                        .unwrap_or_default();
                    for &child in children {
                        if let Some(parents) = step_blocked_by.get(&child) {
                            if parents.iter().all(|p| completed.contains(p)) {
                                ready.insert(child);
                            }
                        } else {
                            ready.insert(child);
                        }
                    }
                }
                None => {}
                _ => continue,
            }

            if let Some(&step) = ready.iter().min() {
                ready.remove(&step);
                let time = t + completion_time(step);
                *worker = Some((step, time));
            } else {
                *worker = None;
            }
        }

        if completed.len() == max_steps {
            println!("part 2: {}", t);
            break;
        }
    }
}

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)?
        .lines()
        .map(parse_step)
        .collect::<Vec<(char, char)>>();

    let mut step_blocks = HashMap::new();
    let mut step_blocked_by = HashMap::new();

    for &(step, block) in &input {
        step_blocks.entry(step).or_insert_with(Vec::new).push(block);
        step_blocked_by
            .entry(block)
            .or_insert_with(Vec::new)
            .push(step);
    }

    let ready = input
        .iter()
        .filter(|(step, _)| !step_blocked_by.contains_key(step))
        .map(|&(step, _)| step)
        .collect::<HashSet<char>>();

    part1(&step_blocks, &step_blocked_by, &ready);
    part2(&step_blocks, &step_blocked_by, &ready);

    Ok(())
}
