use std::collections::VecDeque;

const INPUT: usize = 110_201;

struct Scoreboard {
    position: usize,
    scores: Vec<char>,
    workers: (usize, usize),
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            position: 0,
            scores: vec!['3', '7'],
            workers: (0, 1),
        }
    }
}

impl Iterator for Scoreboard {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.scores.len() {
            let (left, right) = self.workers;
            let left_score = self.scores[left].to_digit(10).unwrap() as usize;
            let right_score = self.scores[right].to_digit(10).unwrap() as usize;
            let score = left_score + right_score;
            self.scores.extend(score.to_string().chars());

            let left = (left + left_score + 1) % self.scores.len();
            let right = (right + right_score + 1) % self.scores.len();

            self.workers = (left, right);
        }

        let item = self.scores.get(self.position).cloned();
        self.position += 1;

        item
    }
}

fn part1() {
    let scoreboard = Scoreboard::default();
    let answer = scoreboard.skip(INPUT).take(10).collect::<String>();
    println!("part 1: {}", answer);
}

fn part2() {
    let scoreboard = Scoreboard::default();
    let input_chars = INPUT.to_string().chars().collect::<Vec<char>>();
    let (answer, _) = scoreboard
        .enumerate()
        .try_fold((0, VecDeque::new()), |(_, mut acc), (i, score)| {
            acc.push_back(score);
            if acc.len() > input_chars.len() {
                let _ = acc.pop_front();
            }

            if acc == input_chars {
                Err((i - input_chars.len() + 1, acc))
            } else {
                Ok((i, acc))
            }
        })
        .unwrap_err();

    println!("part 2: {}", answer);
}

fn main() {
    part1();
    part2();
}
