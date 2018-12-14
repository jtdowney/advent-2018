const INPUT: usize = 110_201;

struct Scoreboard {
    position: usize,
    scores: Vec<char>,
    workers: (usize, usize),
}

impl Scoreboard {
    fn new(scores: &[char]) -> Self {
        Scoreboard {
            position: 0,
            scores: scores.to_vec(),
            workers: (0, 1),
        }
    }
}

impl Iterator for Scoreboard {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&score) = self.scores.get(self.position) {
            self.position += 1;
            return Some(score);
        }

        let (left, right) = self.workers;
        let left_score = self.scores[left].to_digit(10).unwrap() as usize;
        let right_score = self.scores[right].to_digit(10).unwrap() as usize;
        let score = left_score + right_score;
        self.scores.extend(score.to_string().chars());

        let left = (left + left_score + 1) % self.scores.len();
        let right = (right + right_score + 1) % self.scores.len();

        self.workers = (left, right);

        self.next()
    }
}

fn part1() {
    let scoreboard = Scoreboard::new(&['3', '7']);
    let answer = scoreboard.skip(INPUT).take(10).collect::<String>();
    println!("part 1: {}", answer);
}

fn part2() {
    let mut scoreboard = Scoreboard::new(&['3', '7']);
    let input_str = INPUT.to_string();

    loop {
        let _ = scoreboard.next();
        let tail = scoreboard
            .scores
            .iter()
            .rev()
            .take(input_str.len() + 2)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        if let Some(i) = tail.find(&input_str) {
            let answer = scoreboard.scores.len() - tail.len() + i;
            println!("part 2: {}", answer);
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
