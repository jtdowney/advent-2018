use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
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
    type Err = ParseIntError;

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

fn part1(schedule: &HashMap<u16, HashMap<u8, usize>>) {
    let (&guard, counts) = schedule
        .iter()
        .max_by_key(|(_, counts)| counts.values().sum::<usize>())
        .unwrap();

    let (&minute, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap();

    let answer = u32::from(guard) * u32::from(minute);
    println!("part 1: {}", answer);
}

fn part2(schedule: &HashMap<u16, HashMap<u8, usize>>) {
    let (&guard, counts) = schedule
        .iter()
        .max_by_key(|(_, counts)| counts.values().max())
        .unwrap();

    let (&minute, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap();

    let answer = u32::from(guard) * u32::from(minute);
    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let mut input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, _>>()
        .expect("Unable to parse input");

    input.sort_by_key(|e| (e.year, e.month, e.day, e.hour, e.minute));

    let (schedule, _, _) = input.iter().fold(
        (HashMap::new(), 0, 0),
        |(mut acc, current_guard, start_sleep), event| match event.action {
            Action::StartShift(guard) => (acc, guard, 0),
            Action::FallAsleep => (acc, current_guard, event.minute),
            Action::WakeUp => {
                for minute in start_sleep..event.minute {
                    *acc.entry(current_guard)
                        .or_insert_with(HashMap::new)
                        .entry(minute)
                        .or_insert(0) += 1;
                }

                (acc, current_guard, 0)
            }
        },
    );

    part1(&schedule);
    part2(&schedule);
}
