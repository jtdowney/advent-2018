extern crate failure;

use failure::Error;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Action {
    StartShift(u16),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Copy, Clone)]
struct Event {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: Action,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut parts = source.split_whitespace();

        let mut date_parts = parts.next().unwrap().trim_start_matches('[').split('-');
        let year = date_parts.next().unwrap().parse()?;
        let month = date_parts.next().unwrap().parse()?;
        let day = date_parts.next().unwrap().parse()?;

        let mut time_parts = parts.next().unwrap().trim_end_matches(']').split(':');
        let hour = time_parts.next().unwrap().parse()?;
        let minute = time_parts.next().unwrap().parse()?;

        let action = match parts.next().unwrap() {
            "Guard" => {
                let id = parts.next().unwrap().trim_start_matches('#').parse()?;
                Action::StartShift(id)
            }
            "falls" => Action::FallAsleep,
            "wakes" => Action::WakeUp,
            _ => unimplemented!(),
        };

        Ok(Event {
            year,
            month,
            day,
            hour,
            minute,
            action,
        })
    }
}

fn part1(input: &[(u16, Range<u8>)]) {
    let (target, _) = input
        .iter()
        .fold(HashMap::new(), |mut acc, (guard, sleep_time)| {
            *acc.entry(*guard).or_insert(0) +=
                u16::from(sleep_time.end) - u16::from(sleep_time.start);
            acc
        }).into_iter()
        .max_by_key(|&(_, sleep_time)| sleep_time)
        .unwrap();

    let shifts = input
        .iter()
        .filter(|(guard, _)| *guard == target)
        .collect::<Vec<_>>();

    let (minute, _) = (0..60)
        .fold(HashMap::new(), |mut acc, minute| {
            *acc.entry(minute).or_insert(0) += shifts
                .iter()
                .filter(|(_, sleep_time)| minute >= sleep_time.start && minute < sleep_time.end)
                .count();
            acc
        }).into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    let answer = u32::from(target) * u32::from(minute);
    println!("part 1: {}", answer);
}

fn part2(input: &[(u16, Range<u8>)]) {
    let ((target, minute), _) = (0..60)
        .fold(HashMap::new(), |mut acc, minute| {
            input
                .iter()
                .filter(|(_, sleep_time)| minute >= sleep_time.start && minute < sleep_time.end)
                .for_each(|&(guard, _)| {
                    *acc.entry((guard, minute)).or_insert(0) += 1;
                });

            acc
        }).into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    let answer = u32::from(target) * u32::from(minute);
    println!("part 2: {}", answer);
}

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("No file provided");
    let mut input = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, _>>()?;

    input.sort_by_key(|e| (e.year, e.month, e.day, e.hour, e.minute));

    let (schedule, _, _) = input.iter().fold(
        (vec![], 0, 0),
        |(mut acc, current_guard, start_sleep), event| match event.action {
            Action::StartShift(guard) => (acc, guard, 0),
            Action::FallAsleep => (acc, current_guard, event.minute),
            Action::WakeUp => {
                acc.push((current_guard, start_sleep..event.minute));
                (acc, current_guard, 0)
            }
        },
    );

    part1(&schedule);
    part2(&schedule);

    Ok(())
}
