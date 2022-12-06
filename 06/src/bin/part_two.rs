use std::io::{read_to_string, stdin};
use aoc2022_06::find_start_pos_after_preamble;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    if let Some(pos) = find_start_pos_after_preamble(14, &input) {
        println!("{pos}");
    }
}
