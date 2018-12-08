extern crate itertools;

use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
struct Point(isize, isize);

impl Point {
    fn distance(&self, Point(ox, oy): Point) -> isize {
        let &Point(sx, sy) = self;
        (sx - ox).abs() + (sy - oy).abs()
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut parts = source.split(',');
        let x = parts.next().unwrap().trim().parse()?;
        let y = parts.next().unwrap().trim().parse()?;

        Ok(Point(x, y))
    }
}

fn part1(input: &[Point]) {
    let (startx, starty, endx, endy) = input.iter().fold(
        (
            isize::max_value(),
            isize::max_value(),
            isize::min_value(),
            isize::min_value(),
        ),
        |(sx, sy, ex, ey), &Point(x, y)| (x.min(sx), y.min(sy), x.max(ex), y.max(ey)),
    );

    let grid = iproduct!(startx..=endx, starty..=endy)
        .map(|(x, y)| Point(x, y))
        .map(|point| {
            let closest = input
                .iter()
                .cloned()
                .map(|p| (p, point.distance(p)))
                .sorted_by_key(|&(_, d)| d);

            match (closest[0], closest[1]) {
                ((_, d1), (_, d2)) if d1 == d2 => (point, None),
                ((p, _), _) => (point, Some(p)),
            }
        })
        .collect::<HashMap<Point, Option<Point>>>();

    let edge_points = grid
        .iter()
        .filter(|&(Point(x, y), _)| *x == startx || *x == endx || *y == starty || *y == endy)
        .filter_map(|(_, &point)| point)
        .collect::<HashSet<Point>>();

    let (_, answer) = grid
        .values()
        .filter_map(|&point| point)
        .filter(|point| !edge_points.contains(point))
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    println!("part 1: {}", answer);
}

fn part2(input: &[Point]) {
    let (startx, starty, endx, endy) = input.iter().fold(
        (
            isize::max_value(),
            isize::max_value(),
            isize::min_value(),
            isize::min_value(),
        ),
        |(sx, sy, ex, ey), &Point(x, y)| (x.min(sx), y.min(sy), x.max(ex), y.max(ey)),
    );

    let answer = iproduct!(startx..=endx, starty..=endy)
        .map(|(x, y)| Point(x, y))
        .filter(|point| input.iter().map(|&p| point.distance(p)).sum::<isize>() < 10000)
        .count();

    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, _>>()
        .expect("Unable to parse input");

    part1(&input);
    part2(&input);
}
