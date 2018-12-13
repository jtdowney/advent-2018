use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum Track {
    Straight,
    CurveLeft,
    CurveRight,
    Intersection,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Up, Turn::Straight) => self,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Down, Turn::Straight) => self,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Left, Turn::Straight) => self,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Right, Turn::Straight) => self,
        }
    }
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
    Straight,
}

impl Turn {
    fn next(self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn next(self, cart: Cart) -> Point {
        let Point(x, y) = self;
        match cart {
            Cart {
                direction: Direction::Up,
                ..
            } => Point(x, y - 1),
            Cart {
                direction: Direction::Down,
                ..
            } => Point(x, y + 1),
            Cart {
                direction: Direction::Left,
                ..
            } => Point(x - 1, y),
            Cart {
                direction: Direction::Right,
                ..
            } => Point(x + 1, y),
        }
    }
}

#[derive(Copy, Clone)]
struct Cart {
    position: Point,
    direction: Direction,
    next_turn: Turn,
    crashed: bool,
}

impl Cart {
    fn next(self, track: Track) -> Cart {
        match track {
            Track::Straight => self,
            Track::CurveLeft => {
                let Cart { direction, .. } = self;
                match direction {
                    Direction::Up => Cart {
                        direction: Direction::Left,
                        ..self
                    },
                    Direction::Down => Cart {
                        direction: Direction::Right,
                        ..self
                    },
                    Direction::Right => Cart {
                        direction: Direction::Down,
                        ..self
                    },
                    Direction::Left => Cart {
                        direction: Direction::Up,
                        ..self
                    },
                }
            }
            Track::CurveRight => {
                let Cart { direction, .. } = self;
                match direction {
                    Direction::Up => Cart {
                        direction: Direction::Right,
                        ..self
                    },
                    Direction::Down => Cart {
                        direction: Direction::Left,
                        ..self
                    },
                    Direction::Right => Cart {
                        direction: Direction::Up,
                        ..self
                    },
                    Direction::Left => Cart {
                        direction: Direction::Down,
                        ..self
                    },
                }
            }
            Track::Intersection => {
                let Cart {
                    direction,
                    next_turn,
                    ..
                } = self;
                Cart {
                    direction: direction.turn(next_turn),
                    next_turn: next_turn.next(),
                    ..self
                }
            }
        }
    }
}

fn part1(grid: &HashMap<Point, Track>, carts: &[Cart]) {
    let mut carts = carts.to_vec();

    loop {
        carts.sort_by_key(|&Cart { position, .. }| {
            let Point(x, y) = position;
            (y, x)
        });

        for i in 0..carts.len() {
            let cart = carts[i];
            let track = grid[&cart.position];
            let next_cart = cart.next(track);
            let next_point = cart.position.next(next_cart);
            let crashed = carts
                .iter()
                .any(|&Cart { position, .. }| next_point == position);

            if crashed {
                let Point(x, y) = next_point;
                println!("part 1: {},{}", x, y);
                return;
            } else {
                carts[i] = next_cart;
                carts[i].position = next_point;
            }
        }
    }
}

fn part2(grid: &HashMap<Point, Track>, carts: &[Cart]) {
    let mut carts = carts.to_vec();

    loop {
        carts.sort_by_key(|&Cart { position, .. }| {
            let Point(x, y) = position;
            (y, x)
        });

        for i in 0..carts.len() {
            let cart = carts[i];
            let track = grid[&cart.position];
            let next_cart = cart.next(track);
            let next_point = cart.position.next(next_cart);
            let crashed = carts
                .iter()
                .position(|&Cart { position, .. }| next_point == position);

            if let Some(other) = crashed {
                carts[i].crashed = true;
                carts[other].crashed = true;
            } else {
                carts[i] = next_cart;
                carts[i].position = next_point;
            }
        }

        carts.retain(|Cart { crashed, .. }| !crashed);

        if carts.len() == 1 {
            let Cart { position, .. } = carts.first().unwrap();
            let Point(x, y) = position;
            println!("part 2: {},{}", x, y);
            break;
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let (grid, carts) = fs::read_to_string(filename)
        .expect("File to read")
        .lines()
        .enumerate()
        .fold(
            (HashMap::new(), vec![]),
            |(mut grid, mut carts), (y, line)| {
                line.chars().enumerate().for_each(|(x, ch)| {
                    let point = Point(x, y);
                    match ch {
                        '|' | '-' => {
                            grid.insert(point, Track::Straight);
                        }
                        '/' => {
                            grid.insert(point, Track::CurveRight);
                        }
                        '\\' => {
                            grid.insert(point, Track::CurveLeft);
                        }
                        '+' => {
                            grid.insert(point, Track::Intersection);
                        }
                        '^' => {
                            grid.insert(point, Track::Straight);
                            carts.push(Cart {
                                position: point,
                                direction: Direction::Up,
                                next_turn: Turn::Left,
                                crashed: false,
                            });
                        }
                        '>' => {
                            grid.insert(point, Track::Straight);
                            carts.push(Cart {
                                position: point,
                                direction: Direction::Right,
                                next_turn: Turn::Left,
                                crashed: false,
                            });
                        }
                        'v' => {
                            grid.insert(point, Track::Straight);
                            carts.push(Cart {
                                position: point,
                                direction: Direction::Down,
                                next_turn: Turn::Left,
                                crashed: false,
                            });
                        }
                        '<' => {
                            grid.insert(point, Track::Straight);
                            carts.push(Cart {
                                position: point,
                                direction: Direction::Left,
                                next_turn: Turn::Left,
                                crashed: false,
                            });
                        }
                        ' ' => {}
                        _ => unimplemented!(),
                    };
                });
                (grid, carts)
            },
        );

    part1(&grid, &carts);
    part2(&grid, &carts);
}
