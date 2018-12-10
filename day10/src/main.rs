extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
    static ref PARTICLE_REGEX: Regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .expect("Compiled regex");
}

struct Particle {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let captures = PARTICLE_REGEX
            .captures(source)
            .expect("Input to match regex");
        let px = captures
            .get(1)
            .map(|c| c.as_str())
            .unwrap_or_default()
            .parse()?;
        let py = captures
            .get(2)
            .map(|c| c.as_str())
            .unwrap_or_default()
            .parse()?;
        let vx = captures
            .get(3)
            .map(|c| c.as_str())
            .unwrap_or_default()
            .parse()?;
        let vy = captures
            .get(4)
            .map(|c| c.as_str())
            .unwrap_or_default()
            .parse()?;

        let position = (px, py);
        let velocity = (vx, vy);
        Ok(Particle { position, velocity })
    }
}

fn bounds(particles: &[Particle]) -> (isize, isize, isize, isize) {
    particles.iter().fold(
        (
            isize::max_value(),
            isize::max_value(),
            isize::min_value(),
            isize::min_value(),
        ),
        |(sx, sy, ex, ey), particle| {
            let (x, y) = particle.position;
            (x.min(sx), y.min(sy), x.max(ex), y.max(ey))
        },
    )
}

fn draw(particles: &[Particle]) {
    let (startx, starty, endx, endy) = bounds(particles);
    let grid = particles
        .iter()
        .map(|p| p.position)
        .collect::<HashSet<(isize, isize)>>();

    for y in starty..=endy {
        for x in startx..=endx {
            match grid.get(&(x, y)) {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }

        println!();
    }
}

fn tick(particles: &[Particle]) -> Vec<Particle> {
    particles
        .iter()
        .map(|particle| {
            let (px, py) = particle.position;
            let (vx, vy) = particle.velocity;

            let position = (px + vx, py + vy);
            Particle {
                position,
                velocity: particle.velocity,
            }
        })
        .collect()
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let mut input = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Particle>, _>>()
        .expect("Valid input");

    let (startx, starty, endx, endy) = bounds(&input);
    let mut width = endx - startx;
    let mut height = endy - starty;
    for t in 0.. {
        let next = tick(&input);

        let (startx, starty, endx, endy) = bounds(&next);
        let next_width = endx - startx;
        let next_height = endy - starty;

        if next_width > width || next_height > height {
            println!("part 1:");
            draw(&input);

            println!("part 2: {}", t);

            break;
        } else {
            input = next;
            width = next_width;
            height = next_height;
        }
    }
}
