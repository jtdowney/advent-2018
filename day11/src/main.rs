extern crate itertools;

use itertools::iproduct;
use std::collections::HashMap;

const INPUT: isize = 5535;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point(isize, isize);

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

fn part1(grid: &HashMap<Point, isize>) {
    let (Point(x, y), _) = grid
        .keys()
        .filter_map(|&start| {
            let Point(x, y) = start;
            let score: isize = iproduct!(0..3, 0..3)
                .map(|(dx, dy)| Point(x + dx, y + dy))
                .map(|point| grid.get(&point).cloned())
                .collect::<Option<Vec<isize>>>()?
                .iter()
                .sum();
            Some((start, score))
        })
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("part 1: {},{}", x, y);
}

fn part2(grid: &HashMap<Point, isize>) {
    let cache = iproduct!(1..=300, 1..=300).map(|(x, y)| Point(x, y)).fold(
        HashMap::new(),
        |mut acc, point| {
            let Point(x, y) = point;
            let prev = acc.get(&Point(x, y - 1)).cloned().unwrap_or_default();
            let row: isize = (1..=x).map(|i| grid[&Point(i, y)]).sum();

            let score: isize = prev + row;

            acc.insert(point, score);
            acc
        },
    );

    let (Point(x, y), offset) = iproduct!(1..=300, 1..=300)
        .flat_map(|(x, y)| {
            let end = 300 - x.max(y);
            (0..=end).map(move |i| (Point(x, y), i))
        })
        .max_by_key(|(Point(x, y), i)| {
            let target = cache[&Point(x + i, y + i)];
            let top = cache.get(&Point(x + i, y - 1)).cloned().unwrap_or_default();
            let side = cache.get(&Point(x - 1, y + i)).cloned().unwrap_or_default();
            let overage = cache.get(&Point(x - 1, y - 1)).cloned().unwrap_or_default();

            target - top - side + overage
        })
        .unwrap();

    println!("part 2: {},{},{}", x, y, offset + 1);
}

fn main() {
    let grid = iproduct!(1..=300, 1..=300)
        .map(|(x, y)| {
            let point = Point(x, y);
            let power_level = power_level(point);
            (point, power_level)
        })
        .collect();

    part1(&grid);
    part2(&grid);
}
