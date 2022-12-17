use std::io::read_to_string;
use std::io::stdin;
use aoc2022_08::Forest;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let forest = Forest::from_input(&input);
    let answer = forest.get_highest_scenic_score();
    println!("{answer:?}");
}
