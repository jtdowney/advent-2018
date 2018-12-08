use std::env;
use std::fs;
use std::str;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        let mine: usize = self.metadata.iter().cloned().sum();
        let children: usize = self.children.iter().map(|c| c.metadata_sum()).sum();
        mine + children
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| self.children.get(i - 1).map(|c| c.value()))
                .sum()
        }
    }
}

struct Parser<I: Iterator<Item = usize>> {
    inner: I,
}

impl<I: Iterator<Item = usize>> Parser<I> {
    fn new<T>(iter: T) -> Parser<I>
    where
        T: IntoIterator<Item = usize, IntoIter = I>,
    {
        Parser {
            inner: iter.into_iter(),
        }
    }

    fn parse_node(&mut self) -> Option<Node> {
        let children_count = self.inner.next()?;
        let metadata_count = self.inner.next()?;
        let children = (0..children_count)
            .map(|_| self.parse_node())
            .collect::<Option<Vec<Node>>>()?;
        let metadata = (0..metadata_count)
            .map(|_| self.inner.next())
            .collect::<Option<Vec<usize>>>()?;

        Some(Node { children, metadata })
    }
}

fn part1(root: &Node) {
    let answer = root.metadata_sum();
    println!("part 1: {}", answer);
}

fn part2(root: &Node) {
    let answer = root.value();
    println!("part 2: {}", answer);
}

fn main() {
    let filename = env::args().nth(1).expect("No file provided");
    let input = fs::read_to_string(filename)
        .expect("File to read")
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .expect("Input to parse");

    let mut parser = Parser::new(input);
    let root = parser.parse_node().expect("No root node");

    part1(&root);
    part2(&root);
}
