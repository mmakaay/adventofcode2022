use std::io;

type Assignment = (u8, u8);
type AssignmentPair = (Assignment, Assignment);

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let pairs: Vec<AssignmentPair> = lines
        .iter()
        .map(|&line| {
            let p: Vec<u8> = line
                .split([',', '-'])
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            ((p[0], p[1]), (p[2], p[3]))
        })
        .collect();
    let pairs_with_full_overlap: Vec<&AssignmentPair> = pairs
        .iter()
        .filter(|(a, b)| {
            ((a.0..=a.1).contains(&b.0) && (a.0..=a.1).contains(&b.1)) ||
            ((b.0..=b.1).contains(&a.0) && (b.0..=b.1).contains(&a.1))
        })
        .collect();

    let answer = pairs_with_full_overlap.len();
    println!("{answer}");
}
