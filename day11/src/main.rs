extern crate itertools;

use itertools::iproduct;
use std::collections::HashMap;

const INPUT: usize = 5535;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point(usize, usize);

struct Scorer<'a> {
    grid: &'a HashMap<Point, isize>,
    cache: HashMap<(Point, usize), isize>,
}

impl<'a> Scorer<'a> {
    fn new(grid: &'a HashMap<Point, isize>) -> Scorer<'a> {
        Scorer {
            grid,
            cache: HashMap::new(),
        }
    }

    fn score(&mut self, point: Point, size: usize) -> Option<isize> {
        if let Some(&score) = self.cache.get(&(point, size)) {
            return Some(score);
        }

        if size == 1 {
            return self.grid.get(&point).cloned();
        }

        let parts = partition_space(point, size);
        let score = parts
            .iter()
            .map(|&(point, size)| self.score(point, size))
            .collect::<Option<Vec<isize>>>()?
            .iter()
            .sum();
        self.cache.insert((point, size), score);

        Some(score)
    }
}

fn power_level(Point(x, y): Point) -> isize {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + INPUT;
    let power_level = power_level * rack_id;
    let hundreds = power_level
        .to_string()
        .chars()
        .rev()
        .nth(2)
        .map(|c| c as u8 - 48)
        .unwrap_or_default() as isize;
    hundreds - 5
}

fn partition_space(point: Point, size: usize) -> Vec<(Point, usize)> {
    if size == 1 {
        return vec![(point, size)];
    }

    let mut parts = vec![];
    let Point(x, y) = point;
    let left = size / 2;
    let right = size - left;
    parts.push((point, left));
    parts.push((Point(x + left, y + left), right));

    if left != right {
        let min = left.min(right);
        let max = left.max(right);
        let edge = max - min;
        parts.push((Point(x, y + left), min));
        parts.push((Point(x + left, y), min));

        for i in 0..min {
            parts.push((Point(x + i, y + left + min), edge));
            parts.push((Point(x + left + min, y + i), edge));
        }
    } else {
        parts.push((Point(x, y + left), right));
        parts.push((Point(x + left, y), right));
    }

    parts
}

fn create_grid() -> HashMap<Point, isize> {
    iproduct!(1..=300, 1..=300)
        .map(|(x, y)| {
            let point = Point(x, y);
            let power_level = power_level(point);
            (point, power_level)
        })
        .collect()
}

fn part1(scorer: &mut Scorer) {
    let (Point(x, y), _) = iproduct!(1..=300, 1..=300)
        .map(|(x, y)| Point(x, y))
        .filter_map(|start| {
            let score = scorer.score(start, 3)?;
            Some((start, score))
        })
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("part 1: {},{}", x, y);
}

fn part2(scorer: &mut Scorer) {
    let (Point(x, y), size, _) = iproduct!(1..=300, 1..=300, 1..=300)
        .map(|(x, y, size)| (Point(x, y), size))
        .filter_map(|(start, size)| {
            let score = scorer.score(start, size)?;
            Some((start, size, score))
        })
        .max_by_key(|&(_, _, score)| score)
        .unwrap();

    println!("part 2: {},{},{}", x, y, size);
}

fn main() {
    let grid = create_grid();
    let mut scorer = Scorer::new(&grid);

    part1(&mut scorer);
    part2(&mut scorer);
}
