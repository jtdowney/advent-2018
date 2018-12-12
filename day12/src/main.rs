use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn calculate(rules: &HashMap<String, char>, state: &HashSet<isize>) -> HashSet<isize> {
    let start = state.iter().min().unwrap() - 1;
    let end = state.iter().max().unwrap() + 1;
    (start..=end)
        .filter(|i| {
            let lookup = (i - 2..=i + 2)
                .map(|j| state.get(&j).cloned().map(|_| '#').unwrap_or('.'))
                .collect::<String>();
            match rules.get(&lookup) {
                Some(&next) => next == '#',
                None => false,
            }
        })
        .collect()
}

fn part1(rules: &HashMap<String, char>, initial_state: &HashSet<isize>) {
    let state = (0..20).fold(initial_state.clone(), |last_state, _| {
        calculate(rules, &last_state)
    });

    let answer = state.iter().sum::<isize>();

    println!("part 1: {}", answer);
}

fn part2(rules: &HashMap<String, char>, initial_state: &HashSet<isize>) {
    let (generation, growth, sum, _) = (1..)
        .try_fold(
            (0, 0, 0, initial_state.clone()),
            |(_, last_growth, last_sum, last_state), g| {
                let state = calculate(rules, &last_state);
                let sum = state.iter().sum::<isize>();
                let growth = sum - last_sum;
                if last_growth == growth {
                    Err((g, growth, sum, state))
                } else {
                    Ok((g, growth, sum, state))
                }
            },
        )
        .unwrap_err();

    let rest = (50_000_000_000isize - generation) * growth;
    let answer = sum + rest;

    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let data = fs::read_to_string(filename).expect("File to read");
    let mut lines = data.lines();

    let initial_state = lines
        .next()
        .and_then(|line| {
            line.split(": ").nth(1).map(|state| {
                state
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '#' { Some(i as isize) } else { None })
                    .collect::<HashSet<isize>>()
            })
        })
        .unwrap();
    let _ = lines.next();
    let rules = lines
        .map(|line| {
            let mut parts = line.split(" => ");
            let matches = parts.next().unwrap().into();
            let next = parts.next().and_then(|s| s.chars().nth(0)).unwrap();
            (matches, next)
        })
        .collect::<HashMap<String, char>>();

    part1(&rules, &initial_state);
    part2(&rules, &initial_state);
}
