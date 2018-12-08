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
        .try_fold((0, HashSet::new()), |(current, mut seen), value| {
            let current = current + value;
            if seen.insert(current) {
                Ok((current, seen))
            } else {
                Err((current, seen))
            }
        })
        .unwrap_err();

    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<i32>, _>>()
        .expect("Unable to parse input");

    part1(&input);
    part2(&input);
}
