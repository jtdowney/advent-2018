const INPUT: usize = 110201;

fn part1() {
    let mut scores = vec!['3', '7'];
    let mut workers = (0, 1);

    loop {
        let (left, right) = workers;
        let left_score = scores[left].to_digit(10).unwrap() as usize;
        let right_score = scores[right].to_digit(10).unwrap() as usize;
        let score = left_score + right_score;
        scores.extend(score.to_string().chars());

        let left = (left + left_score + 1) % scores.len();
        let right = (right + right_score + 1) % scores.len();

        workers = (left, right);

        if scores.len() >= INPUT + 10 {
            break;
        }
    }

    let answer = scores.iter().skip(INPUT).take(10).collect::<String>();
    println!("part 1: {}", answer);
}

fn part2() {
    let mut scores = vec!['3', '7'];
    let mut workers = (0, 1);
    let input_str = INPUT.to_string();

    loop {
        let (left, right) = workers;
        let left_score = scores[left].to_digit(10).unwrap() as usize;
        let right_score = scores[right].to_digit(10).unwrap() as usize;
        let score = left_score + right_score;
        scores.extend(score.to_string().chars());

        let left = (left + left_score + 1) % scores.len();
        let right = (right + right_score + 1) % scores.len();

        workers = (left, right);

        let tail = scores
            .iter()
            .rev()
            .take(input_str.len() + 2)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        if let Some(i) = tail.find(&input_str) {
            let answer = scores.len() - tail.len() + i;
            println!("part 2: {}", answer);
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
