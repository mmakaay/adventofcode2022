use std::io::{read_to_string, stdin};
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u8>;
type AssignmentPair = (Assignment, Assignment);

trait Overlap {
    fn overlaps_with(&self, other: &Self) -> bool;
    fn 
}

impl Overlap for Assignment {
    fn overlaps_with(&self, other: &Assignment) -> bool {
        self.contains(&other.end())
        || self.contains(&other.start())
        || other.contains(&self.end())
        || other.contains(&self.start())        
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
    let pairs_with_overlap: Vec<&AssignmentPair> = pairs
        .iter()
        .filter(|(a, b)| a.overlaps_with(&b))
        .collect();

    let answer = pairs_with_overlap.len();
    println!("{answer}");
}
