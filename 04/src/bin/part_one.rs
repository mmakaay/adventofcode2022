use std::io::{read_to_string, stdin};
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u8>;
type AssignmentPair = (Assignment, Assignment);

trait FullOverlap {
    fn fully_overlaps(&self, other: &Self) -> bool;
}

impl FullOverlap for Assignment {
    fn fully_overlaps(&self, other: &Assignment) -> bool {
        self.contains(&other.end()) && self.contains(&other.start())   
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let pairs: Vec<AssignmentPair> = lines
        .iter()
        .map(|&line| {
            let p: Vec<u8> = line
                .split([',', '-'])
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            (p[0]..=p[1], p[2]..=p[3])
        })
        .collect();
    let pairs_with_full_overlap: Vec<&AssignmentPair> = pairs
        .iter()
        .filter(|(a, b)| a.fully_overlaps(&b) || b.fully_overlaps(&a))
        .collect();

    let answer = pairs_with_full_overlap.len();
    println!("{answer}");
}
