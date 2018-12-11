extern crate itertools;

use itertools::iproduct;
use std::collections::HashMap;

const INPUT: usize = 5535;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

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

fn create_grid() -> HashMap<Point, isize> {
    iproduct!(1..=300, 1..=300)
        .map(|(x, y)| {
            let point = Point(x, y);
            let power_level = power_level(point);
            (point, power_level)
        })
        .collect()
}

fn main() {
    let grid = create_grid();
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
