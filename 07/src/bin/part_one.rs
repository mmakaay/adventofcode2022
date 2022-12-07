use std::io::{stdin, read_to_string};
use std::time::Instant;
use aoc2022_07::CumulativeDirSizer;

fn main() {
    let now = Instant::now();
    let input = read_to_string(stdin()).unwrap();
    let mut sizer = CumulativeDirSizer::new();
    input.trim().split("\n").for_each(|line| sizer.process(line));

    let answer: usize = sizer.get_sizes().iter().filter(|&size| size <= &100000).sum();

    println!("time: {:?}", now.elapsed());
    println!("answer: {answer}");
}
